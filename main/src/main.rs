use auth_service::{create_user::CreateUserInput, service::AuthService};
use axum::{response::IntoResponse, routing::get, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer,
};
use in_memory_user_repository::{InMemoryUserRepository, InMemoryUserStore};
use in_memory_worksite_repository::InMemoryWorksiteRepository;
use rand::Rng;
use std::{net::SocketAddr, sync::Arc};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};
use worksite_service::{
    models::{Assessment, AssignedTag, Location, Shift, ShiftWorker, Tag, Worker, Worksite},
    service::WorksiteService,
};

#[tokio::main]
async fn main() {
    // Create worksite service
    let worksite = Worksite {
        id: "1".into(),
        name: "Dunder Mifflin".into(),
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
                last_assessment: Some(Assessment {
                    id: "1".into(),
                    value: 1,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "1".into(),
                    value: 1,
                    notes: "".into(),
                }],
                tags: vec![
                    AssignedTag::new("1".into()),
                    AssignedTag::new("2".into()),
                    AssignedTag::new("3".into()),
                ],
            },
            Worker {
                id: "2".into(),
                first_name: "Pam".into(),
                last_name: "Beesly".into(),
                last_assessment: Some(Assessment {
                    id: "2".into(),
                    value: 2,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "2".into(),
                    value: 2,
                    notes: "".into(),
                }],
                tags: vec![
                    AssignedTag::new("1".into()),
                    AssignedTag::new("2".into()),
                    AssignedTag::new("3".into()),
                ],
            },
            Worker {
                id: "3".into(),
                first_name: "Dwight".into(),
                last_name: "Schrute".into(),
                last_assessment: Some(Assessment {
                    id: "3".into(),
                    value: 4,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "3".into(),
                    value: 4,
                    notes: "".into(),
                }],
                tags: vec![AssignedTag::new("3".into())],
            },
            Worker {
                id: "4".into(),
                first_name: "Darryl".into(),
                last_name: "Philbin".into(),
                last_assessment: Some(Assessment {
                    id: "4".into(),
                    value: 1,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "4".into(),
                    value: 1,
                    notes: "".into(),
                }],
                tags: vec![AssignedTag::new("2".into()), AssignedTag::new("3".into())],
            },
            Worker {
                id: "5".into(),
                first_name: "Nate".into(),
                last_name: "Nickerson".into(),
                last_assessment: Some(Assessment {
                    id: "5".into(),
                    value: 3,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "5".into(),
                    value: 3,
                    notes: "".into(),
                }],
                tags: vec![AssignedTag::new("1".into())],
            },
            Worker {
                id: "6".into(),
                first_name: "Roy".into(),
                last_name: "Anderson".into(),
                last_assessment: Some(Assessment {
                    id: "3".into(),
                    value: 3,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "3".into(),
                    value: 3,
                    notes: "".into(),
                }],
                tags: vec![AssignedTag::new("2".into()), AssignedTag::new("3".into())],
            },
            Worker {
                id: "7".into(),
                first_name: "Val".into(),
                last_name: "johnson".into(),
                last_assessment: Some(Assessment {
                    id: "7".into(),
                    value: 2,
                    notes: "".into(),
                }),
                assessments: vec![Assessment {
                    id: "7".into(),
                    value: 2,
                    notes: "".into(),
                }],
                tags: vec![
                    AssignedTag::new("1".into()),
                    AssignedTag::new("2".into()),
                    AssignedTag::new("3".into()),
                ],
            },
        ],
    };
    let worksite_repository = Arc::new(InMemoryWorksiteRepository::with(vec![worksite]));
    let worksite_service = WorksiteService::new(worksite_repository);

    let user_repository = Arc::new(InMemoryUserRepository::empty());
    let auth_service = AuthService::new(user_repository.clone());

    // Create a default user
    auth_service
        .create_user(CreateUserInput {
            email: "user@yallchart.com".into(),
            password: "password".into(),
        })
        .await
        .expect("Failed to create default user");

    // Create WebHtmxState
    let web_htmx_state = WebHtmxState {
        auth_service: Arc::new(auth_service),
        worksite_service: Arc::new(worksite_service),
        flash_config: axum_flash::Config::new(axum_flash::Key::generate()),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .route("/healthcheck", get(get_health_check));

    #[cfg(debug_assertions)]
    let app = app.layer(livereload::layer());

    // Session and Auth Management
    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);
    let user_memory_store = InMemoryUserStore {
        users: user_repository.clone(),
    };
    let auth_layer = AuthLayer::new(user_memory_store, &secret);

    let app = app.layer(auth_layer);
    let app = app.layer(session_layer);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn get_health_check() -> impl IntoResponse {
    "ONE SMALL STEP FOR AN ASSHOLE, ONE GIANT LEAP FOR ASSHOLEKIND"
}
