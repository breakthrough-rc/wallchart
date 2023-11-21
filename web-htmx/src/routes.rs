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
pub fn _home() -> String {
    HOME.into()
}

pub const PLAYGROUND: &str = "/playground";
pub fn _playground() -> String {
    PLAYGROUND.into()
}

pub const CLIENT: &str = "/client";
pub fn _client() -> String {
    CLIENT.into()
}

pub const _LOGIN: &str = "/login";
pub fn _login() -> String {
    _LOGIN.into()
}

pub const WALLCHART: &str = "/wallchart";
pub fn wallchart() -> String {
    WALLCHART.into()
}

pub const LOCATIONS_NEW_MODAL: &str = "/worksites/:worksite_id/locations/new-modal";
pub fn locations_new_modal(worksite_id: &String) -> String {
    format!("/worksites/{}/locations/new-modal", worksite_id)
}

pub const LOCATIONS_NEW: &str = "/worksites/:worksite_id/locations/new";
pub fn locations_new(worksite_id: &String) -> String {
    format!("/worksites/{}/locations/new", worksite_id)
}

pub const TAGS: &str = "/worksites/:worksite_id/tags";
pub fn tags(worksite_id: &String) -> String {
    format!("/worksites/{}/tags", worksite_id)
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

pub const WORKER_TAGS_FORM: &str = "/worksites/:worksite_id/workers/:worker_id/tags-form";
pub fn worker_tags_form(worksite_id: &String, worker_id: &String) -> String {
    format!("/worksites/{}/workers/{}/tags-form", worksite_id, worker_id)
}
