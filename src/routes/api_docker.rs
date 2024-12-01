pub use backup::handler as backup_handler;
pub use backup_check::handler as backup_check_handler;
pub use backup_check_repair::handler as backup_check_repair_handler;
pub use backup_test::handler as backup_test_handler;
pub use getwatchtower::handler as getwatchtower_handler;
pub use logs::handler as logs_handler;
pub use restore::handler as restore_handler;
pub use start::handler as start_handler;
pub use stop::handler as stop_handler;
pub use watchtower::handler as watchtower_handler;

mod watchtower {
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/watchtower")]
    pub async fn handler() -> HttpResponse {
        match DockerController::start_watchtower().await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod getwatchtower {
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(GET "/api/docker/getwatchtower")]
    pub async fn handler() -> HttpResponse {
        match DockerController::start_watchtower().await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod start {
    use crate::configuration::StateConfiguration;
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum::http::Uri;
    use axum::Form;
    use axum_typed_routing::route;
    use serde::Deserialize;
    use tracing::error;

    #[derive(Deserialize)]
    pub struct ApiDockerStartBody {
        pub install_latest_major: Option<String>,
    }

    #[route(POST "/api/docker/start")]
    pub async fn handler(uri: Uri, form: Form<ApiDockerStartBody>) -> HttpResponse {
        let uri = uri.authority().map(
            |authority| format!("{}:{}", authority.host(), authority.port_u16().and_then(
                |port| if port == 8000 {
                    error!("The AIO_URL-port was discovered to be 8000 which is not expected. It is now set to 443 or 80.");
                    None
                } else {
                    Some(port)
                }
            ).unwrap_or_else(|| if uri.scheme_str() == Some("https") { 443 } else { 80 }))
        );
        let mut config = StateConfiguration::instance_mut().await;
        config.aio_url = uri;
        config.was_start_button_clicked = true;
        config.install_latest_major = form.install_latest_major.is_some();
        match DockerController::start_top_container(config, true).await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod backup {
    use crate::configuration::StateConfiguration;
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/backup")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_mut().await;
        match DockerController::start_backup(config).await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod stop {
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/stop")]
    pub async fn handler() -> HttpResponse {
        match DockerController::stop_top_container().await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod backup_check {
    use crate::configuration::StateConfiguration;
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/backup-check")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_mut().await;
        match DockerController::check_backup(config).await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod backup_check_repair {
    use crate::configuration::StateConfiguration;
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/backup-check-repair")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_mut().await;
        match DockerController::repair_backup(config).await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod backup_test {
    use crate::configuration::StateConfiguration;
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/backup-test")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_mut().await;
        match DockerController::test_backup(config).await {
            Ok(()) => HttpResponse::CreatedAndRedirect("/"),
            Err(e) => HttpResponse::Error(e.to_string()),
        }
    }
}

mod restore {
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;

    #[route(POST "/api/docker/restore")]
    pub async fn handler() -> HttpResponse {
        panic!("Not implemented")
        // TODO AIO\Controller\DockerController::class . ':StartBackupContainerRestore'
        //     public function StartBackupContainerRestore(Requet $request, Response $response, array $args) : Response {
        //         $config = $this->configurationManager->GetConfig();
        //         $config['backup-mode'] = 'restore';
        //         $config['selected-restore-time'] = $request->getParsedBody()['selected_restore_time'] ?? '';
        //         $this->configurationManager->WriteConfig($config);
        //         $id = self::TOP_CONTAINER;
        //         $this->PerformRecursiveContainerStop($id);
        //         $id = 'nextcloud-aio-borgbackup';
        //         $this->PerformRecursiveContainerStart($id);
        //         return $response->withStatus(201)->withHeader('Location', '/');
        //     }
    }
}
mod logs {
    use crate::docker_client;
    use crate::routes::HttpResponse;
    use axum::extract::Query;
    use axum_typed_routing::route;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct DockerLogsQuery {
        id: String,
    }

    #[route(GET "/api/docker/logs")]
    pub async fn handler(query: Query<DockerLogsQuery>) -> HttpResponse {
        if query.id.starts_with("nextcloud-aio-") {
            HttpResponse::InlineText(docker_client::get_logs(query.id.as_str()))
        } else {
            HttpResponse::NotFound
        }
    }
}
