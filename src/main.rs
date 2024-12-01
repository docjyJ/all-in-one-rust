mod auth;
mod configuration;
mod container;
mod cron;
mod data;
mod docker_client;
mod routes;

use axum::Router;
use axum_typed_routing::TypedRouter;
use std::net::SocketAddr;
use time::Duration;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("initializing router...");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    let app = Router::new()
        .typed_route(routes::view::index_handler)
        .typed_route(routes::view::setup_handler)
        .typed_route(routes::view::login_handler)
        .typed_route(routes::view::containers_handler)
        .typed_route(routes::api_auth::getlogin_handler)
        .typed_route(routes::api_auth::login_handler)
        .typed_route(routes::api_auth::logout_handler)
        .typed_route(routes::api_configuration::handler)
        .typed_route(routes::api_docker::backup_handler)
        .typed_route(routes::api_docker::backup_check_handler)
        .typed_route(routes::api_docker::backup_check_repair_handler)
        .typed_route(routes::api_docker::backup_test_handler)
        .typed_route(routes::api_docker::getwatchtower_handler)
        .typed_route(routes::api_docker::logs_handler)
        .typed_route(routes::api_docker::restore_handler)
        .typed_route(routes::api_docker::start_handler)
        .typed_route(routes::api_docker::stop_handler)
        .typed_route(routes::api_docker::watchtower_handler)
        .typed_route(routes::assets::automatic_reload_js)
        .typed_route(routes::assets::before_unload_js)
        .typed_route(routes::assets::disable_clamav_js)
        .typed_route(routes::assets::disable_collabora_js)
        .typed_route(routes::assets::disable_docker_socket_proxy_js)
        .typed_route(routes::assets::disable_fulltextsearch_js)
        .typed_route(routes::assets::disable_imaginary_js)
        .typed_route(routes::assets::disable_onlyoffice_js)
        .typed_route(routes::assets::disable_talk_js)
        .typed_route(routes::assets::disable_talk_recording_js)
        .typed_route(routes::assets::disable_whiteboard_js)
        .typed_route(routes::assets::forms_js)
        .typed_route(routes::assets::options_form_submit_js)
        .typed_route(routes::assets::robots_txt)
        .typed_route(routes::assets::second_tab_warning_js)
        .typed_route(routes::assets::style_css)
        .typed_route(routes::assets::timezone_js)
        .typed_route(routes::assets::toggle_dark_mode_js)
        .typed_route(routes::assets::favicon_png)
        .typed_route(routes::assets::jenna_kim_the_globe_webp)
        .typed_route(routes::assets::jenna_kim_the_globe_dark_webp)
        .typed_route(routes::assets::nextcloud_logo_svg)
        .layer(session_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("router initialized, now listening on port {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
//
// use crate::routes::assets;
// use actix_web::{App, HttpServer};
// use actix_web::middleware::Logger;
// use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore, config::CookieContentSecurity};
// use actix_web::cookie::{Key, SameSite};
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(
//         || App::new()
//             .wrap(Logger::default())
//             .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
//                 .cookie_content_security(CookieContentSecurity::Private)
//                 .cookie_same_site(SameSite::Lax)
//                 .build())
//             .service(assets::automatic_reload_js)
//             .service(assets::before_unload_js)
//             .service(assets::disable_clamav_js)
//             .service(assets::disable_collabora_js)
//             .service(assets::disable_docker_socket_proxy_js)
//             .service(assets::disable_fulltextsearch_js)
//             .service(assets::disable_imaginary_js)
//             .service(assets::disable_onlyoffice_js)
//             .service(assets::disable_talk_js)
//             .service(assets::disable_talk_recording_js)
//             .service(assets::disable_whiteboard_js)
//             .service(assets::forms_js)
//             .service(assets::options_form_submit_js)
//             .service(assets::robots_txt)
//             .service(assets::second_tab_warning_js)
//             .service(assets::style_css)
//             .service(assets::timezone_js)
//             .service(assets::toggle_dark_mode_js)
//             .service(assets::favicon_png)
//             .service(assets::jenna_kim_the_globe_webp)
//             .service(assets::jenna_kim_the_globe_dark_webp)
//             .service(assets::nextcloud_logo_svg))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }
//
