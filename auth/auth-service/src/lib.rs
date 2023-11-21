use axum_login::RequireAuthorizationLayer;
use models::User;

//##PLOP INSERT MOD HOOK##
pub mod get_user;
pub mod delete_user;
pub mod get_users;
pub mod create_user;
pub mod get_user_for_login;
pub mod models;
pub mod ports;
pub mod service;

pub type RequireAuth = RequireAuthorizationLayer<String, User, ()>;
