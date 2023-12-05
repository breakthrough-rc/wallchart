use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use futures::future::join_all;
use http::{HeaderMap, StatusCode};
use rscx::{component, html, props};
use serde::Deserialize;

use auth_service::{create_user::CreateUserInput, get_user::GetUserInput};
use auth_service::{delete_user::DeleteUserInput, models::User};
use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    card::Card,
    form::{Button, GridCell, GridLayout, Label, Select, SelectOption, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal, ModalSize},
    table::{Confirm, DeleteActionLink, TDVariant, Table, TableData, TableHeading},
};

use crate::{
    components::{
        page::{PageHeader, PageLayout},
        page_content::PageContent,
    },
    routes,
    state::WebHtmxState,
};

pub fn users_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(routes::USERS, get(get_users).post(post_users))
        .route(routes::USERS_NEW, get(get_users_form))
        .route(routes::USER, get(get_user_detail).delete(delete_user))
        .route(routes::USER_MODAL, get(get_user_detail_modal))
        .route(routes::USERS_NEW_MODAL, get(get_users_form_modal))
        .with_state(state)
}

async fn get_users(State(state): State<WebHtmxState>) -> impl IntoResponse {
    let users = state
        .auth_service
        .get_users()
        .await
        .expect("Failed to get users");

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Users".into(),
                buttons: html! {
                    <PrimaryButton
                        hx_get=routes::users_new_modal()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::users_new()
                    >
                        Add New User
                    </PrimaryButton>
                }
            }
        >
            <PageContent title="Add, edit, remove Users">
                <Card>
                    <UsersTable users=users />
                </Card>
            </PageContent>
        </PageLayout>
    })
}

#[component]
fn UsersTable(users: Vec<User>) -> String {
    html! {
        <Table
            headings=vec![
                TableHeading::title("Email"),
                TableHeading::title("Role"),
                TableHeading::empty("Actions"),
            ]
            body=join_all(users
                .iter()
                .map(|user| async {
                    html! {
                        <UserTableRow user=user.clone() />
                    }
                }))
                .await
        />
    }
}

#[component]
pub fn UserTableRow(user: User) -> String {
    html! {
        <TableData variant=TDVariant::First>
            <button
                hx-get=routes::user_modal(&user.id)
                hx-target=modal_target()
            >
                {&user.email}
            </button>
        </TableData>
        <TableData>
            Organizer
        </TableData>
        <TableData variant=TDVariant::Last>
            <DeleteActionLink
                hx_delete=routes::user(&user.id)
                hx_swap="outerHTML swap:1s"
                hx_target="closest tr"
                confirm=Confirm {
                    title: "Remove User".into(),
                    message: format!("Are you sure you want to remove this user: {}?", &user.email),
                }
                sr_text=&user.email
                show_loader_on_delete=true
            >
                Remove
            </DeleteActionLink>
        </TableData>
    }
}

#[derive(Deserialize, Debug)]
struct AddUserFormData {
    email: String,
    password: String,
}

async fn post_users(
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<AddUserFormData>,
) -> impl IntoResponse {
    auth_service
        .create_user(CreateUserInput {
            email: form.email,
            password: form.password,
        })
        .await
        .expect("Failed to add user");
    (
        StatusCode::OK,
        flash.success("User added successfully!"),
        [
            ("hx-redirect", routes::users()),
            ("hx-retarget", "body".into()),
        ],
    )
}

async fn get_users_form(headers: HeaderMap) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
            header="Add User"
        >
            <UserForm action=routes::users() />
        </PageLayout>
    })
}

async fn get_users_form_modal() -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="👤 Add User"
                subtitle="Enter user details below."
            />
            <UserForm action=routes::users() />
        </Modal>
    })
}

#[props]
pub struct UserFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(setter(into), default)]
    email: String,

    #[builder(setter(into), default)]
    role: String,
}

#[component]
pub fn UserForm(props: UserFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <GridLayout class="mt-10">
                    <GridCell span=3>
                        <Label for_input="email">Email</Label>
                        <TextInput name="email" autocomplete="email" input_type="email" value=props.email/>
                    </GridCell>

                    <GridCell span=3>
                        <Label for_input="password">Password</Label>
                        <TextInput name="password" autocomplete="password" input_type="password" />
                    </GridCell>

                    <GridCell span=3>
                        <Label for_input="role">Role</Label>
                        <Select name="role">
                            <SelectOption selected=props.role == "Organizer">Organizer</SelectOption>
                            <SelectOption selected=props.role == "Admin">Admin</SelectOption>
                            <SelectOption selected=props.role == "SuperAdmin">SuperAdmin</SelectOption>
                        </Select>
                    </GridCell>
                </GridLayout>
            </div>
            <div class="mt-6 flex items-center justify-end gap-x-6">
                <Button
                    onclick="history.go(-1)"
                    attrs=Attrs::with("data-toggle-action", "close".into())
                >
                    Cancel
                </Button>
                <Button kind="submit">Save</Button>
            </div>
        </form>
    }
}

async fn delete_user(
    extract::Path(user_id): extract::Path<String>,
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    let result = auth_service
        .delete_user(DeleteUserInput {
            user_id: user_id.clone(),
        })
        .await;

    match result {
        Ok(_) => "".into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error deleting user").into_response(),
    }
}

async fn get_user_detail(
    extract::Path(user_id): extract::Path<String>,
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    let user = auth_service
        .get_user(GetUserInput {
            user_id: user_id.clone(),
        })
        .await
        .expect("Failed to get user")
        .ok_or("User not found")
        .expect("User not found");

    Html(html! {
        <PageLayout header=user.email.clone()>
            <UserForm
                action=routes::user(&user.id)
                email=user.email.clone()
                role="Organizer"
             />
        </PageLayout>
    })
}

async fn get_user_detail_modal(
    extract::Path(user_id): extract::Path<String>,
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
) -> impl IntoResponse {
    let user = auth_service
        .get_user(GetUserInput {
            user_id: user_id.clone(),
        })
        .await
        .expect("Failed to get user")
        .ok_or("User not found")
        .expect("User not found");

    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="👤 Edit User"
                subtitle="Make changes to the user below."
            />
            <UserForm
                action=routes::user(&user.id)
                email=user.email.clone()
                role="Organizer"
             />
        </Modal>
    })
}
