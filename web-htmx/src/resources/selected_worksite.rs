use crate::routes;
use crate::state::WebHtmxState;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, put},
    Form, Router,
};
use axum_login::tower_sessions::Session;
use http::StatusCode;
use rscx::{html, CollectFragmentAsync};
use serde::Deserialize;
use web_client::server::{
    attrs::Attrs,
    form::{Button, GridCell, GridLayout, Select, SelectOption},
    headers::SecondaryHeader,
    modal::{Modal, ModalSize},
};
use worksite_service::get_worksite::GetWorksiteInput;

pub fn selected_worksite_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::SELECTED_WORKSITE, put(put_selected_worksite))
        .route(
            routes::SELECTED_WORKSITE_MODAL,
            get(get_selected_worksite_modal),
        )
        .with_state(state)
}

#[derive(Deserialize, Debug)]
struct SetSelectedWorksiteFormData {
    selected_worksite_id: String,
}

async fn put_selected_worksite(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
    session: Session,
    Form(form): Form<SetSelectedWorksiteFormData>,
) -> impl IntoResponse {
    let worksite = worksite_service
        .get_worksite(GetWorksiteInput {
            id: form.selected_worksite_id.clone(),
        })
        .await
        .unwrap()
        .ok_or("Worksite not found")
        .unwrap();

    session.insert_value(
        "selected_worksite_id",
        form.selected_worksite_id.clone().into(),
    );
    session.insert_value("selected_worksite_name", worksite.name.clone().into());

    (
        StatusCode::OK,
        [
            ("hx-redirect", routes::worksite(&form.selected_worksite_id)),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn get_selected_worksite_modal(
    State(WebHtmxState {
        worksite_service, ..
    }): State<WebHtmxState>,
) -> impl IntoResponse {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");
    let worksite_id = ctx.worksite_id.clone();
    let worksites = worksite_service.get_worksites().await.unwrap();

    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="View Worksite"
                subtitle="Select a worksite to view."
            />
            <form hx-put=routes::selected_worksite()>
                <GridLayout>
                    <GridCell>
                        <Select
                            name="selected_worksite_id"
                        >
                        {
                            worksites
                                .iter()
                                .map(|w| async {
                                    html! {
                                        <SelectOption
                                            value=w.id.clone()
                                            selected=w.id == worksite_id
                                        >
                                            {w.name.clone()}
                                        </SelectOption>
                                    }
                                })
                                .collect_fragment_async()
                                .await
                        }
                        </Select>
                    </GridCell>
                    <GridCell>
                        <Button kind="submit">Go</Button>
                    </GridCell>
                </GridLayout>
            </form>
        </Modal>
    })
}
