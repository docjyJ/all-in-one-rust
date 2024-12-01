use crate::configuration::{BackupMode, MutConfiguration, StateConfiguration};
use crate::container::definition::ContainerDefinition;
use crate::docker_client::{
    connect_container_to_network, create_container, get_databasecontainer_exit_code, DockerClient,
    Result,
};
use tracing::error;

const TOP_CONTAINER: &str = "nextcloud-aio-apache";
const BACKUP_CONTAINER: &str = "nextcloud-aio-borgbackup";
const DOMAINCHECK_CONTAINER: &str = "nextcloud-aio-domaincheck";
const DATABASE_CONTAINER: &str = "nextcloud-aio-database";
const WATCHTOWER_CONTAINER: &str = "nextcloud-aio-watchtower";

#[derive(Eq, PartialEq)]
pub enum ContainerState {
    ImageDoesNotExist,
    NotRestarting,
    Restarting,
    Running,
    Starting,
    Stopped,
}
#[derive(Eq, PartialEq)]
pub enum VersionState {
    Different,
    Equal,
}

pub struct DockerController {
    client: DockerClient,
    definition: &'static ContainerDefinition,
}

impl DockerController {
    async fn new() -> Result<Self> {
        Ok(Self {
            client: DockerClient::new()?,
            definition: ContainerDefinition::instance().await,
        })
    }

    async fn recursive_start(&self, id: &str, pull_image: bool) -> Result<()> {
        for c in self.definition.dependency_list(id).iter().rev() {
            match self.client.container_get_running_state(&c).await? {
                ContainerState::ImageDoesNotExist => {
                    error!("Not starting {} because it does not exist.", c.identifier)
                }
                ContainerState::Stopped => {
                    if id == DATABASE_CONTAINER && get_databasecontainer_exit_code() > 0 {
                        error!("Not pulling the latest database image because the container was not correctly shut down.");
                    } else {
                        if pull_image && !self.client.repository_is_reachable(&c).await {
                            error!("Not pulling the image for the {} container because docker hub does not seem to be reachable.", c.container_name);
                        } else {
                            self.client.container_delete(&c).await?;
                            self.client.volumes_create(c.volumes.as_slice()).await?;
                            if pull_image {
                                self.client.image_pull(&c).await?;
                            }
                            create_container(&c);
                            self.client.container_start(&c.identifier).await?;
                            connect_container_to_network(&c);
                        }
                    }
                }
                _ => error!(
                    "Not starting {} because it was already started.",
                    c.identifier
                ),
            }
        }
        Ok(())
        //     private function PerformRecursiveContainerStart(string $id, bool $pullImage = true) : void {
        //         $container = $this->containerDefinitionFetcher->GetContainerById($id);
        //         foreach($container->GetDependsOn() as $dependency) {
        //             $this->PerformRecursiveContainerStart($dependency, $pullImage);
        //         }
        //         if ($container->GetRunningState() === ContainerState::Running) {
        //             error_log('Not starting ' . $id . ' because it was already started.');
        //             return;
        //         }
        //         if ($id === 'nextcloud-aio-database') {
        //             if ($this->dockerActionManager->GetDatabasecontainerExitCode() > 0) {
        //                 $pullImage = false;
        //                 error_log('Not pulling the latest database image because the container was not correctly shut down.');
        //             }
        //         }
        //         if ($pullImage) {
        //             if (!$this->dockerActionManager->isDockerHubReachable($container)) {
        //                 $pullImage = false;
        //                 error_log('Not pulling the image for the ' . $container->GetContainerName() . ' container because docker hub does not seem to be reachable.');
        //             }
        //         }
        //         $this->dockerActionManager->DeleteContainer($container);
        //         $this->dockerActionManager->CreateVolumes($container);
        //         if ($pullImage) {
        //             $this->dockerActionManager->PullImage($container);
        //         }
        //         $this->dockerActionManager->CreateContainer($container);
        //         $this->dockerActionManager->StartContainer($container);
        //         $this->dockerActionManager->ConnectContainerToNetwork($container);
        //     }
    }

    async fn recursive_stop(&self, id: &str) -> Result<()> {
        for c in self.definition.dependency_list(id) {
            match self.client.container_get_running_state(c).await? {
                ContainerState::ImageDoesNotExist => {
                    error!("Not stopping {} because it does not exist.", c.identifier)
                }
                ContainerState::Stopped => error!(
                    "Not stopping {} because it was already stopped.",
                    c.identifier
                ),
                _ => self.client.container_stop(c).await?,
            }
        }
        Ok(())
    }

    async fn recursive_stop_and_start(
        &self,
        id_stop: &str,
        id_start: &str,
        pull_image: bool,
    ) -> Result<()> {
        self.recursive_stop(id_stop).await?;
        self.recursive_start(id_start, pull_image).await
    }

    async fn start_domaincheck_container(&self, config: &MutConfiguration) -> Result<()> {
        // Don't start if domain is already set or start button was clicked
        if config.domain.is_some() || config.was_start_button_clicked {
            return Ok(());
        }

        let domaincheck_container = ContainerDefinition::instance()
            .await
            .get(DOMAINCHECK_CONTAINER)
            .unwrap();

        let apache_container = ContainerDefinition::instance()
            .await
            .get(TOP_CONTAINER)
            .unwrap();

        // If the apache container is running, return early
        if self
            .client
            .container_get_running_state(&apache_container)
            .await?
            == ContainerState::Running
        {
            return Ok(());
        }

        // If the domaincheck container is running, check if it was started recently
        // TODO if get_container_running_state(&domaincheck_container).await? == ContainerState::Running {
        //     if let Some(domaincheck_was_started) = state.cache().await.get("domaincheckWasStarted").await {
        //         if domaincheck_was_started == "1" {
        //             return Ok(());
        //         }
        //     }
        // }

        // Stop the domaincheck container
        self.recursive_stop(DOMAINCHECK_CONTAINER).await?;

        // Start the domaincheck container recursively
        if let Err(e) = self.recursive_start(DOMAINCHECK_CONTAINER, true).await {
            error!("Could not start domaincheck container: {}", e);
        }

        // Cache the start status of the domaincheck container
        // TODO state.cache().await.set("domaincheckWasStarted", "1", 600).await;

        Ok(())
        //     public function StartDomaincheckContainer() : void
        //     {
        //         # Don't start if domain is already set
        //         if ($this->configurationManager->GetDomain() !== '' || $this->configurationManager->wasStartButtonClicked()) {
        //             return;
        //         }
        //         $id = 'nextcloud-aio-domaincheck';
        //         $cacheKey = 'domaincheckWasStarted';
        //         $domaincheckContainer = $this->containerDefinitionFetcher->GetContainerById($id);
        //         $apacheContainer = $this->containerDefinitionFetcher->GetContainerById(self::TOP_CONTAINER);
        //         if ($apacheContainer->GetRunningState() === ContainerState::Running) {
        //             return;
        //         } elseif ($domaincheckContainer->GetRunningState() === ContainerState::Running) {
        //             $domaincheckWasStarted = apcu_fetch($cacheKey);
        //             if($domaincheckWasStarted !== false && is_string($domaincheckWasStarted)) {
        //                 return;
        //             }
        //         }
        //         $this->StopDomaincheckContainer();
        //         try {
        //             $this->PerformRecursiveContainerStart($id);
        //         } catch (\Exception $e) {
        //             error_log('Could not start domaincheck container: ' . $e->getMessage());
        //         }
        //         apcu_add($cacheKey, '1', 600);
        //     }
    }

    async fn is_backup_container_running(&self) -> Result<bool> {
        self.is_container_running(BACKUP_CONTAINER).await
    }

    async fn is_container_running(&self, id: &str) -> Result<bool> {
        if let Some(c) = self.definition.get(id) {
            Ok(self.client.container_get_running_state(c).await? == ContainerState::Running)
        } else {
            Ok(false)
        }
    }

    pub async fn check_backup(mut config: MutConfiguration) -> Result<()> {
        config.backup_mode = BackupMode::Check;
        config.commit();
        Self::new()
            .await?
            .recursive_start(BACKUP_CONTAINER, true)
            .await
    }

    pub async fn start_watchtower() -> Result<()> {
        Self::new()
            .await?
            .recursive_start(WATCHTOWER_CONTAINER, true)
            .await
    }

    pub async fn start_top_container(mut config: MutConfiguration, pull_image: bool) -> Result<()> {
        config.aio_token = Some(hex::encode(rand::random::<[u8; 24]>()));
        config.commit();
        Self::new()
            .await?
            .recursive_stop_and_start(DOMAINCHECK_CONTAINER, TOP_CONTAINER, pull_image)
            .await
    }

    pub async fn start_backup(mut config: MutConfiguration) -> Result<()> {
        config.backup_mode = BackupMode::Backup;
        config.commit();
        Self::new()
            .await?
            .recursive_stop_and_start(TOP_CONTAINER, BACKUP_CONTAINER, true)
            .await
        //     public function startBackup() : void {
        //         $config = $this->configurationManager->GetConfig();
        //         $config['backup-mode'] = 'backup';
        //         $this->configurationManager->WriteConfig($config);
        //         $id = self::TOP_CONTAINER;
        //         $this->PerformRecursiveContainerStop($id);
        //         $id = 'nextcloud-aio-borgbackup';
        //         $this->PerformRecursiveContainerStart($id);
        //     }
    }

    pub async fn stop_top_container() -> Result<()> {
        Self::new().await?.recursive_stop(TOP_CONTAINER).await
    }

    pub async fn is_login_allowed() -> Result<bool> {
        Self::new().await?.is_container_running(TOP_CONTAINER).await
    }

    pub async fn repair_backup(mut config: MutConfiguration) -> Result<()> {
        config.backup_mode = BackupMode::CheckRepair;
        config.commit();
        Self::new()
            .await?
            .recursive_start(BACKUP_CONTAINER, true)
            .await?;
        let mut config = StateConfiguration::instance_mut().await;
        config.backup_mode = BackupMode::Check;
        config.commit();
        Ok(())
    }
    pub async fn test_backup(mut config: MutConfiguration) -> Result<()> {
        config.backup_mode = BackupMode::Test;
        config.commit();
        Self::new()
            .await?
            .recursive_stop_and_start(TOP_CONTAINER, BACKUP_CONTAINER, true)
            .await
    }

    // readonly class ConfigurationController {
    // }

    // pub fn set_config(&self, request: Request, response: Response, args: Vec<Box<()>>) -> Response {
    // panic!("Not implemented"); // TODO
    //     public function SetConfig(Request $request, Response $response, array $args) : Response {
    //         try {
    //             if (isset($request->getParsedBody()['domain'])) {
    //                 $domain = $request->getParsedBody()['domain'] ?? '';
    //                 $this->configurationManager->SetDomain($domain);
    //             }
    //             if (isset($request->getParsedBody()['current-master-password']) || isset($request->getParsedBody()['new-master-password'])) {
    //                 $currentMasterPassword = $request->getParsedBody()['current-master-password'] ?? '';
    //                 $newMasterPassword = $request->getParsedBody()['new-master-password'] ?? '';
    //                 $this->configurationManager->ChangeMasterPassword($currentMasterPassword, $newMasterPassword);
    //             }
    //             if (isset($request->getParsedBody()['borg_backup_host_location'])) {
    //                 $location = $request->getParsedBody()['borg_backup_host_location'] ?? '';
    //                 $this->configurationManager->SetBorgBackupHostLocation($location);
    //             }
    //             if (isset($request->getParsedBody()['borg_restore_host_location']) || isset($request->getParsedBody()['borg_restore_password'])) {
    //                 $restoreLocation = $request->getParsedBody()['borg_restore_host_location'] ?? '';
    //                 $borgPassword = $request->getParsedBody()['borg_restore_password'] ?? '';
    //                 $this->configurationManager->SetBorgRestoreHostLocationAndPassword($restoreLocation, $borgPassword);
    //             }
    //             if (isset($request->getParsedBody()['daily_backup_time'])) {
    //                 if (isset($request->getParsedBody()['automatic_updates'])) {
    //                     $enableAutomaticUpdates = true;
    //                 } else {
    //                     $enableAutomaticUpdates = false;
    //                 }
    //                 if (isset($request->getParsedBody()['success_notification'])) {
    //                     $successNotification = true;
    //                 } else {
    //                     $successNotification = false;
    //                 }
    //                 $dailyBackupTime = $request->getParsedBody()['daily_backup_time'] ?? '';
    //                 $this->configurationManager->SetDailyBackupTime($dailyBackupTime, $enableAutomaticUpdates, $successNotification);
    //             }
    //             if (isset($request->getParsedBody()['delete_daily_backup_time'])) {
    //                 $this->configurationManager->DeleteDailyBackupTime();
    //             }
    //             if (isset($request->getParsedBody()['additional_backup_directories'])) {
    //                 $additionalBackupDirectories = $request->getParsedBody()['additional_backup_directories'] ?? '';
    //                 $this->configurationManager->SetAdditionalBackupDirectories($additionalBackupDirectories);
    //             }
    //             if (isset($request->getParsedBody()['delete_timezone'])) {
    //                 $this->configurationManager->DeleteTimezone();
    //             }
    //             if (isset($request->getParsedBody()['timezone'])) {
    //                 $timezone = $request->getParsedBody()['timezone'] ?? '';
    //                 $this->configurationManager->SetTimezone($timezone);
    //             }
    //             if (isset($request->getParsedBody()['options-form'])) {
    //                 if (isset($request->getParsedBody()['collabora']) && isset($request->getParsedBody()['onlyoffice'])) {
    //                     throw new InvalidSettingConfigurationException("Collabora and Onlyoffice are not allowed to be enabled at the same time!");
    //                 }
    //                 if (isset($request->getParsedBody()['clamav'])) {
    //                     $this->configurationManager->SetClamavEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetClamavEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['onlyoffice'])) {
    //                     $this->configurationManager->SetOnlyofficeEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetOnlyofficeEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['collabora'])) {
    //                     $this->configurationManager->SetCollaboraEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetCollaboraEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['talk'])) {
    //                     $this->configurationManager->SetTalkEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetTalkEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['talk-recording'])) {
    //                     $this->configurationManager->SetTalkRecordingEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetTalkRecordingEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['imaginary'])) {
    //                     $this->configurationManager->SetImaginaryEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetImaginaryEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['fulltextsearch'])) {
    //                     $this->configurationManager->SetFulltextsearchEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetFulltextsearchEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['docker-socket-proxy'])) {
    //                     $this->configurationManager->SetDockerSocketProxyEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetDockerSocketProxyEnabledState(0);
    //                 }
    //                 if (isset($request->getParsedBody()['whiteboard'])) {
    //                     $this->configurationManager->SetWhiteboardEnabledState(1);
    //                 } else {
    //                     $this->configurationManager->SetWhiteboardEnabledState(0);
    //                 }
    //             }
    //             if (isset($request->getParsedBody()['delete_collabora_dictionaries'])) {
    //                 $this->configurationManager->DeleteCollaboraDictionaries();
    //             }
    //             if (isset($request->getParsedBody()['collabora_dictionaries'])) {
    //                 $collaboraDictionaries = $request->getParsedBody()['collabora_dictionaries'] ?? '';
    //                 $this->configurationManager->SetCollaboraDictionaries($collaboraDictionaries);
    //             }
    //             if (isset($request->getParsedBody()['delete_borg_backup_host_location'])) {
    //                 $this->configurationManager->DeleteBorgBackupHostLocation();
    //             }
    //             return $response->withStatus(201)->withHeader('Location', '/');
    //         } catch (InvalidSettingConfigurationException $ex) {
    //             $response->getBody()->write($ex->getMessage());
    //             return $response->withStatus(422);
    //         }
    //     }
    // }
}
