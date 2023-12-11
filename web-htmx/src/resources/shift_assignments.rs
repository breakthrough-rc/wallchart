use crate::{components::page::PageLayout, routes, state::WebHtmxState};
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
    button::SecondaryButton,
    form::{Button, GridCell, GridLayout, Label, Select, SelectOption},
    headers::SecondaryHeader,
    modal::{Modal, ModalSize},
};
use worksite_service::{
    assign_worker::AssignWorkerInput, get_workers::GetWorkersInput, models::Worker,
    remove_worker_from_shift::RemoveWorkerFromShiftInput,
};

pub fn shift_assignments_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(
            routes::SHIFT_ASSIGNMENTS_CREATE_FORM,
            get(get_shift_assignment_create_form).post(post_shift_assignment),
        )
        .route(routes::SHIFT_ASSIGNMENT, delete(delete_worker_from_shift))
        .with_state(state)
}

async fn delete_worker_from_shift(
    extract::Path((worksite_id, _location_id, shift_id, worker_id)): extract::Path<(
        String,
        String,
        String,
        String,
    )>,
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
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

async fn get_shift_assignment_create_form(
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
        <PageLayout
            header="Assign a Shift"
        >
            <Modal size=ModalSize::MediumScreen>
                <SecondaryHeader
                    title="👤 Assign a Shift"
                    subtitle="Assign a worker to this shift."
                />
                <AssignShiftForm
                    workers=workers
                    action=routes::shift_assignments_create_form(&wallchart_id, &location_id, &shift_id)
                    create_worker_action=routes::workers_create_form_content(&wallchart_id)
                />
            </Modal>
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
        [
            ("hx-redirect", routes::wallchart()),
            ("hx-retarget", "body".into()),
        ],
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
                    <GridLayout class="mt-10">
                        <GridCell>
                            <Label for_input="worker_id">Worker</Label>
                            <Select name="worker_id" >
                            {
                                props
                                    .workers
                                    .iter()
                                    .map(|worker| async {
                                        html! {
                                            <SelectOption value=worker.id.to_string()>{worker.full_name()}</SelectOption>
                                        }
                                    })
                                    .collect_fragment_async()
                                    .await
                            }
                            </Select>
                        </GridCell>
                        <GridCell>
                            <div class="mt-6 flex items-center justify-end gap-x-6">
                                <Button
                                    onclick="history.go(-1)"
                                    attrs=Attrs::with("data-toggle-action", "close".into())
                                >
                                    Cancel
                                </Button>
                                <SecondaryButton
                                    hx_get=props.create_worker_action
                                    hx_target="closest form"
                                >
                                    Create a new worker
                                </SecondaryButton>
                                <Button kind="submit">Assign</Button>
                            </div>
                        </GridCell>
                    </GridLayout>
                </div>
            </form>
        </div>
    }
}
