use axum_login::RequireAuthorizationLayer;
use models::User;

pub mod create_user;
pub mod delete_user;
pub mod get_user;
pub mod get_user_for_login;
pub mod get_users;
pub mod models;
pub mod ports;
pub mod service;

pub type RequireAuth = RequireAuthorizationLayer<String, User, ()>;
