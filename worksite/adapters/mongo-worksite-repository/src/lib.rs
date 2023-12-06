use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use worksite_service::{
    models::Worksite,
    ports::worksite_repository::{RepositoryFailure, WorksiteRepository},
};

#[derive(Debug, Serialize, Deserialize)]
struct WorksiteRecord {
    pub id: String,
}

impl WorksiteRecord {
    pub fn to_worksite(&self) -> Worksite {
        todo!()
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
