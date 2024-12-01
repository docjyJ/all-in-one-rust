pub use getlogin::handler as getlogin_handler;
pub use login::handler as login_handler;
pub use logout::handler as logout_handler;

mod logout {
    use crate::auth::clear_auth;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;
    use tower_sessions::Session;

    #[route(POST "/api/auth/logout")]
    pub async fn handler(session: Session) -> HttpResponse {
        match clear_auth(&session).await {
            Ok(()) => HttpResponse::TemporaryRedirect("/login"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod login {
    use crate::auth::set_auth_from_password;
    use crate::routes::HttpResponse;
    use axum::Form;
    use axum_typed_routing::route;
    use serde::Deserialize;
    use tower_sessions::Session;

    #[derive(Deserialize)]
    pub struct PasswordForm {
        pub password: String,
    }
    #[route(POST "/api/auth/login")]
    pub async fn handler(
        session: Session,
        Form(PasswordForm { password }): Form<PasswordForm>,
    ) -> HttpResponse {
        match set_auth_from_password(&session, &password).await {
            Ok(true) => HttpResponse::TemporaryRedirect("/"),
            Ok(false) => HttpResponse::UnauthorizedRedirect("/login"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod getlogin {
    use crate::auth::set_auth_from_token;
    use crate::routes::HttpResponse;
    use axum::extract::Query;
    use axum_typed_routing::route;
    use serde::Deserialize;
    use tower_sessions::Session;

    #[derive(Deserialize)]
    pub struct TokenQuery {
        pub token: String,
    }

    #[route(GET "/api/auth/getlogin")]
    pub async fn handler(
        session: Session,
        Query(TokenQuery { token }): Query<TokenQuery>,
    ) -> HttpResponse {
        match set_auth_from_token(&session, &token).await {
            Ok(true) => HttpResponse::TemporaryRedirect("/"),
            Ok(false) => HttpResponse::UnauthorizedRedirect("/login"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}
