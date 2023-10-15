use axum::response::Html;
use rscx::html;

use crate::{
    components::wallchart::{Assessment, Location, Shift, Tag, Wallchart, Worker, Worksite},
    page::PageLayout,
};

pub async fn get_wallchart_page() -> Html<String> {
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

    Html(html! {
    <PageLayout>
        <div class="my-4">
            <Wallchart worksite=worksite/>
        </div>
    </PageLayout>
    })
}
