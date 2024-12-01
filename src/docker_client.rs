use crate::container::controller::ContainerState;
use crate::container::controller::VersionState;
use crate::container::models::{Container, ContainerVolume};
use axum::http::header;
use bollard_stubs::models::{ContainerInspectResponse, ImageInspect, VolumeCreateOptions};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use tracing::log::warn;

#[derive(Serialize, Deserialize)]
struct DockerHubToken {
    token: String,
    access_token: String,
    expires_in: u32,
    issued_at: OffsetDateTime,
}

impl DockerHubToken {
    fn bearer(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

const BASE_URL: &str = "http://localhost/v1.47/";

pub(crate) type Result<T> = std::result::Result<T, reqwest::Error>;

pub struct DockerClient {
    client: Client,
}

impl DockerClient {
    pub fn new() -> Result<Self> {
        Client::builder().build().map(|client| Self { client })
    }

    pub async fn container_delete(&self, c: &Container) -> Result<()> {
        match self
            .client
            .delete(format!("{BASE_URL}/containers/{}", c.identifier))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match e.status() {
                Some(StatusCode::NOT_FOUND) => Ok(()),
                _ => Err(e),
            },
        }
        //     public function DeleteContainer(Container $container) : void {
        //         $url = $this->BuildApiUrl(sprintf('containers/%s?v=true', urlencode($container->GetIdentifier())));
        //         try {
        //             $this->guzzleClient->delete($url);
        //         } catch (RequestException $e) {
        //             if ($e->getCode() !== 404) {
        //                 throw $e;
        //             }
        //         }
        //     }
    }

    pub async fn container_start(&self, id: &str) -> Result<()> {
        match self
            .client
            .post(format!("{BASE_URL}/containers/{id}/start"))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
        //     public function StartContainer(Container $container) : void {
        //         $url = $this->BuildApiUrl(sprintf('containers/%s/start', urlencode($container->GetIdentifier())));
        //         try {
        //             $this->guzzleClient->post($url);
        //         } catch (RequestException $e) {
        //             throw new \Exception("Could not start container " . $container->GetIdentifier() . ": " . $e->getMessage());
        //         }
        //     }
    }

    pub async fn container_stop(&self, c: &Container) -> Result<()> {
        match self
            .client
            .post(format!(
                "{BASE_URL}/containers/{}/stop?t={}",
                c.identifier, c.max_shutdown_time
            ))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => match e.status() {
                Some(StatusCode::NOT_FOUND) | Some(StatusCode::NOT_MODIFIED) => Ok(()),
                _ => Err(e),
            },
        }
        //     public function StopContainer(Container $container) : void {
        //         $url = $this->BuildApiUrl(sprintf('containers/%s/stop?t=%s', urlencode($container->GetIdentifier()), $container->GetMaxShutdownTime()));
        //         try {
        //             $this->guzzleClient->post($url);
        //         } catch (RequestException $e) {
        //             if ($e->getCode() !== 404 && $e->getCode() !== 304) {
        //                 throw $e;
        //             }
        //         }
        //     }
    }

    pub async fn container_get_running_state(&self, c: &Container) -> Result<ContainerState> {
        match self
            .client
            .get(format!("{BASE_URL}/containers/{}/json", c.identifier))
            .send()
            .await
        {
            Ok(r) => match r.json::<ContainerInspectResponse>().await {
                Ok(r) => Ok(if r.state.and_then(|s| s.running).unwrap_or(false) {
                    ContainerState::Running
                } else {
                    ContainerState::Stopped
                }),
                Err(e) => Err(e),
            },
            Err(e) => match e.status() {
                Some(StatusCode::NOT_FOUND) => Ok(ContainerState::ImageDoesNotExist),
                _ => Err(e),
            },
        }
        // public function GetContainerRunningState(Container $container) : ContainerState
        // {
        //     $url = $this->BuildApiUrl(sprintf('containers/%s/json', urlencode($container->GetIdentifier())));
        //     try {
        //         $response = $this->guzzleClient->get($url);
        //     } catch (RequestException $e) {
        //         if ($e->getCode() === 404) {
        //             return ContainerState::ImageDoesNotExist;
        //         }
        //         throw $e;
        //     }
        //     $responseBody = json_decode((string)$response->getBody(), true);
        //     if ($responseBody['State']['Running'] === true) {
        //         return ContainerState::Running;
        //     } else {
        //         return ContainerState::Stopped;
        //     }
        // }
    }

    pub async fn image_pull(&self, c: &Container) -> Result<()> {
        let id = self.build_image_name(c).await;
        let exist = self
            .client
            .get(format!("{BASE_URL}/images/{id}/json"))
            .send()
            .await
            .is_ok();

        match self
            .client
            .post(format!("{BASE_URL}/images/create?fromImage={id}"))
            .send()
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                if exist {
                    Ok(())
                } else {
                    Err(e)
                }
            }
        }
        // public function PullImage(Container $container) : void
        // {
        //     $imageName = $this->BuildImageName($container);
        //     $encodedImageName = urlencode($imageName);
        //     $url = $this->BuildApiUrl(sprintf('images/create?fromImage=%s', $encodedImageName));
        //     $imageIsThere = true;
        //     try {
        //         $imageUrl = $this->BuildApiUrl(sprintf('images/%s/json', $encodedImageName));
        //         $this->guzzleClient->get($imageUrl)->getBody()->getContents();
        //     } catch (\Throwable $e) {
        //     $imageIsThere = false;
        // }
        //     try {
        //         $this->guzzleClient->post($url);
        //     } catch (RequestException $e) {
        //     if ($imageIsThere === false) {
        //         throw new \Exception("Could not pull image " . $imageName . ". Please run 'sudo docker exec -it nextcloud-aio-mastercontainer docker pull " . $imageName . "' in order to find out why it failed.");
        //     }
        // }
        // }
    }

    pub async fn volumes_create(&self, volumes: &[ContainerVolume]) -> Result<()> {
        let url = format!("{BASE_URL}/volumes/create");
        for v in volumes {
            if v.name == "nextcloud_aio_nextcloud_datadir"
                || v.name == "nextcloud_aio_backupdir"
                || v.name.chars().next() == Some('/')
            {
                continue;
            }
            self.client
                .post(&url)
                .json(&VolumeCreateOptions {
                    name: Some(v.name.clone()),
                    ..Default::default()
                })
                .send()
                .await?;
        }
        Ok(())
        //     public function CreateVolumes(Container $container): void
        //     {
        //         $url = $this->BuildApiUrl('volumes/create');
        //         foreach($container->GetVolumes()->GetVolumes() as $volume) {
        //             $forbiddenChars = [
        //                 '/',
        //             ];
        //
        //             if ($volume->name === 'nextcloud_aio_nextcloud_datadir' || $volume->name === 'nextcloud_aio_backupdir') {
        //                 return;
        //             }
        //
        //             $firstChar = substr($volume->name, 0, 1);
        //             if(!in_array($firstChar, $forbiddenChars)) {
        //                 $this->guzzleClient->request(
        //                     'POST',
        //                     $url,
        //                     [
        //                         'json' => [
        //                             'name' => $volume->name,
        //                         ],
        //                     ]
        //                 );
        //             }
        //         }
        //     }
    }

    pub async fn repository_is_reachable(&self, c: &Container) -> bool {
        let tag = if c.image_tag == "%AIO_CHANNEL%" {
            Some(self.get_current_channel().await.unwrap())
        } else {
            None
        };

        self.get_latest_digest_of_tag(
            c.container_name.as_str(),
            tag.as_ref().unwrap_or(&c.image_tag),
        )
        .await
        .is_ok()
        //     public function isDockerHubReachable(Container $container) : bool {
        //         $tag = $container->GetImageTag();
        //         if ($tag === '%AIO_CHANNEL%') {
        //             $tag = $this->GetCurrentChannel();
        //         }
        //         $remoteDigest = $this->dockerHubManager->GetLatestDigestOfTag($container->GetContainerName(), $tag);
        //         if ($remoteDigest === null) {
        //             return false;
        //         } else {
        //             return true;
        //         }
        //     }
    }

    // TODO Refactor

    async fn build_image_name(&self, c: &Container) -> String {
        if c.image_tag == "%AIO_CHANNEL%" {
            format!(
                "{}:{}",
                c.container_name,
                self.get_current_channel().await.unwrap()
            )
        } else {
            format!("{}:{}", c.container_name, c.image_tag)
        }
    }

    async fn get_created_time_of_nextcloud_image(&self) -> Option<OffsetDateTime> {
        match self
            .client
            .get("https://hub.docker.com/v2/repositories/nextcloud/aio-nextcloud/tags/")
            .send()
            .await
        {
            Ok(r) => match r.json::<ImageInspect>().await {
                Ok(i) => i.created,
                Err(e) => {
                    warn!("Error: {}", e);
                    None
                }
            },
            Err(e) => {
                warn!("Error: {}", e);
                None
            }
        }
        //     private function GetCreatedTimeOfNextcloudImage() : ?string {
        //         $imageName = 'nextcloud/aio-nextcloud' . ':' . $this->GetCurrentChannel();
        //         try {
        //             $imageUrl = $this->BuildApiUrl(sprintf('images/%s/json', $imageName));
        //             $imageOutput = json_decode($this->guzzleClient->get($imageUrl)->getBody()->getContents(), true);
        //
        //             if (!isset($imageOutput['Created'])) {
        //                 error_log('Created is not set of image ' . $imageName);
        //                 return null;
        //             }
        //
        //             return str_replace('T', ' ', (string)$imageOutput['Created']);
        //         } catch (\Exception $e) {
        //             return null;
        //         }
        //     }
    }

    pub async fn is_nextcloud_image_outdated(&self) -> bool {
        if let Some(created_time) = self.get_created_time_of_nextcloud_image().await {
            OffsetDateTime::now_utc() - created_time > Duration::days(90)
        } else {
            false
        }
    }

    pub async fn get_latest_digest_of_tag(&self, name: &str, tag: &str) -> Result<String> {
        // TODO Cache
        let token = self
            .client
            .get(format!(
                "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{}:pull",
                name
            ))
            .send()
            .await?
            .json::<DockerHubToken>()
            .await?;

        let manifest_url = format!("https://registry-1.docker.io/v2/{}/manifests/{}", name, tag);
        let latest_digest = self.client.get(&manifest_url)
            .header(header::ACCEPT, "application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.docker.distribution.manifest.v2+json")
            .header(header::AUTHORIZATION, token.bearer())
            .send().await?.headers().get("docker-content-digest").unwrap().to_str().unwrap().to_string();

        Ok(latest_digest)

        //     public function GetLatestDigestOfTag(string $name, string $tag) : ?string {
        //         $cacheKey = 'dockerhub-manifest-' . $name . $tag;
        //         $cachedVersion = apcu_fetch($cacheKey);
        //         if($cachedVersion !== false && is_string($cachedVersion)) {
        //             return $cachedVersion;
        //         }
        //         try {
        //             $authTokenRequest = $this->guzzleClient->request(
        //                 'GET',
        //                 'https:
        //             );
        //             $body = $authTokenRequest->getBody()->getContents();
        //             $decodedBody = json_decode($body, true);
        //             if(isset($decodedBody['token'])) {
        //                 $authToken = $decodedBody['token'];
        //                 $manifestRequest = $this->guzzleClient->request(
        //                     'HEAD',
        //                     'https:
        //                     [
        //                         'headers' => [
        //                             'Accept' => 'application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.docker.distribution.manifest.v2+json',
        //                             'Authorization' => 'Bearer ' . $authToken,
        //                         ],
        //                     ]
        //                 );
        //                 $responseHeaders = $manifestRequest->getHeader('docker-content-digest');
        //                 if(count($responseHeaders) === 1) {
        //                     $latestVersion = $responseHeaders[0];
        //                     apcu_add($cacheKey, $latestVersion, 600);
        //                     return $latestVersion;
        //                 }
        //             }
        //             error_log('Could not get digest of container ' . $name . ':' . $tag);
        //             return null;
        //         } catch (\Exception $e) {
        //             error_log('Could not get digest of container ' . $name . ':' . $tag . ' ' . $e->getMessage());
        //             return null;
        //         }
        //     }
    }

    pub async fn get_current_channel(&self) -> Result<String> {
        // TODO Cache the channel name
        let tag = self
            .client
            .get(format!(
                "{BASE_URL}/containers/nextcloud-aio-mastercontainer/json"
            ))
            .send()
            .await?
            .json::<ContainerInspectResponse>()
            .await?
            .config
            .and_then(|c| c.image)
            .and_then(|i| {
                let mut it = i.split(":");
                if it.next().is_some() {
                    if let Some(t) = it.next() {
                        if it.next().is_none() {
                            Some(t.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
        Ok(
            tag.unwrap_or_else(|| {
                warn!("No tag was found when getting the current channel. You probably did not follow the documentation correctly. Changing the channel to the default 'latest'.");
                "latest".to_string()
            })
        )

        //     public function GetCurrentChannel() : string {
        //         $cacheKey = 'aio-ChannelName';
        //         $channelName = apcu_fetch($cacheKey);
        //         if($channelName !== false && is_string($channelName)) {
        //             return $channelName;
        //         }
        //         $containerName = 'nextcloud-aio-mastercontainer';
        //         $url = $this->BuildApiUrl(sprintf('containers/%s/json', $containerName));
        //         try {
        //             $output = json_decode($this->guzzleClient->get($url)->getBody()->getContents(), true);
        //             $containerChecksum = $output['Image'];
        //             $tagArray = explode(':', $output['Config']['Image']);
        //             $tag = $tagArray[1];
        //             apcu_add($cacheKey, $tag);
        //             /**
        //              * @psalm-suppress TypeDoesNotContainNull
        //              * @psalm-suppress DocblockTypeContradiction
        //              */
        //             if ($tag === null) {
        //                 error_log("No tag was found when getting the current channel. You probably did not follow the documentation correctly. Changing the channel to the default 'latest'.");
        //                 $tag = 'latest';
        //             }
        //             return $tag;
        //         } catch (\Exception $e) {
        //             error_log('Could not get current channel ' . $e->getMessage());
        //         }
        //         return 'latest';
        //     }
    }
}

pub fn get_container_restarting_state(container: Container) -> ContainerState {
    panic!("Not implemented") // TODO
                              //     public function GetContainerRestartingState(Container $container) : ContainerState
                              //     {
                              //         $url = $this->BuildApiUrl(sprintf('containers/%s/json', urlencode($container->GetIdentifier())));
                              //         try {
                              //             $response = $this->guzzleClient->get($url);
                              //         } catch (RequestException $e) {
                              //             if ($e->getCode() === 404) {
                              //                 return ContainerState::ImageDoesNotExist;
                              //             }
                              //             throw $e;
                              //         }
                              //         $responseBody = json_decode((string)$response->getBody(), true);
                              //         if ($responseBody['State']['Restarting'] === true) {
                              //             return ContainerState::Restarting;
                              //         } else {
                              //             return ContainerState::NotRestarting;
                              //         }
                              //     }
}

pub fn get_container_update_state(container: &Container) -> VersionState {
    panic!("Not implemented") // TODO
                              //     public function GetContainerUpdateState(Container $container) : VersionState
                              //     {
                              //         $tag = $container->GetImageTag();
                              //         if ($tag === '%AIO_CHANNEL%') {
                              //             $tag = $this->GetCurrentChannel();
                              //         }
                              //         $runningDigests = $this->GetRepoDigestsOfContainer($container->GetIdentifier());
                              //         if ($runningDigests === null) {
                              //             return VersionState::Different;
                              //         }
                              //         $remoteDigest = $this->dockerHubManager->GetLatestDigestOfTag($container->GetContainerName(), $tag);
                              //         if ($remoteDigest === null) {
                              //             return VersionState::Equal;
                              //         }
                              //         foreach($runningDigests as $runningDigest) {
                              //             if ($runningDigest === $remoteDigest) {
                              //                 return VersionState::Equal;
                              //             }
                              //         }
                              //         return VersionState::Different;
                              //     }
}

pub fn get_container_starting_state(container: Container) -> ContainerState {
    panic!("Not implemented") // TODO
                              //     public function GetContainerStartingState(Container $container) : ContainerState
                              //     {
                              //         $runningState = $this->GetContainerRunningState($container);
                              //         if ($runningState === ContainerState::Stopped || $runningState === ContainerState::ImageDoesNotExist) {
                              //             return $runningState;
                              //         }
                              //         $containerName = $container->GetIdentifier();
                              //         $internalPort = $container->GetInternalPort();
                              //         if($internalPort === '%APACHE_PORT%') {
                              //             $internalPort = $this->configurationManager->GetApachePort();
                              //         } elseif($internalPort === '%TALK_PORT%') {
                              //             $internalPort = $this->configurationManager->GetTalkPort();
                              //         }
                              //         if ($internalPort !== "" && $internalPort !== 'host') {
                              //             $connection = @fsockopen($containerName, (int)$internalPort, $errno, $errstr, 0.2);
                              //             if ($connection) {
                              //                 fclose($connection);
                              //                 return ContainerState::Running;
                              //             } else {
                              //                 return ContainerState::Starting;
                              //             }
                              //         } else {
                              //             return ContainerState::Running;
                              //         }
                              //     }
}

pub fn get_logs(id: &str) -> String {
    panic!("Not implemented") // TODO
                              //     public function GetLogs(string $id) : string
                              //     {
                              //         $url = $this->BuildApiUrl(
                              //             sprintf(
                              //                 'containers/%s/logs?stdout=true&stderr=true&timestamps=true',
                              //                 urlencode($id)
                              //             ));
                              //         $responseBody = (string)$this->guzzleClient->get($url)->getBody();
                              //         $response = "";
                              //         $separator = "\r\n";
                              //         $line = strtok($responseBody, $separator);
                              //         $response = substr($line, 8) . $separator;
                              //         while ($line !== false) {
                              //             $line = strtok($separator);
                              //             $response .= substr($line, 8) . $separator;
                              //         }
                              //         return $response;
                              //     }
}

pub fn create_container(c: &Container) -> () {
    panic!("Not implemented") // TODO
                              //     public function CreateContainer(Container $container) : void {
                              //         $volumes = [];
                              //         foreach ($container->GetVolumes()->GetVolumes() as $volume) {
                              //             $volumeEntry = $volume->name . ':' . $volume->mountPoint;
                              //             if ($volume->isWritable) {
                              //                 $volumeEntry = $volumeEntry . ':' . 'rw';
                              //             } else {
                              //                 $volumeEntry = $volumeEntry . ':' . 'ro';
                              //             }
                              //             $volumes[] = $volumeEntry;
                              //         }
                              //         $requestBody = [
                              //             'Image' => $this->BuildImageName($container),
                              //         ];
                              //         if (count($volumes) > 0) {
                              //             $requestBody['HostConfig']['Binds'] = $volumes;
                              //         }
                              //         foreach($container->GetSecrets() as $secret) {
                              //             $this->configurationManager->GetAndGenerateSecret($secret);
                              //         }
                              //         $aioVariables = $container->GetAioVariables()->GetVariables();
                              //         foreach($aioVariables as $variable) {
                              //             $config = $this->configurationManager->GetConfig();
                              //             $variableArray = explode('=', $variable);
                              //             $config[$variableArray[0]] = $variableArray[1];
                              //             $this->configurationManager->WriteConfig($config);
                              //             sleep(1);
                              //         }
                              //         $envs = $container->GetEnvironmentVariables()->GetVariables();
                              //         if ($container->GetIdentifier() === 'nextcloud-aio-nextcloud') {
                              //             $envs[] = $this->GetAllNextcloudExecCommands();
                              //         }
                              //         foreach($envs as $key => $env) {
                              //             if (str_starts_with($env, 'extra_params=')) {
                              //                 $env = str_replace('%COLLABORA_SECCOMP_POLICY%', $this->configurationManager->GetCollaboraSeccompPolicy(), $env);
                              //                 $env = str_replace('%NC_DOMAIN%', $this->configurationManager->GetDomain(), $env);
                              //                 $envs[$key] = $env;
                              //                 continue;
                              //             }
                              //             $patterns = ['/%(.*)%/'];
                              //             if(preg_match($patterns[0], $env, $out) === 1) {
                              //                 $replacements = array();
                              //                 if($out[1] === 'NC_DOMAIN') {
                              //                     $replacements[1] = $this->configurationManager->GetDomain();
                              //                 } elseif($out[1] === 'NC_BASE_DN') {
                              //                     $replacements[1] = $this->configurationManager->GetBaseDN();
                              //                 } elseif ($out[1] === 'AIO_TOKEN') {
                              //                     $replacements[1] = $this->configurationManager->GetToken();
                              //                 } elseif ($out[1] === 'BORGBACKUP_MODE') {
                              //                     $replacements[1] = $this->configurationManager->GetBackupMode();
                              //                 } elseif ($out[1] === 'AIO_URL') {
                              //                     $replacements[1] = $this->configurationManager->GetAIOURL();
                              //                 } elseif ($out[1] === 'SELECTED_RESTORE_TIME') {
                              //                     $replacements[1] = $this->configurationManager->GetSelectedRestoreTime();
                              //                 } elseif ($out[1] === 'APACHE_PORT') {
                              //                     $replacements[1] = $this->configurationManager->GetApachePort();
                              //                 } elseif ($out[1] === 'TALK_PORT') {
                              //                     $replacements[1] = $this->configurationManager->GetTalkPort();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_MOUNT') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudMount();
                              //                 } elseif ($out[1] === 'BACKUP_RESTORE_PASSWORD') {
                              //                     $replacements[1] = $this->configurationManager->GetBorgRestorePassword();
                              //                 } elseif ($out[1] === 'CLAMAV_ENABLED') {
                              //                     if ($this->configurationManager->isClamavEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'TALK_RECORDING_ENABLED') {
                              //                     if ($this->configurationManager->isTalkRecordingEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'ONLYOFFICE_ENABLED') {
                              //                     if ($this->configurationManager->isOnlyofficeEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'COLLABORA_ENABLED') {
                              //                     if ($this->configurationManager->isCollaboraEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'TALK_ENABLED') {
                              //                     if ($this->configurationManager->isTalkEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'UPDATE_NEXTCLOUD_APPS') {
                              //                     if ($this->configurationManager->isDailyBackupRunning() && $this->configurationManager->areAutomaticUpdatesEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'TIMEZONE') {
                              //                     if ($this->configurationManager->GetTimezone() === '') {
                              //                         $replacements[1] = 'Etc/UTC';
                              //                     } else {
                              //                         $replacements[1] = $this->configurationManager->GetTimezone();
                              //                     }
                              //                 } elseif ($out[1] === 'COLLABORA_DICTIONARIES') {
                              //                     if ($this->configurationManager->GetCollaboraDictionaries() === '') {
                              //                         $replacements[1] = 'de_DE en_GB en_US es_ES fr_FR it nl pt_BR pt_PT ru';
                              //                     } else {
                              //                         $replacements[1] = $this->configurationManager->GetCollaboraDictionaries();
                              //                     }
                              //                 } elseif ($out[1] === 'IMAGINARY_ENABLED') {
                              //                     if ($this->configurationManager->isImaginaryEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'FULLTEXTSEARCH_ENABLED') {
                              //                     if ($this->configurationManager->isFulltextsearchEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'DOCKER_SOCKET_PROXY_ENABLED') {
                              //                     if ($this->configurationManager->isDockerSocketProxyEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'NEXTCLOUD_UPLOAD_LIMIT') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudUploadLimit();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_MEMORY_LIMIT') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudMemoryLimit();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_MAX_TIME') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudMaxTime();
                              //                 } elseif ($out[1] === 'BORG_RETENTION_POLICY') {
                              //                     $replacements[1] = $this->configurationManager->GetBorgRetentionPolicy();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_TRUSTED_CACERTS_DIR') {
                              //                     $replacements[1] = $this->configurationManager->GetTrustedCacertsDir();
                              //                 } elseif ($out[1] === 'ADDITIONAL_DIRECTORIES_BACKUP') {
                              //                     if ($this->configurationManager->GetAdditionalBackupDirectoriesString() !== '') {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'BORGBACKUP_HOST_LOCATION') {
                              //                     $replacements[1] = $this->configurationManager->GetBorgBackupHostLocation();
                              //                 } elseif ($out[1] === 'APACHE_MAX_SIZE') {
                              //                     $replacements[1] = $this->configurationManager->GetApacheMaxSize();
                              //                 } elseif ($out[1] === 'COLLABORA_SECCOMP_POLICY') {
                              //                     $replacements[1] = $this->configurationManager->GetCollaboraSeccompPolicy();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_STARTUP_APPS') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudStartupApps();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_ADDITIONAL_APKS') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudAdditionalApks();
                              //                 } elseif ($out[1] === 'NEXTCLOUD_ADDITIONAL_PHP_EXTENSIONS') {
                              //                     $replacements[1] = $this->configurationManager->GetNextcloudAdditionalPhpExtensions();
                              //                 } elseif ($out[1] === 'INSTALL_LATEST_MAJOR') {
                              //                     if ($this->configurationManager->shouldLatestMajorGetInstalled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'REMOVE_DISABLED_APPS') {
                              //                     if ($this->configurationManager->shouldDisabledAppsGetRemoved()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } elseif ($out[1] === 'AIO_DATABASE_HOST') {
                              //                     $replacements[1] = gethostbyname('nextcloud-aio-database');
                              //                 } elseif ($out[1] === 'CADDY_IP_ADDRESS') {
                              //                     $replacements[1] = '';
                              //                     $communityContainers = $this->configurationManager->GetEnabledCommunityContainers();
                              //                     if (in_array('caddy', $communityContainers, true)) {
                              //                         $replacements[1] = gethostbyname('nextcloud-aio-caddy');
                              //                     }
                              //                 } elseif ($out[1] === 'WHITEBOARD_ENABLED') {
                              //                     if ($this->configurationManager->isWhiteboardEnabled()) {
                              //                         $replacements[1] = 'yes';
                              //                     } else {
                              //                         $replacements[1] = '';
                              //                     }
                              //                 } else {
                              //                     $secret = $this->configurationManager->GetSecret($out[1]);
                              //                     if ($secret === "") {
                              //                         throw new \Exception("The secret " . $out[1] . " is empty. Cannot substitute its value. Please check if it is defined in secrets of containers.json.");
                              //                     }
                              //                     $replacements[1] = $secret;
                              //                 }
                              //                 $envs[$key] = preg_replace($patterns, $replacements, $env);
                              //             }
                              //         }
                              //         if(count($envs) > 0) {
                              //             $requestBody['Env'] = $envs;
                              //         }
                              //         $requestBody['HostConfig']['RestartPolicy']['Name'] = $container->GetRestartPolicy();
                              //         $requestBody['HostConfig']['ReadonlyRootfs'] = $container->GetReadOnlySetting();
                              //         $exposedPorts = [];
                              //         if ($container->GetInternalPort() !== 'host') {
                              //             foreach($container->GetPorts()->GetPorts() as $value) {
                              //                 $port = $value->port;
                              //                 $protocol = $value->protocol;
                              //                 if ($port === '%APACHE_PORT%') {
                              //                     $port = $this->configurationManager->GetApachePort();
                              //                     if ($port !== '443' && $protocol === 'udp') {
                              //                         continue;
                              //                     }
                              //                 } else if ($port === '%TALK_PORT%') {
                              //                     $port = $this->configurationManager->GetTalkPort();
                              //                 }
                              //                 $portWithProtocol = $port . '/' . $protocol;
                              //                 $exposedPorts[$portWithProtocol] = null;
                              //             }
                              //             $requestBody['HostConfig']['NetworkMode'] = 'nextcloud-aio';
                              //         } else {
                              //             $requestBody['HostConfig']['NetworkMode'] = 'host';
                              //         }
                              //         if(count($exposedPorts) > 0) {
                              //             $requestBody['ExposedPorts'] = $exposedPorts;
                              //             foreach ($container->GetPorts()->GetPorts() as $value) {
                              //                 $port = $value->port;
                              //                 $protocol = $value->protocol;
                              //                 if ($port === '%APACHE_PORT%') {
                              //                     $port = $this->configurationManager->GetApachePort();
                              //                     if ($port !== '443' && $protocol === 'udp') {
                              //                         continue;
                              //                     }
                              //                 } else if ($port === '%TALK_PORT%') {
                              //                     $port = $this->configurationManager->GetTalkPort();
                              //                 }
                              //                 $ipBinding = $value->ipBinding;
                              //                 if ($ipBinding === '%APACHE_IP_BINDING%') {
                              //                     $ipBinding = $this->configurationManager->GetApacheIPBinding();
                              //                     if ($ipBinding === '@INTERNAL') {
                              //                         continue;
                              //                     }
                              //                 }
                              //                 $portWithProtocol = $port . '/' . $protocol;
                              //                 $requestBody['HostConfig']['PortBindings'][$portWithProtocol] = [
                              //                     [
                              //                     'HostPort' => $port,
                              //                     'HostIp' => $ipBinding,
                              //                     ]
                              //                 ];
                              //             }
                              //         }
                              //         $devices = [];
                              //         foreach($container->GetDevices() as $device) {
                              //             if ($device === '/dev/dri' && ! $this->configurationManager->isDriDeviceEnabled()) {
                              //                 continue;
                              //             }
                              //             $devices[] = ["PathOnHost" => $device, "PathInContainer" => $device, "CgroupPermissions" => "rwm"];
                              //         }
                              //         if (count($devices) > 0) {
                              //             $requestBody['HostConfig']['Devices'] = $devices;
                              //         }
                              //         $shmSize = $container->GetShmSize();
                              //         if ($shmSize > 0) {
                              //             $requestBody['HostConfig']['ShmSize'] = $shmSize;
                              //         }
                              //         $tmpfs = [];
                              //         foreach($container->GetTmpfs() as $tmp) {
                              //             $mode = "";
                              //             if (str_contains($tmp, ':')) {
                              //                 $mode = explode(':', $tmp)[1];
                              //                 $tmp = explode(':', $tmp)[0];
                              //             }
                              //             $tmpfs[$tmp] = $mode;
                              //         }
                              //         if (count($tmpfs) > 0) {
                              //             $requestBody['HostConfig']['Tmpfs'] =  $tmpfs;
                              //         }
                              //         $requestBody['HostConfig']['Init'] = $container->GetInit();
                              //         $capAdds = $container->GetCapAdds();
                              //         if (count($capAdds) > 0) {
                              //             $requestBody['HostConfig']['CapAdd'] = $capAdds;
                              //         }
                              //         if (!in_array('NET_RAW', $capAdds, true)) {
                              //             $requestBody['HostConfig']['CapDrop'] = ['NET_RAW'];
                              //         }
                              //         $requestBody['HostConfig']['SecurityOpt'] = ["label:disable"];
                              //         if ($container->isApparmorUnconfined()) {
                              //             $requestBody['HostConfig']['SecurityOpt'] = ["apparmor:unconfined", "label:disable"];
                              //         }
                              //         $mounts = [];
                              //         if ($container->GetIdentifier() === 'nextcloud-aio-borgbackup') {
                              //             foreach ($this->getAllBackupVolumes() as $additionalBackupVolumes) {
                              //                 if ($additionalBackupVolumes !== '') {
                              //                     $mounts[] = ["Type" => "volume", "Source" => $additionalBackupVolumes, "Target" => "/nextcloud_aio_volumes/" . $additionalBackupVolumes, "ReadOnly" => false];
                              //                 }
                              //             }
                              //             foreach ($this->configurationManager->GetAdditionalBackupDirectoriesArray() as $additionalBackupDirectories) {
                              //                 if ($additionalBackupDirectories !== '') {
                              //                     if (!str_starts_with($additionalBackupDirectories, '/')) {
                              //                         $mounts[] = ["Type" => "volume", "Source" => $additionalBackupDirectories, "Target" => "/docker_volumes/" . $additionalBackupDirectories, "ReadOnly" => true];
                              //                     } else {
                              //                         $mounts[] = ["Type" => "bind", "Source" => $additionalBackupDirectories, "Target" => "/host_mounts" . $additionalBackupDirectories, "ReadOnly" => true, "BindOptions" => ["NonRecursive" => true]];
                              //                     }
                              //                 }
                              //             }
                              //         } elseif ($container->GetIdentifier() === 'nextcloud-aio-talk') {
                              //             $requestBody['HostConfig']['Ulimits'] = [["Name" => "nofile", "Hard" => 200000, "Soft" => 200000]];
                              //         } elseif ($container->GetIdentifier() === 'nextcloud-aio-caddy') {
                              //             $requestBody['HostConfig']['ExtraHosts'] = ['host.docker.internal:host-gateway'];
                              //         }
                              //         if (count($mounts) > 0) {
                              //             $requestBody['HostConfig']['Mounts'] = $mounts;
                              //         }
                              //         $url = $this->BuildApiUrl('containers/create?name=' . $container->GetIdentifier());
                              //         try {
                              //             $this->guzzleClient->request(
                              //                 'POST',
                              //                 $url,
                              //                 [
                              //                     'json' => $requestBody
                              //                 ]
                              //             );
                              //         } catch (RequestException $e) {
                              //             throw new \Exception("Could not create container " . $container->GetIdentifier() . ": " . $e->getMessage());
                              //         }
                              //     }
}

pub fn is_any_update_available() -> bool {
    panic!("Not implemented") // TODO
                              //     public function isAnyUpdateAvailable() : bool {
                              //         if (!$this->configurationManager->wasStartButtonClicked()) {
                              //             return false;
                              //         }
                              //         $id = 'nextcloud-aio-apache';
                              //         if ($this->isContainerUpdateAvailable($id) !== "") {
                              //             return true;
                              //         } else {
                              //             return false;
                              //         }
                              //     }
}

fn get_backup_volumes(id: String) -> String {
    panic!("Not implemented") // TODO
                              //     private function getBackupVolumes(string $id) : string
                              //     {
                              //         $container = $this->containerDefinitionFetcher->GetContainerById($id);
                              //         $backupVolumes = '';
                              //         foreach ($container->GetBackupVolumes() as $backupVolume) {
                              //             $backupVolumes .= $backupVolume . ' ';
                              //         }
                              //         foreach ($container->GetDependsOn() as $dependency) {
                              //             $backupVolumes .= $this->getBackupVolumes($dependency);
                              //         }
                              //         return $backupVolumes;
                              //     }
}

fn get_all_backup_volumes() -> Vec<String> {
    panic!("Not implemented") // TODO
                              //     private function getAllBackupVolumes() : array {
                              //         $id = 'nextcloud-aio-apache';
                              //         $backupVolumesArray = explode(' ', $this->getBackupVolumes($id));
                              //         return array_unique($backupVolumesArray);
                              //     }
}

fn get_nextcloud_exec_commands(id: &str) -> String {
    panic!("Not implemented") // TODO
                              //     private function GetNextcloudExecCommands(string $id) : string
                              //     {
                              //         $container = $this->containerDefinitionFetcher->GetContainerById($id);
                              //         $nextcloudExecCommands = '';
                              //         foreach ($container->GetNextcloudExecCommands() as $execCommand) {
                              //             $nextcloudExecCommands .= $execCommand . PHP_EOL;
                              //         }
                              //         foreach ($container->GetDependsOn() as $dependency) {
                              //             $nextcloudExecCommands .= $this->GetNextcloudExecCommands($dependency);
                              //         }
                              //         return $nextcloudExecCommands;
                              //     }
}

fn get_all_nextcloud_exec_commands() -> String {
    panic!("Not implemented") // TODO
                              //     private function GetAllNextcloudExecCommands() : string
                              //     {
                              //         $id = 'nextcloud-aio-apache';
                              //         return 'NEXTCLOUD_EXEC_COMMANDS=' . $this->GetNextcloudExecCommands($id);
                              //     }
}

pub fn get_repo_digests_of_container(container_name: &str) -> Option<Vec<String>> {
    panic!("Not implemented") // TODO
                              //     private function GetRepoDigestsOfContainer(string $containerName) : ?array {
                              //         try {
                              //             $containerUrl = $this->BuildApiUrl(sprintf('containers/%s/json', $containerName));
                              //             $containerOutput = json_decode($this->guzzleClient->get($containerUrl)->getBody()->getContents(), true);
                              //             $imageName = $containerOutput['Image'];
                              //             $imageUrl = $this->BuildApiUrl(sprintf('images/%s/json', $imageName));
                              //             $imageOutput = json_decode($this->guzzleClient->get($imageUrl)->getBody()->getContents(), true);
                              //             if (!isset($imageOutput['RepoDigests'])) {
                              //                 error_log('RepoDigests is not set of container ' . $containerName);
                              //                 return null;
                              //             }
                              //             if (!is_array($imageOutput['RepoDigests'])) {
                              //                 error_log('RepoDigests of ' . $containerName . ' is not an array which is not allowed!');
                              //                 return null;
                              //             }
                              //             $repoDigestArray = [];
                              //             $oneDigestGiven = false;
                              //             foreach($imageOutput['RepoDigests'] as $repoDigest) {
                              //                 $digestPosition = strpos($repoDigest, '@');
                              //                 if ($digestPosition === false) {
                              //                     error_log('Somehow the RepoDigest of ' . $containerName . ' does not contain a @.');
                              //                     return null;
                              //                 }
                              //                 $repoDigestArray[] = substr($repoDigest, $digestPosition + 1);
                              //                 $oneDigestGiven = true;
                              //             }
                              //             if ($oneDigestGiven) {
                              //                 return $repoDigestArray;
                              //             }
                              //             return null;
                              //         } catch (\Exception $e) {
                              //             return null;
                              //         }
                              //     }
}

pub fn is_mastercontainer_update_available() -> bool {
    panic!("Not implemented") // TODO
                              //     public function IsMastercontainerUpdateAvailable() : bool
                              //     {
                              //         $imageName = 'nextcloud/all-in-one';
                              //         $containerName = 'nextcloud-aio-mastercontainer';
                              //         $tag = $this->GetCurrentChannel();
                              //         $runningDigests = $this->GetRepoDigestsOfContainer($containerName);
                              //         if ($runningDigests === null) {
                              //             return true;
                              //         }
                              //         $remoteDigest = $this->dockerHubManager->GetLatestDigestOfTag($imageName, $tag);
                              //         if ($remoteDigest === null) {
                              //             return false;
                              //         }
                              //         foreach ($runningDigests as $runningDigest) {
                              //             if ($remoteDigest === $runningDigest) {
                              //                 return false;
                              //             }
                              //         }
                              //         return true;
                              //     }
}

pub fn send_notification(
    container: Container,
    subject: String,
    message: String,
    file: String,
) -> () {
    panic!("Not implemented") // TODO
                              //     public function sendNotification(Container $container, string $subject, string $message, string $file = '/notify.sh') : void
                              //     {
                              //         if ($this->GetContainerStartingState($container) === ContainerState::Running) {
                              //             $containerName = $container->GetIdentifier();
                              //             $url = $this->BuildApiUrl(sprintf('containers/%s/exec', urlencode($containerName)));
                              //             $response = json_decode(
                              //                 $this->guzzleClient->request(
                              //                     'POST',
                              //                     $url,
                              //                     [
                              //                         'json' => [
                              //                             'AttachStdout' => true,
                              //                             'Tty' => true,
                              //                             'Cmd' => [
                              //                                 'bash',
                              //                                 $file,
                              //                                 $subject,
                              //                                 $message
                              //                             ],
                              //                         ],
                              //                     ]
                              //                 )->getBody()->getContents(),
                              //                 true
                              //             );
                              //             $id = $response['Id'];
                              //             $url = $this->BuildApiUrl(sprintf('exec/%s/start', $id));
                              //             $this->guzzleClient->request(
                              //                 'POST',
                              //                 $url,
                              //                 [
                              //                     'json' => [
                              //                         'Detach' => false,
                              //                         'Tty' => true,
                              //                     ],
                              //                 ]
                              //             );
                              //         }
                              //     }
}

fn disconnect_container_from_bridge_network(id: String) -> () {
    panic!("Not implemented") // TODO
                              //     private function DisconnectContainerFromBridgeNetwork(string $id) : void
                              //     {
                              //         $url = $this->BuildApiUrl(
                              //             sprintf('networks/%s/disconnect', 'bridge')
                              //         );
                              //         try {
                              //             $this->guzzleClient->request(
                              //                 'POST',
                              //                 $url,
                              //                 [
                              //                     'json' => [
                              //                         'container' => $id,
                              //                     ],
                              //                 ]
                              //             );
                              //         } catch (RequestException $e) {
                              //         }
                              //     }
}

pub fn connect_container_id_to_network(id: String, internal_port: String, network: String) -> () {
    panic!("Not implemented") // TODO
                              //     private function ConnectContainerIdToNetwork(string $id, string $internalPort, string $network = 'nextcloud-aio') : void
                              //     {
                              //         if ($internalPort === 'host') {
                              //             return;
                              //         }
                              //         $url = $this->BuildApiUrl('networks/create');
                              //         try {
                              //             $this->guzzleClient->request(
                              //                 'POST',
                              //                 $url,
                              //                 [
                              //                     'json' => [
                              //                         'Name' => $network,
                              //                         'CheckDuplicate' => true,
                              //                         'Driver' => 'bridge',
                              //                         'Internal' => false,
                              //                     ]
                              //                 ]
                              //             );
                              //         } catch (RequestException $e) {
                              //             if ($e->getCode() !== 409) {
                              //                 throw new \Exception("Could not create the nextcloud-aio network: " . $e->getMessage());
                              //             }
                              //         }
                              //         $url = $this->BuildApiUrl(
                              //             sprintf('networks/%s/connect', $network)
                              //         );
                              //         try {
                              //             $this->guzzleClient->request(
                              //                 'POST',
                              //                 $url,
                              //                 [
                              //                     'json' => [
                              //                         'container' => $id,
                              //                     ]
                              //                 ]
                              //             );
                              //         } catch (RequestException $e) {
                              //             if ($e->getCode() !== 403) {
                              //                 throw $e;
                              //             }
                              //         }
                              //     }
}

pub fn connect_master_container_to_network() -> () {
    panic!("Not implemented") // TODO
                              //     public function ConnectMasterContainerToNetwork() : void
                              //     {
                              //         $this->ConnectContainerIdToNetwork('nextcloud-aio-mastercontainer', '');
                              //     }
}

pub fn connect_container_to_network(c: &Container) -> () {
    panic!("Not implemented") // TODO
                              //     public function ConnectContainerToNetwork(Container $container) : void
                              //     {
                              //         $this->ConnectContainerIdToNetwork($container->GetIdentifier(), $container->GetInternalPort());
                              //     }
}

pub fn get_backupcontainer_exit_code() -> i32 {
    panic!("Not implemented") // TODO
                              //     public function GetBackupcontainerExitCode() : int
                              //     {
                              //         $containerName = 'nextcloud-aio-borgbackup';
                              //         $url = $this->BuildApiUrl(sprintf('containers/%s/json', urlencode($containerName)));
                              //         try {
                              //             $response = $this->guzzleClient->get($url);
                              //         } catch (RequestException $e) {
                              //             if ($e->getCode() === 404) {
                              //                 return -1;
                              //             }
                              //             throw $e;
                              //         }
                              //         $responseBody = json_decode((string)$response->getBody(), true);
                              //         $exitCode = $responseBody['State']['ExitCode'];
                              //         if (is_int($exitCode)) {
                              //             return $exitCode;
                              //         } else {
                              //             return -1;
                              //         }
                              //     }
}

pub fn get_databasecontainer_exit_code() -> i32 {
    panic!("Not implemented") // TODO
                              //     public function GetDatabasecontainerExitCode() : int
                              //     {
                              //         $containerName = 'nextcloud-aio-database';
                              //         $url = $this->BuildApiUrl(sprintf('containers/%s/json', urlencode($containerName)));
                              //         try {
                              //             $response = $this->guzzleClient->get($url);
                              //         } catch (RequestException $e) {
                              //             if ($e->getCode() === 404) {
                              //                 return -1;
                              //             }
                              //             throw $e;
                              //         }
                              //         $responseBody = json_decode((string)$response->getBody(), true);
                              //         $exitCode = $responseBody['State']['ExitCode'];
                              //         if (is_int($exitCode)) {
                              //             return $exitCode;
                              //         } else {
                              //             return -1;
                              //         }
                              //     }
}
