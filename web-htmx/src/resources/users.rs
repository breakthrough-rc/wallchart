use crate::{
    components::{
        login_form::LoginForm,
        page::{PageHeader, PageLayout},
    },
    routes::{
        self, home, login, user, users, users_new, users_new_modal, LOGIN, USER, USERS, USERS_NEW,
        USERS_NEW_MODAL,
    },
    state::WebHtmxState,
};
use auth_service::{
    create_user::CreateUserInput, get_user::GetUserInput, get_user_for_login::GetUserForLoginInput,
};
use auth_service::{delete_user::DeleteUserInput, models::User};
use axum::{
    extract::{self, State},
    response::{Html, IntoResponse},
    routing::get,
    Form, Router,
};
use axum_flash::Flash;
use http::{HeaderMap, StatusCode};
use in_memory_user_repository::AuthContext;
use rscx::{component, html, props, CollectFragmentAsync};
use serde::Deserialize;
use web_client::server::{
    attrs::Attrs,
    button::PrimaryButton,
    form::{Button, GridCell, GridLayout, Label, TextInput},
    modal::{Modal, ModalSize},
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
                        hx_target="body"
                        hx_swap="beforeend"
                        hx_push_url=users_new()
                    >
                        Add New User
                    </PrimaryButton>
                }
            }
        >
            <Users users=users />
        </PageLayout>
    })
}

#[props]
pub struct UsersProps {
    users: Vec<User>,
}

#[component]
pub fn Users(props: UsersProps) -> String {
    html! {
        <table class="min-w-full divide-y divide-gray-300">
            <thead class="bg-gray-50">

                <tr>
                    <th scope="col" class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-3">Email</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">Role</th>
                    <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"></th>
                </tr>
            </thead>
            <tbody class="bg-white">
                {
                    props
                        .users
                        .iter()
                        .map(|user| async {
                            html! {
                                <User
                                    user=user.clone()
                                />
                            }
                        })
                        .collect_fragment_async()
                        .await
                }
            </tbody>
        </table>
    }
}

#[props]
pub struct UserProps {
    user: User,
}

#[component]
pub fn User(props: UserProps) -> String {
    html! {
        <tr class="border-t border-gray-300" data-loading-states>
            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-3">
                  <button
                      hx-get=user(&props.user.id)
                      hx-target="body"
                  >
                      {props.user.email}
                  </button>
            </td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">Organizer</td>
            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                <button
                    type="button"
                    hx-delete={user(&props.user.id)}
                    class="text-center inline-flex items-center rounded bg-white px-2 py-1 text-xs font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 disabled:bg-gray-50 disabled:shadow-none disabled:cursor-not-allowed disabled:text-gray-500"
                    hx-swap="outerHTML swap:1s"
                    hx-target="closest tr"
                    data-loading-disable
                >
                    <div
                        class="htmx-indicator inline-flex animate-spin mr-2 items-center justify-center rounded-full w-4 h-4 bg-gradient-to-tr from-gray-500 to-white"
                    >
                        <span class="inline h-3 w-3 rounded-full bg-white hover:bg-gray-50"></span>
                    </div>
                    Delete
                </button>
            </td>
        </tr>
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
                role="Organizer" />
        </PageLayout>
    })
}
