use std::env;

use dotenvy::dotenv;
use tracing::instrument;

/**
* A single struct that represents all of the env vars.
* This struct should be created once during bootstrapping and then its values can be handed out as
* necessary
*
* As it grows larger, we may want to make it composed of smaller structs representing different
* groups of related env vars. E.g. DBEnvVars, GoogleApiCreds, etc.
*/
pub struct Environment {
    pub auth_mongo_db_url: String,
}

/**
* Function to do all the "dirty work" of pulling env vars into the Environment struct.
*/
#[instrument]
pub fn load_environment() -> Environment {
    dotenv().ok();
    Environment {
        auth_mongo_db_url: env::var("AUTH_MONGO_DB_URL").expect("AUTH_MONGO_DB_URL must be set"),
    }
}
