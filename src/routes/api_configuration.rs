pub use root::handler;

mod root {
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/configuration")]
    pub async fn handler() -> HttpResponse {
        // TODO \AIO\Controller\ConfigurationController::class . ':SetConfig'
        panic!("Not implemented") // TODO
    }
}
