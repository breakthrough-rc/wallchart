use async_trait::async_trait;
use chrono::{serde::ts_seconds, DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use worksite_service::{
    models::{Address, Assessment, AssignedTag, Shift, ShiftWorker, Tag, Worker, Worksite},
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
            address: self.address.as_ref().map(|a| a.to_address()).clone(),
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

fn to_worksite_record(_worksite: &Worksite) -> WorksiteRecord {
    todo!()
}

#[derive(Clone, Debug)]
pub struct MongoWorksiteRepository {
    collection: mongodb::Collection<WorksiteRecord>,
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
