use auth_service::{
    create_user::CreateUserInput, get_user_for_login::GetUserForLoginInput, service::AuthService,
};
use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Router,
};
use axum_login::{tower_sessions::SessionManagerLayer, AuthManagerLayerBuilder};
use chrono::prelude::*;
use environment::load_environment;
use mongo_user_repository::{MongoUserRepository, MongoUserStore};
use mongo_worksite_repository::MongoWorksiteRepository;
use std::{net::SocketAddr, sync::Arc};
use tower::ServiceBuilder;

use tower_sessions::{cookie::time::Duration, mongodb::Client, Expiry, MongoDBStore};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};
use worksite_service::{
    models::{
        Address, Assessment, AssignedTag, Location, Shift, ShiftWorker, Tag, Worker, Worksite,
    },
    ports::worksite_repository::WorksiteRepository,
    service::WorksiteService,
};

mod environment;

const DEFAULT_WORKSITE_ID: &str = "1";
const DEFAULT_WORKSITE_NAME: &str = "Dunder Miflin";

#[tokio::main]
async fn main() {
    let env = load_environment();

    // Create worksite service
    let worksite = Worksite {
        id: DEFAULT_WORKSITE_ID.into(),
        name: DEFAULT_WORKSITE_NAME.into(),
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
                        workers: vec![ShiftWorker::new("4".into()), ShiftWorker::new("5".into())],
                    },
                    Shift {
                        id: "3".into(),
                        name: "Night".into(),
                        workers: vec![ShiftWorker::new("6".into()), ShiftWorker::new("7".into())],
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
    };
    let worksite_repository = Arc::new(
        MongoWorksiteRepository::new(&env.auth_mongo_db_url)
            .await
            .expect("Could not create worksite repository"),
    );
    let worksite_service = WorksiteService::new(worksite_repository.clone());

    let existing_worksite = worksite_repository
        .get_worksite(DEFAULT_WORKSITE_ID.into())
        .await
        .expect("Couldn't fetch from worksite repo on load");

    if existing_worksite.is_some() {
        println!("Default worksite already exists");
    } else {
        println!("Creating default worksite");
        worksite_repository
            .save(worksite)
            .await
            .expect("Failed to create default worksite");
    }
    let user_repository = Arc::new(
        MongoUserRepository::new(&env.auth_mongo_db_url)
            .await
            .expect("Could not create user repository"),
    );
    let auth_service = AuthService::new(user_repository.clone());

    // Create a default user
    let existing_user = auth_service
        .get_user_for_login(GetUserForLoginInput {
            email: "user@yallchart.com".into(),
            password: "password".into(),
        })
        .await;
    if existing_user.is_ok() {
        println!("Default user already exists");
    } else {
        println!("Creating default user");
        auth_service
            .create_user(CreateUserInput {
                email: "user@yallchart.com".into(),
                password: "password".into(),
                role: "Organizer".into(),
            })
            .await
            .expect("Failed to create default user");

        auth_service
            .create_user(CreateUserInput {
                email: "superadminuser@yallchart.com".into(),
                password: "superpassword".into(),
                role: "SuperAdmin".into(),
            })
            .await
            .expect("Failed to create default super admin user");
    }

    // Create WebHtmxState
    let web_htmx_state = WebHtmxState {
        auth_service: Arc::new(auth_service),
        worksite_service: Arc::new(worksite_service),
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
        default_worksite_id: DEFAULT_WORKSITE_ID.into(),
        default_worksite_name: DEFAULT_WORKSITE_NAME.into(),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .route("/healthcheck", get(get_health_check));

    #[cfg(debug_assertions)]
    let app = app.layer(livereload::layer());

    // Session and Auth Management
    let client = Client::with_uri_str(&env.auth_mongo_db_url)
        .await
        .expect("Failed to create mongo client");
    let session_store = MongoDBStore::new(client, "sessions".to_string());
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(SessionManagerLayer::new(session_store.clone()).with_secure(false));

    let user_memory_store = MongoUserStore {
        users: user_repository.clone(),
    };
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));
    let auth_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(AuthManagerLayerBuilder::new(user_memory_store, session_layer).build());

    let app = app.layer(auth_layer);
    let app = app.layer(session_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_health_check() -> impl IntoResponse {
    "ONE SMALL STEP FOR AN ASSHOLE, ONE GIANT LEAP FOR ASSHOLEKIND"
}
