use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Form, Router,
};
use axum_flash::Flash;
use http::StatusCode;
use rscx::{component, html, props, CollectFragmentAsync};
use serde::Deserialize;

use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Label, SelectInput},
    modal::{Modal, ModalSize},
};
use worksite_service::{
    assign_worker::AssignWorkerInput, get_workers::GetWorkersInput, models::Worker,
    remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

use crate::{components::page::PageLayout, state::WebHtmxState};

pub fn shift_assignments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new",
            get(get_shift_assignment_form).post(post_shift_assignment),
        )
        .route(
            "/wallcharts/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new-modal",
            get(get_shift_assignment_form_modal),
        )
        .route(
            "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id",
            delete(delete_worker_from_shift),
        )
        .with_state(state)
}

async fn delete_worker_from_shift(
    extract::Path((worksite_id, location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    println!(
        "Delete worker: {} from shift: {}, from worksite: {} in location: {}",
        worker_id, shift_id, worksite_id, location_id,
    );

    let result = worksite_service
        .remove_worker_from_shift(RemoveWorkerFromShiftInput {
            id: worksite_id,
            shift_id,
            worker_id,
        })
        .await;

    match result {
        Ok(_) => "".into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error deleting worker from shift",
        )
            .into_response(),
    }
}

async fn get_shift_assignment_form_modal(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: wallchart_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <AssignShiftForm
                workers=workers
                action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", &wallchart_id, location_id, shift_id)
                create_worker_action=format!("/wallcharts/{}/workers/new", wallchart_id)
            />
        </Modal>
    })
}

async fn get_shift_assignment_form(
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    State(state): State<WebHtmxState>,
) -> impl IntoResponse {
    let workers = state
        .worksite_service
        .get_workers(GetWorkersInput {
            worksite_id: wallchart_id.clone(),
        })
        .await
        .expect("Failed to get worker");

    Html(html! {
        <PageLayout header="Assign Shift">
            <AssignShiftForm
                workers=workers
                action=format!("/wallcharts/{}/locations/{}/shifts/{}/workers/new", &wallchart_id, location_id, shift_id)
                create_worker_action=format!("/wallcharts/{}/workers/new", wallchart_id)
            />
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct AssignShiftFormData {
    worker_id: String,
}

async fn post_shift_assignment(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    flash: Flash,
    extract::Path((wallchart_id, location_id, shift_id)): extract::Path<(String, String, String)>,
    Form(form): Form<AssignShiftFormData>,
) -> impl IntoResponse {
    worksite_service
        .assign_worker(AssignWorkerInput {
            worksite_id: wallchart_id,
            location_id,
            shift_id,
            worker_id: form.worker_id,
        })
        .await
        .expect("Failed to assign worker");

    (
        StatusCode::OK,
        flash.success("Shift assigned successfully!"),
        [("hx-redirect", "/wallchart"), ("hx-retarget", "body")],
    )
}

#[props]
struct AssignShiftFormProps {
    #[builder(setter(into))]
    workers: Vec<Worker>,

    #[builder(setter(into))]
    action: String,

    #[builder(setter(into))]
    create_worker_action: String,
}

#[component]
fn AssignShiftForm(props: AssignShiftFormProps) -> String {
    html! {
        <div>
            <form hx-post=props.action>
                <div class="pb-12">
                    <p class="mt-1 text-sm leading-6 text-gray-600">
                        Assign a worker to this shift
                    </p>
                    <GridLayout class="mt-10">
                        <GridCell span=4>
                            <Label for_input="worker_id">Worker</Label>
                            <SelectInput name="worker_id" >
                            {
                                props
                                    .workers
                                    .iter()
                                    .map(|worker| async {
                                        html! {
                                            <option value=worker.id>{worker.full_name()}</option>
                                        }
                                    })
                                    .collect_fragment_async()
                                    .await
                            }
                            </SelectInput>
                        </GridCell>
                        <GridCell span=4>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <Button
                                    hx_get=props.create_worker_action
                                    hx_target="closest form"
                                >
                                    Create a new worker
                                </Button>
                                <Button kind="submit">Assign</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
