mod auth_middleware;
mod controller;
mod password_generator;

pub use crate::auth::controller::{
    can_be_installed, clear_auth, is_authenticated, set_auth_from_password, set_auth_from_token,
    setup_password,
};
