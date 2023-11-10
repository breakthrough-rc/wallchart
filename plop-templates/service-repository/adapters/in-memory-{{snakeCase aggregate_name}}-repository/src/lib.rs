use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use {{snakeCase service_name}}_service::models::{{pascalCase aggregate_name}};
use {{snakeCase service_name}}_service::ports::{{snakeCase aggregate_name}}_repository::{RepositoryFailure, {{pascalCase aggregate_name}}Repository};

#[derive(Clone, Debug)]
pub struct InMemory{{pascalCase aggregate_name}}Repository {
    pub {{snakeCase aggregate_name}}s: Arc<RwLock<Vec<{{pascalCase aggregate_name}}>>>,
}

impl InMemory{{pascalCase aggregate_name}}Repository {
    pub fn empty() -> Self {
        Self {
            {{snakeCase aggregate_name}}s: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn with({{snakeCase aggregate_name}}s: Vec<{{pascalCase aggregate_name}}>) -> Self {
        Self {
            {{snakeCase aggregate_name}}s: Arc::new(RwLock::new({{snakeCase aggregate_name}}s)),
        }
    }
}

#[async_trait]
impl {{pascalCase aggregate_name}}Repository for InMemory{{pascalCase aggregate_name}}Repository {
    async fn get_{{snakeCase aggregate_name}}(&self, id: String) -> Result<Option<{{pascalCase aggregate_name}}>, RepositoryFailure> {
        let {{snakeCase aggregate_name}}s = self.{{snakeCase aggregate_name}}s.read().await;
        Ok({{snakeCase aggregate_name}}s.iter().find(|w| w.id == id).map(|w| w.to_owned()))
    }

    async fn save(&self, id: String, {{snakeCase aggregate_name}}: {{pascalCase aggregate_name}}) -> Result<(), RepositoryFailure> {
        let mut {{snakeCase aggregate_name}}s = self.{{snakeCase aggregate_name}}s.write().await;

        {{snakeCase aggregate_name}}s.retain(|w| w.id != id);
        {{snakeCase aggregate_name}}s.push({{snakeCase aggregate_name}}.to_owned());

        Ok(())
    }
}
