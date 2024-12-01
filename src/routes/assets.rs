use crate::routes::HttpResponse;
use axum_typed_routing::route;

#[route(GET "/automatic_reload.js")]
pub async fn automatic_reload_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/automatic_reload.js"))
}

#[route(GET "/before-unload.js")]
pub async fn before_unload_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/before-unload.js"))
}

#[route(GET "/disable-clamav.js")]
pub async fn disable_clamav_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-clamav.js"))
}

#[route(GET "/disable-collabora.js")]
pub async fn disable_collabora_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-collabora.js"))
}

#[route(GET "/disable-docker-socket-proxy.js")]
pub async fn disable_docker_socket_proxy_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-docker-socket-proxy.js"))
}

#[route(GET "/disable-fulltextsearch.js")]
pub async fn disable_fulltextsearch_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-fulltextsearch.js"))
}

#[route(GET "/disable-imaginary.js")]
pub async fn disable_imaginary_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-imaginary.js"))
}

#[route(GET "/disable-onlyoffice.js")]
pub async fn disable_onlyoffice_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-onlyoffice.js"))
}

#[route(GET "/disable-talk.js")]
pub async fn disable_talk_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-talk.js"))
}

#[route(GET "/disable-talk-recording.js")]
pub async fn disable_talk_recording_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-talk-recording.js"))
}

#[route(GET "/disable-whiteboard.js")]
pub async fn disable_whiteboard_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/disable-whiteboard.js"))
}

#[route(GET "/forms.js")]
pub async fn forms_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/forms.js"))
}

#[route(GET "/options-form-submit.js")]
pub async fn options_form_submit_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/options-form-submit.js"))
}

#[route(GET "/robots.txt")]
pub async fn robots_txt() -> HttpResponse {
    HttpResponse::Text(include_str!("../../public/robots.txt"))
}

#[route(GET "/second-tab-warning.js")]
pub async fn second_tab_warning_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/second-tab-warning.js"))
}

#[route(GET "/style.css")]
pub async fn style_css() -> HttpResponse {
    HttpResponse::CSS(include_str!("../../public/style.css"))
}

#[route(GET "/timezone.js")]
pub async fn timezone_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/timezone.js"))
}

#[route(GET "/toggle-dark-mode.js")]
pub async fn toggle_dark_mode_js() -> HttpResponse {
    HttpResponse::JavaScript(include_str!("../../public/toggle-dark-mode.js"))
}

#[route(GET "/img/favicon.png")]
pub async fn favicon_png() -> HttpResponse {
    HttpResponse::PNG(include_bytes!("../../public/img/favicon.png"))
}

#[route(GET "/img/jenna-kim-the-globe.webp")]
pub async fn jenna_kim_the_globe_webp() -> HttpResponse {
    HttpResponse::WEBP(include_bytes!("../../public/img/jenna-kim-the-globe.webp"))
}

#[route(GET "/img/jenna-kim-the-globe-dark.webp")]
pub async fn jenna_kim_the_globe_dark_webp() -> HttpResponse {
    HttpResponse::WEBP(include_bytes!(
        "../../public/img/jenna-kim-the-globe-dark.webp"
    ))
}

#[route(GET "/img/nextcloud-logo.svg")]
pub async fn nextcloud_logo_svg() -> HttpResponse {
    HttpResponse::SVG(include_str!("../../public/img/nextcloud-logo.svg"))
}
