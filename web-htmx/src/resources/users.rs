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

use auth_service::{
    create_user::CreateUserInput, get_user::GetUserInput, get_user_for_login::GetUserForLoginInput,
};
use auth_service::{delete_user::DeleteUserInput, models::User};
use in_memory_user_repository::AuthContext;
use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    card::Card,
    form::{Button, GridCell, GridLayout, Label, TextInput},
    headers::SecondaryHeader,
    modal::{modal_target, Modal, ModalSize},
    table::{Confirm, DeleteActionLink, TDVariant, Table, TableData, TableHeading},
};

use crate::{
    components::{
        page::{PageHeader, PageLayout},
        page_content::PageContent,
    },
    routes::{
        self, home, login, users, users_new, users_new_modal, LOGIN, USER, USERS, USERS_NEW,
        USERS_NEW_MODAL,
    },
    state::WebHtmxState,
};

pub fn users_routes(state: WebHtmxState) -> Router {
    Router::new()
        .route(LOGIN, get(get_login).post(post_login))
        .route(USERS, get(get_users).post(post_users))
        .route(USERS_NEW, get(get_users_form))
        .route(USER, get(get_user_detail).delete(delete_user))
        .route(USERS_NEW_MODAL, get(get_users_form_modal))
        .with_state(state)
}

async fn get_login(State(_state): State<WebHtmxState>) -> impl IntoResponse {
    Html(html! {
        <PageLayout header="Login">
            <LoginForm login_route=login() />
        </PageLayout>
    })
}

#[derive(Deserialize, Debug)]
struct LoginForm {
    email: String,
    password: String,
}

async fn post_login(
    State(WebHtmxState {
        worksite_service: _,
        auth_service,
        flash_config: _,
    }): State<WebHtmxState>,
    mut auth: AuthContext,
    _flash: Flash,
    Form(login_form): Form<LoginForm>,
) -> impl IntoResponse {
    let result = auth_service
        .get_user_for_login(GetUserForLoginInput {
            email: login_form.email,
            password: login_form.password,
        })
        .await;

    match result {
        Ok(user) => match auth.login(&user).await {
            Ok(_) => (
                StatusCode::OK,
                [("hx-redirect", home()), ("hx-retarget", "body".to_string())],
            )
                .into_response(),
            Err(_) => (StatusCode::BAD_REQUEST, "Login failed").into_response(),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Login failed".to_string(),
        )
            .into_response(),
    }
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
                        hx_get=users_new_modal()
                        hx_target=modal_target()
                        hx_swap="beforeend"
                        hx_push_url=users_new()
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
                hx-get=routes::user(&user.id)
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
        [("hx-redirect", users()), ("hx-retarget", "body".into())],
    )
}

async fn get_users_form(headers: HeaderMap) -> impl IntoResponse {
    Html(html! {
        <PageLayout
            partial=headers.contains_key("Hx-Request")
            header="Add User"
        >
            <AddUserForm action=users() />
        </PageLayout>
    })
}

async fn get_users_form_modal() -> impl IntoResponse {
    Html(html! {
        <Modal size=ModalSize::MediumScreen>
            <SecondaryHeader
                title="Add User"
                subtitle="Enter user details below."
            />
            <AddUserForm action=users() />
        </Modal>
    })
}

#[props]
pub struct AddUserFormProps {
    #[builder(setter(into))]
    action: String,

    #[builder(setter(into), default)]
    email: String,

    #[builder(setter(into), default)]
    role: String,
}

#[component]
pub fn AddUserForm(props: AddUserFormProps) -> String {
    html! {
        <form hx-post=props.action>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "Please enter the user's information."
                </p>

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
                        <TextInput name="role" input_type="text" value=props.role />
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
            <AddUserForm
                action=routes::user(&user.id)
                email=user.email.clone()
                role="Organizer"
             />
        </PageLayout>
    })
}

#[props]
struct LoginFormProps {
    #[builder(setter(into))]
    login_route: String,
}

#[component]
fn LoginForm(props: LoginFormProps) -> String {
    html! {
        <form hx-post=props.login_route>
            <div class="pb-12">
                <p class="mt-1 text-sm leading-6 text-gray-600">
                    "pssst: try user@yallchart.com / password"
                </p>
                <GridLayout class="mt-10">
                    <GridCell span=4>
                        <Label for_input="email">Email</Label>
                        <TextInput input_type="email" name="email" autocomplete="email" />
                    </GridCell>
                    <GridCell span=4>
                        <Label for_input="password">Password</Label>
                        <TextInput input_type="password" name="password" autocomplete="password" />
                    </GridCell>
                    <GridCell span=4>
                        <div class="mt-6 flex items-center justify-end gap-x-6">
                            <Button kind="submit">Login</Button>
                        </div>
                    </GridCell>
                </GridLayout>
            </div>
        </form>
    }
}
