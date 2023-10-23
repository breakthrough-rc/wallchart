use async_trait::async_trait;
use db::db::PgPool;
use db::schema::{
    activities, assessments, locations, shift_assignments, shifts, tags, workers, worksites,
};
use diesel::prelude::Associations;
use diesel::{
    prelude::{Identifiable, Queryable},
    Selectable,
};
use usecases::{
    models::Worksite,
    ports::worksite_repository::{RepositoryFailure, WorksiteRepository},
};

#[derive(Clone)]
pub struct DieselWorksiteRepository {
    pub pg_pool: PgPool,
}

#[async_trait]
impl WorksiteRepository for DieselWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        todo!()
    }
    async fn save(&self, id: String, worksite: &Worksite) -> Result<(), RepositoryFailure> {
        todo!()
    }
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug)]
#[diesel(table_name = worksites)]
pub struct WorksiteRecord {
    pub id: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(WorksiteRecord, foreign_key=worksite_id))]
#[diesel(table_name = locations)]
pub struct LocationRecord {
    pub id: String,
    pub name: String,
    pub worksite_id: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(LocationRecord, foreign_key=location_id))]
#[diesel(table_name = shifts)]
pub struct ShiftRecord {
    pub id: String,
    pub name: String,
    pub location_id: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = workers)]
pub struct WorkerRecord {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(WorkerRecord, foreign_key=worker_id))]
#[diesel(table_name = assessments)]
pub struct AssessmentRecord {
    pub id: String,
    pub value: String,
    pub worker_id: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = activities)]
pub struct ActivityRecord {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(ShiftRecord, foreign_key=shift_id))]
#[diesel(belongs_to(WorkerRecord, foreign_key=worker_id))]
#[diesel(table_name = shift_assignments)]
#[diesel(primary_key(shift_id, worker_id))]
pub struct ShiftAssignmentRecord {
    pub shift_id: String,
    pub worker_id: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(ActivityRecord, foreign_key=activity_id))]
#[diesel(belongs_to(WorkerRecord, foreign_key=worker_id))]
#[diesel(table_name = tags)]
#[diesel(primary_key(activity_id, worker_id))]
pub struct TagRecord {
    pub activity_id: String,
    pub worker_id: String,
}
