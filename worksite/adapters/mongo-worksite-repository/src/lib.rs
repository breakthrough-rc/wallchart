use async_trait::async_trait;
use chrono::{serde::ts_seconds, DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use worksite_service::{
    models::{
        Address, Assessment, AssignedTag, Location, Shift, ShiftWorker, Tag, Worker, Worksite,
    },
    ports::worksite_repository::{RepositoryFailure, WorksiteRepository},
};

#[derive(Debug, Serialize, Deserialize)]
struct WorksiteRecord {
    pub id: String,
    pub name: String,
    pub locations: Vec<LocationRecord>,
    pub tags: Vec<TagRecord>,
    pub workers: Vec<WorkerRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRecord {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub assessments: Vec<AssessmentRecord>,
    // Tag ids
    pub tags: Vec<String>,
    pub email: String,
    pub address: Option<AddressRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressRecord {
    pub street_address: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssessmentRecord {
    pub id: String,
    pub value: u8,
    pub notes: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated_at: DateTime<Utc>,
    pub assessor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagRecord {
    pub id: String,
    pub name: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocationRecord {
    pub id: String,
    pub name: String,
    pub shifts: Vec<ShiftRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShiftRecord {
    pub id: String,
    pub name: String,
    // Worker IDs
    pub workers: Vec<String>,
}

impl WorksiteRecord {
    pub fn to_worksite(&self) -> Worksite {
        Worksite {
            id: self.id.clone(),
            name: self.name.clone(),
            locations: self.locations.iter().map(|l| l.to_location()).collect(),
            tags: self.tags.iter().map(|t| t.to_tag()).collect(),
            workers: self.workers.iter().map(|w| w.to_worker()).collect(),
        }
    }
}

impl WorkerRecord {
    pub fn to_worker(&self) -> Worker {
        worksite_service::models::Worker {
            id: self.id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            assessments: self.assessments.iter().map(|a| a.to_assessment()).collect(),
            tags: self
                .tags
                .iter()
                .map(|t| AssignedTag::new(t.clone()))
                .collect(),
            email: self.email.clone(),
            address: self.address.as_ref().map(|a| a.to_address()),
        }
    }
}

impl AddressRecord {
    pub fn to_address(&self) -> Address {
        Address {
            street_address: self.street_address.clone(),
            city: self.city.clone(),
            region: self.region.clone(),
            postal_code: self.postal_code.clone(),
        }
    }
}

impl AssessmentRecord {
    pub fn to_assessment(&self) -> Assessment {
        worksite_service::models::Assessment {
            id: self.id.clone(),
            value: self.value,
            notes: self.notes.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            assessor: self.assessor.clone(),
        }
    }
}

impl TagRecord {
    pub fn to_tag(&self) -> Tag {
        Tag {
            id: self.id.clone(),
            name: self.name.clone(),
            icon: self.icon.clone(),
        }
    }
}

impl LocationRecord {
    pub fn to_location(&self) -> worksite_service::models::Location {
        worksite_service::models::Location {
            id: self.id.clone(),
            name: self.name.clone(),
            shifts: self.shifts.iter().map(|s| s.to_shift()).collect(),
        }
    }
}

impl ShiftRecord {
    pub fn to_shift(&self) -> Shift {
        Shift {
            id: self.id.clone(),
            name: self.name.clone(),
            workers: self
                .workers
                .iter()
                .map(|w| ShiftWorker::new(w.clone()))
                .collect(),
        }
    }
}

fn to_worksite_record(worksite: &Worksite) -> WorksiteRecord {
    WorksiteRecord {
        id: worksite.id.clone(),
        name: worksite.name.clone(),
        locations: worksite
            .locations
            .iter()
            .map(to_location_record)
            .collect(),
        tags: worksite.tags.iter().map(to_tag_record).collect(),
        workers: worksite
            .workers
            .iter()
            .map(to_worker_record)
            .collect(),
    }
}

fn to_worker_record(worker: &Worker) -> WorkerRecord {
    WorkerRecord {
        id: worker.id.clone(),
        first_name: worker.first_name.clone(),
        last_name: worker.last_name.clone(),
        assessments: worker
            .assessments
            .iter()
            .map(to_assessment_record)
            .collect(),
        tags: worker.tags.iter().map(|t| t.0.clone()).collect(),
        email: worker.email.clone(),
        address: worker.address.as_ref().map(to_address_record),
    }
}

fn to_address_record(address: &Address) -> AddressRecord {
    AddressRecord {
        street_address: address.street_address.clone(),
        city: address.city.clone(),
        region: address.region.clone(),
        postal_code: address.postal_code.clone(),
    }
}

fn to_assessment_record(assessment: &Assessment) -> AssessmentRecord {
    AssessmentRecord {
        id: assessment.id.clone(),
        value: assessment.value,
        notes: assessment.notes.clone(),
        created_at: assessment.created_at,
        updated_at: assessment.updated_at,
        assessor: assessment.assessor.clone(),
    }
}

fn to_tag_record(tag: &Tag) -> TagRecord {
    TagRecord {
        id: tag.id.clone(),
        name: tag.name.clone(),
        icon: tag.icon.clone(),
    }
}

fn to_location_record(location: &Location) -> LocationRecord {
    LocationRecord {
        id: location.id.clone(),
        name: location.name.clone(),
        shifts: location.shifts.iter().map(to_shift_record).collect(),
    }
}

fn to_shift_record(shift: &Shift) -> ShiftRecord {
    ShiftRecord {
        id: shift.id.clone(),
        name: shift.name.clone(),
        workers: shift.workers.iter().map(|w| w.0.clone()).collect(),
    }
}

#[derive(Clone, Debug)]
pub struct MongoWorksiteRepository {
    collection: mongodb::Collection<WorksiteRecord>,
}

impl MongoWorksiteRepository {
    pub fn from_client(client: &mongodb::Client) -> Result<Self, mongodb::error::Error> {
        let db = client.database("worksite");
        let collection = db.collection::<WorksiteRecord>("worksites");
        Ok(Self { collection })
    }
    pub async fn new(url: &String) -> Result<Self, mongodb::error::Error> {
        Ok(Self {
            collection: mongodb::Client::with_uri_str(url)
                .await?
                .database("worksite")
                .collection::<WorksiteRecord>("worksites"),
        })
    }
}

#[async_trait]
impl WorksiteRepository for MongoWorksiteRepository {
    async fn get_worksite(&self, id: String) -> Result<Option<Worksite>, RepositoryFailure> {
        let filter = doc! { "id": id };
        let maybe_worksite = self
            .collection
            .find_one(filter, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(maybe_worksite.map(|w| w.to_worksite()))
    }
    async fn get_all(&self) -> Result<Vec<Worksite>, RepositoryFailure> {
        let cursor = self
            .collection
            // Get all of the users
            .find(None, None)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        let users: Vec<WorksiteRecord> = cursor
            .try_collect()
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;

        Ok(users.iter().map(|w| w.to_worksite()).collect())
    }

    async fn save(&self, worksite: Worksite) -> Result<(), RepositoryFailure> {
        let filter = doc! {"id": worksite.id.clone()};
        let record = to_worksite_record(&worksite);
        let options = mongodb::options::ReplaceOptions::builder()
            .upsert(true)
            .build();
        self.collection
            .replace_one(filter, record, options)
            .await
            .map_err(|e| RepositoryFailure::Unknown(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    
    use mongo_testcontainer::Mongo;
    use mongodb::Client;
    use pretty_assertions::assert_eq;
    use testcontainers::clients;
    use worksite_service::{
        models::{
            Address, Assessment, AssignedTag, Location, Shift, ShiftWorker, Tag, Worker, Worksite,
        },
        ports::worksite_repository::WorksiteRepository,
    };

    use crate::MongoWorksiteRepository;

    fn make_worksite() -> Worksite {
        Worksite {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Name".into(),
            locations: vec![
                Location {
                    id: "1".into(),
                    name: "Office".into(),
                    shifts: vec![Shift {
                        id: "1".into(),
                        name: "Day".into(),
                        workers: vec![
                            ShiftWorker::new("1".into()),
                            ShiftWorker::new("2".into()),
                            ShiftWorker::new("3".into()),
                        ],
                    }],
                },
                Location {
                    id: "2".into(),
                    name: "Warehouse".into(),
                    shifts: vec![
                        Shift {
                            id: "2".into(),
                            name: "Day".into(),
                            workers: vec![
                                ShiftWorker::new("4".into()),
                                ShiftWorker::new("5".into()),
                            ],
                        },
                        Shift {
                            id: "3".into(),
                            name: "Night".into(),
                            workers: vec![
                                ShiftWorker::new("6".into()),
                                ShiftWorker::new("7".into()),
                            ],
                        },
                    ],
                },
            ],
            tags: vec![
                Tag {
                    id: "1".into(),
                    name: "Baked a cake".into(),
                    icon: "🍰".into(),
                },
                Tag {
                    id: "2".into(),
                    name: "Shared fries".into(),
                    icon: "🍟".into(),
                },
                Tag {
                    id: "3".into(),
                    name: "Listened to Rancid".into(),
                    icon: "🎸".into(),
                },
            ],
            workers: vec![
                Worker {
                    id: "1".into(),
                    first_name: "Jim".into(),
                    last_name: "Halpert".into(),
                    email: "jim.halpert@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "1".into(),
                        value: 1,
                        notes: "".into(),
                        assessor: "Victoria Hall".into(),
                        created_at: Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![
                        AssignedTag::new("1".into()),
                        AssignedTag::new("2".into()),
                        AssignedTag::new("3".into()),
                    ],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "2".into(),
                    first_name: "Pam".into(),
                    last_name: "Beesly".into(),
                    email: "pam.beesly@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "2".into(),
                        value: 2,
                        notes: "".into(),
                        assessor: "Victoria Hall".into(),
                        created_at: Utc.with_ymd_and_hms(2022, 10, 12, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2022, 10, 12, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![
                        AssignedTag::new("1".into()),
                        AssignedTag::new("2".into()),
                        AssignedTag::new("3".into()),
                    ],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "3".into(),
                    first_name: "Dwight".into(),
                    last_name: "Schrute".into(),
                    email: "dwight.schrute@skynet.org".into(),
                    assessments: vec![
                        Assessment {
                            id: "3".into(),
                            value: 4,
                            notes: "".into(),
                            assessor: "Victoria Hall".into(),
                            created_at: Utc.with_ymd_and_hms(2023, 3, 24, 0, 0, 0).unwrap(),
                            updated_at: Utc.with_ymd_and_hms(2023, 3, 24, 0, 0, 0).unwrap(),
                        },
                        Assessment {
                            id: "33".into(),
                            value: 5,
                            notes: "Wow, what a worker!".into(),
                            assessor: "Victoria Hall".into(),
                            created_at: Utc.with_ymd_and_hms(2022, 5, 4, 0, 0, 0).unwrap(),
                            updated_at: Utc.with_ymd_and_hms(2022, 6, 2, 0, 0, 0).unwrap(),
                        },
                    ],
                    tags: vec![AssignedTag::new("3".into())],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "4".into(),
                    first_name: "Darryl".into(),
                    last_name: "Philbin".into(),
                    email: "darryl.philbin@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "4".into(),
                        value: 1,
                        notes: "".into(),
                        assessor: "Raymond Sears".into(),
                        created_at: Utc.with_ymd_and_hms(2023, 7, 4, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2023, 7, 4, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![AssignedTag::new("2".into()), AssignedTag::new("3".into())],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "5".into(),
                    first_name: "Nate".into(),
                    last_name: "Nickerson".into(),
                    email: "nate.nickerson@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "5".into(),
                        value: 3,
                        notes: "".into(),
                        assessor: "Victoria Hall".into(),
                        created_at: Utc.with_ymd_and_hms(2023, 2, 6, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2023, 2, 6, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![AssignedTag::new("1".into())],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "6".into(),
                    first_name: "Roy".into(),
                    last_name: "Anderson".into(),
                    email: "roy.anderson@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "3".into(),
                        value: 3,
                        notes: "".into(),
                        assessor: "Victoria Hall".into(),
                        created_at: Utc.with_ymd_and_hms(2023, 4, 9, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2023, 4, 9, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![AssignedTag::new("2".into()), AssignedTag::new("3".into())],
                    address: Some(Address::default()),
                },
                Worker {
                    id: "7".into(),
                    first_name: "Val".into(),
                    last_name: "Johnson".into(),
                    email: "val.johnson@skynet.org".into(),
                    assessments: vec![Assessment {
                        id: "7".into(),
                        value: 2,
                        notes: "".into(),
                        assessor: "Victoria Hall".into(),
                        created_at: Utc.with_ymd_and_hms(2023, 10, 18, 0, 0, 0).unwrap(),
                        updated_at: Utc.with_ymd_and_hms(2023, 10, 18, 0, 0, 0).unwrap(),
                    }],
                    tags: vec![
                        AssignedTag::new("1".into()),
                        AssignedTag::new("2".into()),
                        AssignedTag::new("3".into()),
                    ],
                    address: Some(Address::default()),
                },
            ],
        }
    }

    #[tokio::test]
    async fn tests() {
        let docker_cli = clients::Cli::default();
        let container = docker_cli.run(Mongo);
        let host_port = container.get_host_port_ipv4(27017);
        let url = format!("mongodb://127.0.0.1:{host_port}/");
        let mongo_client: Client = Client::with_uri_str(&url).await.unwrap();
        let repo: MongoWorksiteRepository =
            MongoWorksiteRepository::from_client(&mongo_client).unwrap();

        test_create_and_fetch(&repo).await;
        test_get_worksites(&repo).await;
        test_update_worksite(&repo).await;
    }

    async fn test_create_and_fetch(repo: &MongoWorksiteRepository) {
        let worksite = make_worksite();

        repo.save(worksite.clone()).await.unwrap();

        let result = repo.get_worksite(worksite.id.clone()).await.unwrap();
        assert_eq!(result, Some(worksite));
    }

    async fn test_update_worksite(repo: &MongoWorksiteRepository) {
        let worksite = make_worksite();

        repo.save(worksite.clone()).await.unwrap();

        let result = repo.get_worksite(worksite.id.clone()).await.unwrap();
        assert_eq!(result, Some(worksite.clone()));

        let worksite = Worksite {
            name: "New Name".into(),
            ..worksite.clone()
        };

        repo.save(worksite.clone()).await.unwrap();

        let result = repo.get_worksite(worksite.id.clone()).await.unwrap();
        assert_eq!(result, Some(worksite));
    }

    async fn test_get_worksites(repo: &MongoWorksiteRepository) {
        // Note that this is a get all, but we are using a single db instance
        // so our assertions here are going to be weaker so the test is less flaky
        let worksite1 = make_worksite();
        let worksite2 = make_worksite();

        repo.save(worksite1.clone()).await.unwrap();
        repo.save(worksite2.clone()).await.unwrap();

        let result = repo.get_all().await.unwrap();
        assert!(result.contains(&worksite1));
        assert!(result.contains(&worksite2));
    }
}
