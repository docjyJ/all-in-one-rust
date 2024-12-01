pub mod api_auth;
pub mod api_configuration;
pub mod api_docker;
pub mod assets;
pub mod view;

use askama::Template;
use axum::http::{header, StatusCode};
use axum::response::{Html, IntoResponse, Response};

mod error {
    use askama::Template;
    use axum::response::{Html, IntoResponse, Response};

    #[derive(Template)]
    #[template(path = "error.askama.html")]
    pub struct ErrorTemplate {
        pub error: String,
    }

    pub struct ErrorHtml(pub String);

    impl IntoResponse for ErrorHtml {
        fn into_response(self) -> Response {
            ErrorTemplate { error: self.0 }.render().map_or_else(
                |e| Html(e.to_string()).into_response(),
                |s| Html(s).into_response(),
            )
        }
    }
}

pub enum HttpResponse {
    JavaScript(&'static str),
    CSS(&'static str),
    Text(&'static str),
    WEBP(&'static [u8]),
    SVG(&'static str),
    PNG(&'static [u8]),
    CreatedAndRedirect(&'static str),
    InlineText(String),
    TemporaryRedirect(&'static str),
    HTML(String),
    UnauthorizedRedirect(&'static str),
    Error(String),
    NotFound,
}

impl HttpResponse {
    pub fn html_template<T: Template>(template: T) -> Self {
        template.render().map_or_else(
            |e| HttpResponse::Error(e.to_string()),
            |s| HttpResponse::HTML(s),
        )
    }
}

impl IntoResponse for HttpResponse {
    fn into_response(self) -> Response {
        match self {
            HttpResponse::JavaScript(body) => {
                ([(header::CONTENT_TYPE, "text/javascript")], body).into_response()
            }
            HttpResponse::CSS(body) => ([(header::CONTENT_TYPE, "text/css")], body).into_response(),
            HttpResponse::Text(body) => {
                ([(header::CONTENT_TYPE, "text/plain")], body).into_response()
            }
            HttpResponse::WEBP(body) => {
                ([(header::CONTENT_TYPE, "image/webp")], body).into_response()
            }
            HttpResponse::SVG(body) => {
                ([(header::CONTENT_TYPE, "image/svg+xml")], body).into_response()
            }
            HttpResponse::PNG(body) => {
                ([(header::CONTENT_TYPE, "image/png")], body).into_response()
            }
            HttpResponse::CreatedAndRedirect(location) => {
                ([(header::LOCATION, location)], "").into_response()
            }
            HttpResponse::InlineText(body) => {
                ([(header::CONTENT_DISPOSITION, "inline")], body).into_response()
            }
            HttpResponse::TemporaryRedirect(location) => (
                StatusCode::TEMPORARY_REDIRECT,
                [(header::LOCATION, location)],
            )
                .into_response(),
            HttpResponse::HTML(body) => Html(body).into_response(),
            HttpResponse::UnauthorizedRedirect(location) => {
                (StatusCode::UNAUTHORIZED, [(header::LOCATION, location)]).into_response()
            }
            HttpResponse::NotFound => StatusCode::NOT_FOUND.into_response(),
            HttpResponse::Error(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                error,
            )
                .into_response(),
        }
    }
}
