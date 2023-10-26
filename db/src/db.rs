use diesel::pg::{Pg, PgConnection};
use diesel::Connection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::Level;
use tracing::{event, instrument};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

/**
 * DB state
 */
#[instrument]
pub fn get_connection_pool(database_url: &String) -> Pool<AsyncPgConnection> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    Pool::builder(config)
        .build()
        .expect("Could not create postgres connection pool")
}

/*
* A bit non ideal but in order to run our db migrations we must use a sync connection.
* So we are doing that all in this function and then using the pg_pool for the rest of our code.
*/
pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) {
    event!(Level::INFO, "Running Migrations...");
    // Run the migrations!
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    event!(Level::INFO, "Finished running migrations!");
}

#[instrument]
pub fn get_sync_postgres_connection(database_url: &String) -> PgConnection {
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type PgPool = Pool<AsyncPgConnection>;
