use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::{delete, get},
    Form, Router,
};
use axum_flash::Flash;
use futures::future::join_all;
use http::{HeaderMap, StatusCode};
use rscx::{component, html, props};
use serde::Deserialize;

use auth_service::{
    create_user::CreateUserInput, get_user::GetUserInput, update_user::UpdateUserInput,
};
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
        .route(routes::USERS, get(get_users))
        .route(
            routes::USERS_CREATE_FORM,
            get(get_create_form).post(post_create_form),
        )
        .route(
            routes::USER_EDIT_FORM,
            get(get_edit_form).post(post_edit_form),
        )
        .route(routes::USER, delete(delete_user))
        .with_state(state)
}

async fn get_users(State(state): State<WebHtmxState>) -> impl IntoResponse {
    let users = state
        .auth_service
        .get_users()
        .await
        .expect("Failed to get users");

    let presenter = UsersTablePresenter::new(users);
    let view_model: UsersTableProps = presenter.into();

    Html(html! {
        <PageLayout
            header=PageHeader::Toolbar {
                title: "Users".into(),
                buttons: html! {
                    <PrimaryButton
                        hx_get=routes::users_create_form()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=routes::page_modal_from(routes::users_create_form())
                    >
                        Add New User
                    </PrimaryButton>
                }
            }
        >
            <PageContent title="Add, edit, remove Users">
                <Card>
                    <UsersTable users=view_model.users />
                </Card>
            </PageContent>
        </PageLayout>
    })
}

struct UsersTablePresenter {
    users: Vec<User>, // TODO This should be the out model not domain model
}

impl UsersTablePresenter {
    fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}

impl From<UsersTablePresenter> for UsersTableProps {
    fn from(presenter: UsersTablePresenter) -> Self {
        Self {
            users: presenter
                .users
                .into_iter()
                .map(|user| UserVM {
                    edit_form_url: routes::user_edit_form(&user.id),
                    delete_url: routes::user(&user.id),
                    email: user.email,
                    role: user.role,
                })
                .collect(),
        }
    }
}

#[props]
struct UsersTableProps {
    users: Vec<UserVM>,
}

#[component]
fn UsersTable(props: UsersTableProps) -> String {
    html! {
        <Table
            headings=vec![
                TableHeading::title("Email"),
                TableHeading::title("Role"),
                TableHeading::empty("Actions"),
            ]
            body=join_all(props
                .users
                .into_iter()
                .map(|user| async {
                    html! {
                        <UserTableRow user=user />
                    }
                }))
                .await
        />
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserVM {
    // routes::user_edit_form(&user.id)
    edit_form_url: String,

    // routes::user(&user.id)
    delete_url: String,
    email: String,
    role: String,
}

#[props]
pub struct UserTableRowProps {
    user: UserVM,
}

#[component]
fn UserTableRow(props: UserTableRowProps) -> String {
    html! {
        <TableData variant=TDVariant::First>
            <button
                hx-get=props.user.edit_form_url
                hx-target=modal_target()
                hx-push-url=routes::page_modal_from(props.user.edit_form_url.clone())
            >
                {&props.user.email}
            </button>
        </TableData>
        <TableData>
            {&props.user.role}
        </TableData>
        <TableData variant=TDVariant::Last>
            <DeleteActionLink
                hx_delete=props.user.delete_url
                hx_swap="outerHTML swap:1s"
                hx_target="closest tr"
                confirm=Confirm {
                    title: "Remove User".into(),
                    message: format!("Are you sure you want to remove this user: {}?", &props.user.email),
                }
                sr_text=&props.user.email
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

async fn post_create_form(
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

async fn get_create_form(headers: HeaderMap) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
            header="Add User"
        >
            <Modal size=ModalSize::MediumScreen>
                <SecondaryHeader
                    title="👤 Add User"
                    subtitle="Enter user details below."
                />
                <UserForm action=routes::users_create_form() />
            </Modal>
        </PageLayout>
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

    #[builder(setter(into), default = true)]
    show_password: bool,
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

                    {
                        if props.show_password {
                            html! {
                                <GridCell span=3>
                                    <Label for_input="password">Password</Label>
                                    <TextInput name="password" autocomplete="password" input_type="password" />
                                </GridCell>
                            }
                        }
                        else { "".into() }
                    }

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

async fn get_edit_form(
    extract::Path(user_id): extract::Path<String>,
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
    headers: HeaderMap,
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
        <PageLayout
            partial=headers.contains_key("Hx-Request")
            header="Edit User"
        >
            <Modal size=ModalSize::MediumScreen>
                <SecondaryHeader
                    title="👤 Edit User"
                    subtitle="Make changes to the user below."
                />
                <UserForm
                    action=routes::user_edit_form(&user.id)
                    email=user.email.clone()
                    role=user.role
                    show_password=false
                />
            </Modal>
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct UpdateUserFormData {
    email: String,
    role: String,
}

async fn post_edit_form(
    extract::Path(user_id): extract::Path<String>,
    State(WebHtmxState { auth_service, .. }): State<WebHtmxState>,
    flash: Flash,
    Form(form): Form<UpdateUserFormData>,
) -> impl IntoResponse {
    auth_service
        .update_user(UpdateUserInput {
            user_id,
            email: form.email,
            role: form.role,
        })
        .await
        .expect("Failed to update user");
    (
        StatusCode::OK,
        flash.success("Updated user successfully!"),
        [
            ("hx-redirect", routes::users()),
            ("hx-retarget", "body".into()),
        ],
    )
}
