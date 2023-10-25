use axum::{response::IntoResponse, routing::get, Router};
use in_memory_worksite_repository::InMemoryWorksiteRepository;
use std::{net::SocketAddr, sync::Arc};
use usecases::{
    get_worksite::GetWorksite,
    models::{Assessment, Location, Shift, Tag, Worker, Worksite},
    remove_worker_from_shift::RemoveWorkerFromShift,
    service::WorksiteService,
};
use web_htmx::{livereload, routes as web_routes, state::WebHtmxState};

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
                        Worker {
                            id: "1".into(),
                            name: "Jim Halpert".into(),
                            last_assessment: Assessment {
                                id: "1".into(),
                                value: 1,
                            },
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
                        },
                        Worker {
                            id: "2".into(),
                            name: "Pam Beesly".into(),
                            last_assessment: Assessment {
                                id: "2".into(),
                                value: 2,
                            },
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
                        },
                        Worker {
                            id: "3".into(),
                            name: "Dwight Schrute".into(),
                            last_assessment: Assessment {
                                id: "3".into(),
                                value: 4,
                            },
                            tags: vec![Tag {
                                id: "3".into(),
                                name: "Listened to Rancid".into(),
                                icon: "🎸".into(),
                            }],
                        },
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
                            Worker {
                                id: "4".into(),
                                name: "Darryl Philbin".into(),
                                last_assessment: Assessment {
                                    id: "4".into(),
                                    value: 1,
                                },
                                tags: vec![
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
                            },
                            Worker {
                                id: "5".into(),
                                name: "Nate Nickerson".into(),
                                last_assessment: Assessment {
                                    id: "5".into(),
                                    value: 3,
                                },
                                tags: vec![Tag {
                                    id: "1".into(),
                                    name: "Baked a cake".into(),
                                    icon: "🍰".into(),
                                }],
                            },
                        ],
                    },
                    Shift {
                        id: "3".into(),
                        name: "Night".into(),
                        workers: vec![
                            Worker {
                                id: "6".into(),
                                name: "Roy Anderson".into(),
                                last_assessment: Assessment {
                                    id: "3".into(),
                                    value: 3,
                                },
                                tags: vec![
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
                            },
                            Worker {
                                id: "7".into(),
                                name: "Val johnson".into(),
                                last_assessment: Assessment {
                                    id: "7".into(),
                                    value: 2,
                                },
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
                            },
                        ],
                    },
                ],
            },
        ],
    };
    let worksite_repository = Arc::new(InMemoryWorksiteRepository::with(vec![worksite]));
    let get_worksite = GetWorksite {
        worksite_repository: worksite_repository.clone(),
    };
    let remove_worker_from_shift = RemoveWorkerFromShift {
        worksite_repository: worksite_repository.clone(),
    };
    let worksite_service = WorksiteService {
        get_worksite,
        remove_worker_from_shift,
    };

    // Create WebHtmxState
    let web_htmx_state = WebHtmxState {
        worksite_service: Arc::new(worksite_service),
    };

    let app = Router::new()
        .merge(web_routes(web_htmx_state))
        .route("/healthcheck", get(get_health_check));

    #[cfg(debug_assertions)]
    let app = app.layer(livereload::layer());

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
