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

pub const WORKSITES_MODAL: &str = "/worksites/modal";
pub fn worksites_modal() -> String {
    WORKSITES_MODAL.into()
}

pub const LOCATIONS: &str = "/worksites/:worksite_id/locations";
pub fn locations(worksite_id: &String) -> String {
    format!("/worksites/{}/locations", worksite_id)
}

pub const LOCATIONS_CREATE_FORM: &str = "/worksites/:worksite_id/locations/create-form";
pub fn locations_create_form(worksite_id: &String) -> String {
    format!("/worksites/{}/locations/create-form", worksite_id)
}

pub const _LOCATIONS_NEW: &str = "/worksites/:worksite_id/locations/new";
pub fn locations_new(worksite_id: &String) -> String {
    format!("/worksites/{}/locations/new", worksite_id)
}

pub const SHIFTS: &str = "/worksites/:worksite_id/locations/:location_id/shifts";
pub fn shifts(worksite_id: &String, location_id: &String) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts",
        worksite_id, location_id
    )
}

pub const SHIFTS_NEW_MODAL: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/new-modal";
pub fn shifts_new_modal(worksite_id: &String, location_id: &String) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/new-modal",
        worksite_id, location_id
    )
}

pub const _SHIFTS_NEW: &str = "/worksites/:worksite_id/locations/:location_id/shifts/new";
pub fn _shifts_new(worksite_id: &String, location_id: &String) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/new",
        worksite_id, location_id
    )
}

pub const SHIFT_ASSIGNMENTS_NEW: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new";
pub fn shift_assignments_new(
    worksite_id: &String,
    location_id: &String,
    shift_id: &String,
) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/{}/workers/new",
        worksite_id, location_id, shift_id
    )
}

pub const SHIFT_ASSIGNMENTS_NEW_MODAL: &str =
    "/worksites/:worksite_id/locations/:location_id/shifts/:shift_id/workers/new-modal";
pub fn shift_assignments_new_modal(
    worksite_id: &String,
    location_id: &String,
    shift_id: &String,
) -> String {
    format!(
        "/worksites/{}/locations/{}/shifts/{}/workers/new-modal",
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

pub const WORKERS_NEW: &str = "/worksites/:worksite_id/workers/new";
pub fn workers_new(worksite_id: &String) -> String {
    format!("/worksites/{}/workers/new", worksite_id)
}

pub const WORKERS_NEW_MODAL: &str = "/worksites/:worksite_id/workers/new-modal";
pub fn workers_new_modal(worksite_id: &String) -> String {
    format!("/worksites/{}/workers/new-modal", worksite_id)
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

pub const USERS_NEW: &str = "/users/new";
pub fn users_new() -> String {
    USERS_NEW.into()
}

pub const USERS_NEW_MODAL: &str = "/users/new-modal";
pub fn users_new_modal() -> String {
    USERS_NEW_MODAL.into()
}

pub const USER: &str = "/users/:user_id";
pub fn user(user_id: &String) -> String {
    format!("/users/{}", user_id)
}

pub const USER_MODAL: &str = "/users/:user_id/modal";
pub fn user_modal(user_id: &String) -> String {
    format!("/users/{}/modal", user_id)
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
    let query = modal_resource_uri.replace(&page_url, "?modal=");

    format!("{}{}", &ctx.page_url, query)
}
