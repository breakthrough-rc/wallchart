/**
 * This module is a dumping ground of of routes plus functions for hydrating those routes
*
* so a const like "/something/:something_id"
* and a function like fn something(id: String) -> String { format!("/something/{}", id) }
 *
* The idea is that this will make is easier to refactor routes in the future and to avoid passing
* around "magic strings." Additionally, this helps the dependency graph by not having weird
* circular dependencies between difference resources and components.
*
* The downside is that these routes are not colocated in the appropriate
* resource module.
 */

pub const HOME: &str = "/";
pub fn home() -> String {
    HOME.into()
}

#[cfg(debug_assertions)]
pub const HOME_REDIRECT: &str = PLAYGROUND;

#[cfg(not(debug_assertions))]
pub const HOME_REDIRECT: &str = WALLCHART;

pub const PLAYGROUND: &str = "/playground";
pub fn _playground() -> String {
    PLAYGROUND.into()
}

pub const CLIENT: &str = "/client";
pub fn _client() -> String {
    CLIENT.into()
}

pub const LOGIN: &str = "/login";
pub fn login() -> String {
    LOGIN.into()
}

pub const LOGOUT: &str = "/logout";
pub fn logout() -> String {
    LOGOUT.into()
}

pub const WALLCHART: &str = "/wallchart";
pub fn wallchart() -> String {
    WALLCHART.into()
}

pub const WORKSITE: &str = "/worksites/:worksite_id";
pub fn worksite(worksite_id: &String) -> String {
    format!("/worksites/{}", worksite_id)
}

pub const WORKSITE_EDIT_FORM: &str = "/worksites/:worksite_id/edit-form";
pub fn worksite_edit_form(worksite_id: &String) -> String {
    format!("/worksites/{}/edit-form", worksite_id)
}

pub const WORKSITES: &str = "/worksites";
pub fn worksites() -> String {
    WORKSITES.into()
}

pub const WORKSITES_CREATE_FORM: &str = "/worksites/create-form";
pub fn worksites_create_form() -> String {
    WORKSITES_CREATE_FORM.into()
}

pub const LOCATIONS: &str = "/worksites/:worksite_id/locations";
pub fn locations(worksite_id: &String) -> String {
    format!("/worksites/{}/locations", worksite_id)
}

pub const LOCATIONS_CREATE_FORM: &str = "/worksites/:worksite_id/locations/create-form";
pub fn locations_create_form(worksite_id: &String) -> String {
    format!("/worksites/{}/locations/create-form", worksite_id)
}

pub const SHIFTS: &str = "/worksites/:worksite_id/locations/:location_id/shifts";
pub fn shifts(worksite_id: &String, location_id: &String) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts",
        worksite_id, location_id
    )
}

pub const SHIFTS_CREATE_FORM: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/create-form";
pub fn shifts_create_form(worksite_id: &String, location_id: &String) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/create-form",
        worksite_id, location_id
    )
}

pub const SHIFT_ASSIGNMENTS_CREATE_FORM: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/create-form";
pub fn shift_assignments_create_form(
    worksite_id: &String,
    location_id: &String,
    shift_id: &String,
) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/{}/workers/create-form",
        worksite_id, location_id, shift_id
    )
}

pub const SHIFT_ASSIGNMENT: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/:worker_id";
pub fn shift_assignment(
    worksite_id: &String,
    location_id: &String,
    shift_id: &String,
    worker_id: &String,
) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/{}/workers/{}",
        worksite_id, location_id, shift_id, worker_id
    )
}

pub const TAGS: &str = "/worksites/:worksite_id/tags";
pub fn tags(worksite_id: &String) -> String {
    format!("/worksites/{}/tags", worksite_id)
}

pub const TAGS_CREATE_FORM: &str = "/worksites/:worksite_id/tags/create-form";
pub fn tags_create_form(worksite_id: &String) -> String {
    format!("/worksites/{}/tags/create-form", worksite_id)
}

pub const TAG: &str = "/worksites/:worksite_id/tags/:tag_id";
pub fn tag(worksite_id: &String, tag_id: &String) -> String {
    format!("/worksites/{}/tags/{}", worksite_id, tag_id)
}

pub const TAG_EDIT_FORM: &str = "/worksites/:worksite_id/tags/:tag_id/edit-form";
pub fn tag_edit_form(worksite_id: &String, tag_id: &String) -> String {
    format!("/worksites/{}/tags/{}/edit-form", worksite_id, tag_id)
}

pub const WORKERS: &str = "/worksites/:worksite_id/workers";
pub fn workers(worksite_id: &String) -> String {
    format!("/worksites/{}/workers", worksite_id)
}

pub const WORKERS_CREATE_FORM: &str = "/worksites/:worksite_id/workers/create-form";
pub fn workers_create_form(worksite_id: &String) -> String {
    format!("/worksites/{}/workers/create-form", worksite_id)
}
pub fn workers_create_form_content(worksite_id: &String) -> String {
    format!("{}?content", workers_create_form(worksite_id))
}

pub const WORKER: &str = "/worksites/:worksite_id/workers/:worker_id";
pub fn worker(worksite_id: &String, worker_id: &String) -> String {
    format!("/worksites/{}/workers/{}", worksite_id, worker_id)
}

pub const WORKER_PROFILE: &str = "/worksites/:worksite_id/workers/:worker_id/profile";
pub fn worker_profile(worksite_id: &String, worker_id: &String) -> String {
    format!("/worksites/{}/workers/{}/profile", worksite_id, worker_id)
}

pub const USERS: &str = "/users";
pub fn users() -> String {
    USERS.into()
}

pub const USERS_CREATE_FORM: &str = "/users/create-form";
pub fn users_create_form() -> String {
    USERS_CREATE_FORM.into()
}

pub const USER: &str = "/users/:user_id";
pub fn user(user_id: &String) -> String {
    format!("/users/{}", user_id)
}

pub const USER_EDIT_FORM: &str = "/users/:user_id/edit-form";
pub fn user_edit_form(user_id: &String) -> String {
    format!("/users/{}/edit-form", user_id)
}

pub const ASSIGNED_TAGS: &str = "/worksites/:worksite_id/workers/:worker_id/tags";
pub fn assigned_tags(worksite_id: &String, worker_id: &String) -> String {
    format!("/worksites/{}/workers/{}/tags", worksite_id, worker_id)
}

pub const ASSIGNED_TAGS_FORM: &str = "/worksites/:worksite_id/workers/:worker_id/tags-form";
pub fn assigned_tags_form(worksite_id: &String, worker_id: &String) -> String {
    format!("/worksites/{}/workers/{}/tags-form", worksite_id, worker_id)
}

pub const CSV_UPLOAD: &str = "/csv-upload";
pub fn csv_upload() -> String {
    "/csv-upload".into()
}

pub const ASSESSMENTS: &str = "/worksites/:worksite_id/workers/:worker_id/assessments";
pub fn assessments(worksite_id: &String, worker_id: &String) -> String {
    format!(
        "/worksites/{}/workers/{}/assessments",
        worksite_id, worker_id
    )
}

pub const ASSESSMENT: &str =
    "/worksites/:worksite_id/workers/:worker_id/assessments/:assessment_id";
pub fn assessment(worksite_id: &String, worker_id: &String, assessment_id: &String) -> String {
    format!(
        "/worksites/{}/workers/{}/assessments/{}",
        worksite_id, worker_id, assessment_id
    )
}

pub const SELECTED_WORKSITE: &str = "/selected-worksite";
pub fn selected_worksite() -> String {
    SELECTED_WORKSITE.into()
}

pub const SELECTED_WORKSITE_MODAL: &str = "/selected-worksite/modal";
pub fn selected_worksite_modal() -> String {
    SELECTED_WORKSITE_MODAL.into()
}

pub fn page_modal_from(modal_resource_uri: String) -> String {
    let ctx: crate::context::Context =
        crate::context::context().expect("Unable to retrieve htmx context.");

    let page_url = format!("{}/", &ctx.page_url);

    if modal_resource_uri.starts_with(&page_url) {
        let query = modal_resource_uri.replace(&page_url, "?modal=");
        format!("{}{}", &ctx.page_url, query)
    } else {
        format!("{}?modal={}", &ctx.page_url, modal_resource_uri)
    }
}

pub const SUPPORT: &str = "/support";
pub fn support() -> String {
    SUPPORT.into()
}

pub const FORBIDDEN: &str = "/forbidden";
