use std::{collections::HashMap, sync::Arc};

use serde::Deserialize;
use thiserror::Error;

use crate::{
    models::{
        Email, FirstName, LastName, Location, LocationName, Shift, ShiftName, Worker, Worksite,
        WorksiteName,
    },
    ports::worksite_repository::WorksiteRepository,
};

#[derive(Clone)]
pub struct CsvUpload {
    pub worksite_repository: Arc<dyn WorksiteRepository>,
}

#[derive(Clone, Debug)]
pub struct CsvUploadInput {
    /*
     * The stringified CSV content. We will attempt to deserialize this into
     *  worker records
     */
    pub csv_input: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct WorkerRecord {
    worksite: String,
    location: String,
    shift1: Option<String>,
    shift2: Option<String>,
    shift3: Option<String>,
    first_name: String,
    last_name: String,
    email: String,
}

// Change the return type, if needed
pub type CsvUploadOutput = Result<Vec<Worksite>, CsvUploadFailure>;

/**
* This is some gross code that maps the gross CSV formatted data into our domain and then saves
* it
*
* The code was very hairy to try to get to be "efficient" so I settled for "works" and hopefully
* "readable"
*
* Esentially we do multiple linear passes over the data to create the worksites, locations, shifts,
* workers, and shift assignments.
*
* We then combine them all and then save them in the DB. This should reduce round trips to the DB,
* however, it is still possible for failures between calls to the repo. Maybe we want a `save_all`
* method on the repo?
*/
impl CsvUpload {
    pub async fn csv_upload(&self, input: CsvUploadInput) -> CsvUploadOutput {
        let mut rdr = csv::Reader::from_reader(input.csv_input.as_bytes());
        let records: Result<Vec<WorkerRecord>, csv::Error> = rdr.deserialize().collect();
        let records = records.map_err(|e| CsvUploadFailure::ParseFailure(e.to_string()))?;
        let worksites: &mut HashMap<WorksiteName, Worksite> = &mut HashMap::new();
        let workers: &mut HashMap<(WorksiteName, FirstName, LastName, Email), Worker> =
            &mut HashMap::new();

        // TODO:
        // 1. Validate input and return helpful errors
        //

        // Create worksites with workers, but no locations or shifts yet.
        for record in records.clone().iter() {
            let worksite = worksites
                .get(&record.worksite)
                .cloned()
                .unwrap_or_else(|| Worksite::new(record.worksite.clone()));

            let worker = Worker::new(
                record.first_name.clone(),
                record.last_name.clone(),
                record.email.clone(),
            );

            let worksite = worksite.add_worker(worker.clone());

            worksites.insert(worksite.name.clone(), worksite);
            workers.insert(
                (
                    record.worksite.clone(),
                    record.first_name.clone(),
                    record.last_name.clone(),
                    record.email.clone(),
                ),
                worker.clone(),
            );
        }

        // Create all the locations
        let locations: &mut HashMap<(WorksiteName, LocationName), Location> = &mut HashMap::new();
        for record in records.clone().iter() {
            let location = Location::new(record.location.clone());
            locations.insert((record.worksite.clone(), record.location.clone()), location);
        }

        // Create all the shifts
        let shifts: &mut HashMap<(WorksiteName, LocationName, ShiftName), Shift> =
            &mut HashMap::new();
        for record in records.clone().iter() {
            // Just do puts on all of them until they are loaded up.
            // It's a tiny bit wasteful but its simple
            if let Some(shift1) = &record.shift1 {
                shifts.insert(
                    (
                        record.worksite.clone(),
                        record.location.clone(),
                        shift1.clone(),
                    ),
                    Shift::new(shift1.clone()),
                );
            }
            if let Some(shift2) = &record.shift1 {
                shifts.insert(
                    (
                        record.worksite.clone(),
                        record.location.clone(),
                        shift2.clone(),
                    ),
                    Shift::new(shift2.clone()),
                );
            }
            if let Some(shift3) = &record.shift1 {
                shifts.insert(
                    (
                        record.worksite.clone(),
                        record.location.clone(),
                        shift3.clone(),
                    ),
                    Shift::new(shift3.clone()),
                );
            }
        }

        // Add the shifts to the locations
        for ((worksite_name, location_name, _), shift) in shifts.iter_mut() {
            let location = locations
                .get(&(worksite_name.clone(), location_name.clone()))
                .unwrap();
            let location = location.add_shift(shift.clone());
            locations.insert(
                (worksite_name.clone(), location_name.clone()),
                location.clone(),
            );
        }

        // Add the locations to the worksites
        for ((worksite_name, _), location) in locations.iter_mut() {
            let worksite = worksites.get(worksite_name).unwrap();
            let worksite = worksite.add_location(location.clone());
            worksites.insert(worksite_name.clone(), worksite.clone());
        }

        // Add the shift assignments
        for record in records.clone().iter() {
            let worksite = worksites.get(&record.worksite.clone()).unwrap();
            let location = locations
                .get(&(record.worksite.clone(), record.location.clone()))
                .unwrap();
            let worker = workers
                .get(&(
                    record.worksite.clone(),
                    record.first_name.clone(),
                    record.last_name.clone(),
                    record.email.clone(),
                ))
                .unwrap();

            let worksite = match &record.shift1 {
                None => worksite.clone(),
                Some(shift1) => {
                    let shift = shifts.get(&(
                        record.worksite.clone(),
                        record.location.clone(),
                        shift1.clone(),
                    ));

                    shift
                        .map(|shift| {
                            worksite.assign_worker(
                                worker.id.clone(),
                                shift.id.clone(),
                                location.id.clone(),
                            )
                        })
                        .unwrap_or(worksite.clone())
                }
            };
            let worksite = match &record.shift2 {
                None => worksite.clone(),
                Some(shift2) => {
                    let shift = shifts.get(&(
                        record.worksite.clone(),
                        record.location.clone(),
                        shift2.clone(),
                    ));

                    shift
                        .map(|shift| {
                            worksite.assign_worker(
                                worker.id.clone(),
                                shift.id.clone(),
                                location.id.clone(),
                            )
                        })
                        .unwrap_or(worksite.clone())
                }
            };
            let worksite = match &record.shift3 {
                None => worksite.clone(),
                Some(shift3) => {
                    let shift = shifts.get(&(
                        record.worksite.clone(),
                        record.location.clone(),
                        shift3.clone(),
                    ));

                    shift
                        .map(|shift| {
                            worksite.assign_worker(
                                worker.id.clone(),
                                shift.id.clone(),
                                location.id.clone(),
                            )
                        })
                        .unwrap_or(worksite.clone())
                }
            };

            worksites.insert(worksite.name.clone(), worksite.clone());
        }

        // Actually save the worksites!
        for (_, worksite) in worksites.iter_mut() {
            self.worksite_repository
                .save(worksite.clone())
                .await
                .map_err(|e| CsvUploadFailure::Unknown(e.to_string()))?;
        }

        Ok(worksites
            .values()
            .map(|w| w.clone())
            .collect::<Vec<Worksite>>())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum CsvUploadFailure {
    #[error("Failed to parse csv")]
    ParseFailure(String),
    #[error("Something went wrong")]
    Unknown(String),
}
