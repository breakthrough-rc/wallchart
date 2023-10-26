use async_trait::async_trait;
use db::db::PgPool;
use db::schema::{
    activities, assessments, locations, shift_assignments, shifts, tags, workers, worksites,
};
use diesel::prelude::*;
use diesel::OptionalExtension;
use diesel_async::scoped_futures::ScopedFutureExt;
use diesel_async::RunQueryDsl;
use worksite_service::models::Assessment;
use worksite_service::{
    models::Worksite,
    ports::worksite_repository::{RepositoryFailure, WorksiteRepository},
};

use crate::records::{
    LocationRecord, ShiftAssignmentRecord, ShiftRecord, WorkerRecord, WorksiteRecord,
};

#[derive(Clone)]
pub struct DieselWorksiteRepository {
    pub pg_pool: PgPool,
}

#[async_trait]
impl WorksiteRepository for DieselWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        let conn = &mut self
            .pg_pool
            .get()
            .await
            .map_err(|_| RepositoryFailure::FailedToGetConnectionFromPool)?;

        let maybe_worksite: Option<WorksiteRecord> = worksites::table
            .find(id)
            .first::<WorksiteRecord>(conn)
            .await
            .optional()
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let worksite = match maybe_worksite {
            Some(worksite) => worksite,
            None => return Ok(None),
        };

        let locations = LocationRecord::belonging_to(&worksite)
            .load::<LocationRecord>(conn)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let shifts = ShiftRecord::belonging_to(&locations)
            .load::<ShiftRecord>(conn)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        // Many to Many's handled according to these docs
        // https://diesel.rs/guides/relations.html#many-to-many-or-mn
        let workers: Vec<(ShiftAssignmentRecord, WorkerRecord)> =
            ShiftAssignmentRecord::belonging_to(&shifts)
                .inner_join(workers::table)
                .select((
                    ShiftAssignmentRecord::as_select(),
                    WorkerRecord::as_select(),
                ))
                .load(conn)
                .await
                .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let workers_per_shift: Vec<(ShiftRecord, Vec<WorkerRecord>)> = workers
            .grouped_by(&shifts)
            .into_iter()
            .zip(shifts)
            .map(|(workers, shift)| {
                (
                    shift,
                    workers.into_iter().map(|(_, worker)| worker).collect(),
                )
            })
            .collect();

        // Ignoring tags and assessments for now
        // Start grouping everything together before we map to the domain
        // https://docs.rs/diesel/latest/diesel/associations/index.html
        let grouped_locations: Vec<(LocationRecord, Vec<(ShiftRecord, Vec<WorkerRecord>)>)> =
            workers_per_shift
                .grouped_by(&locations)
                .into_iter()
                .zip(locations)
                .map(|(shifts, location)| (location, shifts))
                .collect();

        Ok(Some(to_worksite(worksite, grouped_locations)))
    }

    async fn save(&self, id: String, worksite: &Worksite) -> Result<(), RepositoryFailure> {
        // TODOs: Should this be a single transaction?
        let conn = &mut self
            .pg_pool
            .get()
            .await
            .map_err(|_| RepositoryFailure::FailedToGetConnectionFromPool)?;

        let worksite_record = WorksiteRecord {
            id: worksite.id.clone(),
            name: worksite.name.clone(),
        };

        // 1 - Upsert worksite record
        diesel::insert_into(worksites::table)
            .values(&worksite_record)
            .on_conflict(worksites::id)
            .do_update()
            .set(&worksite_record)
            .returning(WorksiteRecord::as_returning())
            .get_result(conn)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        
        // 2 - Upsert location records as a group
        
        // 3 - Remove any locations that are no longer in the worksite
        // 4 - Upsert shift records as a group
        // 5 - Remove any shifts that are no longer in the worksite (via a location)
        // 6 - Upsert worker records as a group
        // 7 - Upsert worker shift assignments
        // 8 - Remove any shift assignments not present in the worksite
        todo!()
    }
}

fn to_worksite(
    worksite: WorksiteRecord,
    locations: Vec<(LocationRecord, Vec<(ShiftRecord, Vec<WorkerRecord>)>)>,
) -> Worksite {
    let worksite = Worksite {
        id: worksite.id,
        name: worksite.name,
        locations: locations
            .into_iter()
            .map(|(location, shifts)| {
                let shifts = shifts
                    .into_iter()
                    .map(|(shift, workers)| {
                        let workers = workers
                            .into_iter()
                            .map(|worker| worksite_service::models::Worker {
                                id: worker.id,
                                name: worker.first_name + " " + &worker.last_name,
                                last_assessment: Assessment {
                                    id: "1".to_string(),
                                    value: 5,
                                },
                                tags: vec![],
                            })
                            .collect();

                        worksite_service::models::Shift {
                            id: shift.id,
                            name: shift.name,
                            workers,
                        }
                    })
                    .collect();

                worksite_service::models::Location {
                    id: location.id,
                    name: location.name,
                    shifts,
                }
            })
            .collect(),
    };
    worksite
}
