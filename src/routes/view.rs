pub use containers::handler as containers_handler;
pub use index::handler as index_handler;
pub use login::handler as login_handler;
pub use setup::handler as setup_handler;

mod login {
    use crate::container::controller::DockerController;
    use crate::routes::HttpResponse;
    use askama::Template;
    use axum_typed_routing::route;

    #[derive(Template)]
    #[template(path = "login.askama.html")]
    pub struct LoginTemplate {
        pub is_login_allowed: bool,
    }

    #[route(GET "/login")]
    pub async fn handler() -> HttpResponse {
        HttpResponse::html_template(LoginTemplate {
            is_login_allowed: DockerController::is_login_allowed().await.unwrap(),
        })
    }
}

mod index {
    use crate::auth::{can_be_installed, is_authenticated};
    use crate::routes::HttpResponse;
    use axum_typed_routing::route;
    use tower_sessions::Session;

    #[route(GET "/")]
    pub async fn handler(session: Session) -> HttpResponse {
        if can_be_installed() {
            HttpResponse::TemporaryRedirect("/setup")
        } else {
            match is_authenticated(&session).await {
                Ok(true) => HttpResponse::TemporaryRedirect("/containers"),
                Ok(false) => HttpResponse::TemporaryRedirect("/login"),
                Err(e) => HttpResponse::Error(e.to_string()),
            }
        }
    }
}

mod setup {
    use crate::auth::setup_password;
    use crate::configuration::StateConfiguration;
    use crate::routes::HttpResponse;
    use askama::Template;
    use axum_typed_routing::route;

    #[derive(Template)]
    #[template(path = "setup.askama.html")]
    pub struct SetupTemplate {
        pub password: String,
    }

    #[derive(Template)]
    #[template(path = "already-installed.askama.html")]
    pub struct AlreadyInstalledTemplate {}

    #[route(GET "/setup")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_mut().await;
        if let Some(password) = setup_password(config).await {
            HttpResponse::html_template(SetupTemplate { password })
        } else {
            HttpResponse::html_template(AlreadyInstalledTemplate {})
        }
    }
}

mod containers {
    use crate::configuration::StateConfiguration;
    use crate::container::models::Container;
    use crate::routes::HttpResponse;
    use askama::Template;
    use axum_typed_routing::route;

    #[derive(Template)]
    #[template(path = "containers.askama.html")]
    pub struct ContainersTemplate {
        pub domain: String,
        pub apache_port: u16,
        pub borg_backup_host_location: String,
        pub nextcloud_password: String,
        pub containers: Vec<Container>,
        pub borgbackup_password: String,
        pub is_mastercontainer_update_available: bool,
        pub has_backup_run_once: bool,
        pub is_backup_container_running: bool,
        pub backup_exit_code: i32,
        pub is_instance_restore_attempt: bool,
        pub borg_backup_mode: String,
        pub was_start_button_clicked: bool,
        pub has_update_available: bool,
        pub last_backup_time: String,
        pub backup_times: Vec<String>,
        pub current_channel: String,
        pub is_x64_platform: bool,
        pub is_clamav_enabled: bool,
        pub is_onlyoffice_enabled: bool,
        pub is_collabora_enabled: bool,
        pub is_talk_enabled: bool,
        pub borg_restore_password: String,
        pub daily_backup_time: String,
        pub is_daily_backup_running: bool,
        pub timezone: String,
        pub skip_domain_validation: bool,
        pub talk_port: u16,
        pub collabora_dictionaries: Vec<String>,
        pub automatic_updates: bool,
        pub is_backup_section_enabled: bool,
        pub is_imaginary_enabled: bool,
        pub is_fulltextsearch_enabled: bool,
        pub additional_backup_directories: String,
        pub nextcloud_datadir: String,
        pub nextcloud_mount: String,
        pub nextcloud_upload_limit: String,
        pub nextcloud_max_time: u16,
        pub is_dri_device_enabled: bool,
        pub is_talk_recording_enabled: bool,
        pub is_docker_socket_proxy_enabled: bool,
        pub is_whiteboard_enabled: bool,
    }

    #[route(GET "/containers")]
    pub async fn handler() -> HttpResponse {
        let config = StateConfiguration::instance_ref().await;
        HttpResponse::html_template(ContainersTemplate {
            domain: config.domain.clone().unwrap(),
            apache_port: config.apache_port,
            borg_backup_host_location: config.borg_backup_host_location.clone().unwrap(),
            nextcloud_password: config.nextcloud_password.clone().unwrap(),
            containers: vec![],                         //TODO
            borgbackup_password: "".to_string(),        //TODO
            is_mastercontainer_update_available: false, //TODO
            has_backup_run_once: false,                 //TODO
            is_backup_container_running: false,         //TODO
            backup_exit_code: 0,                        //TODO
            is_instance_restore_attempt: false,         //TODO
            borg_backup_mode: "".to_string(),           //TODO
            was_start_button_clicked: config.was_start_button_clicked,
            has_update_available: false,      //TODO
            last_backup_time: "".to_string(), //TODO
            backup_times: vec![],             //TODO
            current_channel: "".to_string(),  //TODO
            #[cfg(target_arch = "arm")]
            is_x64_platform: false,
            #[cfg(target_arch = "arm")]
            is_clamav_enabled: false,
            #[cfg(not(target_arch = "arm"))]
            is_x64_platform: true,
            #[cfg(not(target_arch = "arm"))]
            is_clamav_enabled: config.is_clamav_enabled,
            is_onlyoffice_enabled: config.is_onlyoffice_enabled,
            is_collabora_enabled: config.is_collabora_enabled,
            is_talk_enabled: config.is_talk_enabled,
            borg_restore_password: "".to_string(), //TODO
            daily_backup_time: "".to_string(),     //TODO
            is_daily_backup_running: false,        //TODO
            timezone: "".to_string(),              //TODO
            skip_domain_validation: false,         //TODO
            talk_port: config.talk_port,
            collabora_dictionaries: vec![],   //TODO
            automatic_updates: false,         //TODO
            is_backup_section_enabled: false, //TODO
            is_imaginary_enabled: config.is_imaginary_enabled,
            is_fulltextsearch_enabled: config.is_fulltextsearch_enabled,
            additional_backup_directories: "".to_string(),
            nextcloud_datadir: config.nextcloud_datadir.clone(),
            nextcloud_mount: config.nextcloud_mount.clone().unwrap(),
            nextcloud_upload_limit: config.nextcloud_upload_limit.clone(),
            nextcloud_max_time: config.nextcloud_max_time,
            is_dri_device_enabled: config.nextcloud_enable_dri_device,
            is_talk_recording_enabled: config.is_talk_recording_enabled,
            is_docker_socket_proxy_enabled: config.is_docker_socket_proxy_enabled,
            is_whiteboard_enabled: config.is_whiteboard_enabled,
        })

        //TODO $view = Twig::fromRequest($request);
        //     $view->addExtension(new \AIO\Twig\ClassExtension());
        //     $configurationManager = $container->get(\AIO\Data\ConfigurationManager::class);
        //     $dockerActionManger = $container->get(\AIO\Docker\DockerActionManager::class);
        //     $dockerController = $container->get(\AIO\Controller\DockerController::class);
        //     $dockerActionManger->ConnectMasterContainerToNetwork();
        //     $dockerController->StartDomaincheckContainer();
        //     return $view->render($response, 'containers.twig', [
        //         'domain' => $configurationManager->GetDomain(),
        //         'apache_port' => $configurationManager->GetApachePort(),
        //         'borg_backup_host_location' => $configurationManager->GetBorgBackupHostLocation(),
        //         'nextcloud_password' => $configurationManager->GetAndGenerateSecret('NEXTCLOUD_PASSWORD'),
        //         'containers' => (new \AIO\ContainerDefinitionFetcher($container->get(\AIO\Data\ConfigurationManager::class), $container))->FetchDefinition(),
        //         'borgbackup_password' => $configurationManager->GetAndGenerateSecret('BORGBACKUP_PASSWORD'),
        //         'is_mastercontainer_update_available' => $dockerActionManger->IsMastercontainerUpdateAvailable(),
        //         'has_backup_run_once' => $configurationManager->hasBackupRunOnce(),
        //         'is_backup_container_running' => $dockerActionManger->isBackupContainerRunning(),
        //         'backup_exit_code' => $dockerActionManger->GetBackupcontainerExitCode(),
        //         'is_instance_restore_attempt' => $configurationManager->isInstanceRestoreAttempt(),
        //         'borg_backup_mode' => $configurationManager->GetBorgBackupMode(),
        //         'was_start_button_clicked' => $configurationManager->wasStartButtonClicked(),
        //         'has_update_available' => $dockerActionManger->isAnyUpdateAvailable(),
        //         'last_backup_time' => $configurationManager->GetLastBackupTime(),
        //         'backup_times' => $configurationManager->GetBackupTimes(),
        //         'current_channel' => $dockerActionManger->GetCurrentChannel(),
        //         'is_x64_platform' => $configurationManager->isx64Platform(),
        //         'is_clamav_enabled' => $configurationManager->isClamavEnabled(),
        //         'is_onlyoffice_enabled' => $configurationManager->isOnlyofficeEnabled(),
        //         'is_collabora_enabled' => $configurationManager->isCollaboraEnabled(),
        //         'is_talk_enabled' => $configurationManager->isTalkEnabled(),
        //         'borg_restore_password' => $configurationManager->GetBorgRestorePassword(),
        //         'daily_backup_time' => $configurationManager->GetDailyBackupTime(),
        //         'is_daily_backup_running' => $configurationManager->isDailyBackupRunning(),
        //         'timezone' => $configurationManager->GetTimezone(),
        //         'skip_domain_validation' => $configurationManager->shouldDomainValidationBeSkipped(),
        //         'talk_port' => $configurationManager->GetTalkPort(),
        //         'collabora_dictionaries' => $configurationManager->GetCollaboraDictionaries(),
        //         'automatic_updates' => $configurationManager->areAutomaticUpdatesEnabled(),
        //         'is_backup_section_enabled' => $configurationManager->isBackupSectionEnabled(),
        //         'is_imaginary_enabled' => $configurationManager->isImaginaryEnabled(),
        //         'is_fulltextsearch_enabled' => $configurationManager->isFulltextsearchEnabled(),
        //         'additional_backup_directories' => $configurationManager->GetAdditionalBackupDirectoriesString(),
        //         'nextcloud_datadir' => $configurationManager->GetNextcloudDatadirMount(),
        //         'nextcloud_mount' => $configurationManager->GetNextcloudMount(),
        //         'nextcloud_upload_limit' => $configurationManager->GetNextcloudUploadLimit(),
        //         'nextcloud_max_time' => $configurationManager->GetNextcloudMaxTime(),
        //         'nextcloud_memory_limit' => $configurationManager->GetNextcloudMemoryLimit(),
        //         'is_dri_device_enabled' => $configurationManager->isDriDeviceEnabled(),
        //         'is_talk_recording_enabled' => $configurationManager->isTalkRecordingEnabled(),
        //         'is_docker_socket_proxy_enabled' => $configurationManager->isDockerSocketProxyEnabled(),
        //         'is_whiteboard_enabled' => $configurationManager->isWhiteboardEnabled(),
        //     ]);
    }
}
