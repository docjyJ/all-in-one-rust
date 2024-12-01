use serde::{Deserialize, Serialize};
use serde_json::{to_vec, Error, Result};
use std::env::var;
use std::fs::write;
use std::path::Path;

mod int_bool {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(if *value { 1 } else { 0 })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|data: i32| data == 1)
    }
}
mod string_bool {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(if *value { "true" } else { "false" })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|data: &str| data == "true")
    }
}
mod string_vec {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value.join(" ").as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|data: &str| {
            data.split(' ')
                .map(String::from)
                .filter(|x| !x.is_empty())
                .collect()
        })
    }
}
mod int_string {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u16, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(value.to_string().as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
            .and_then(|data: &str| data.parse().map_err(serde::de::Error::custom))
    }
}

fn not(value: &bool) -> bool {
    !*value
}

#[derive(Eq, PartialEq, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackupMode {
    #[serde(skip)]
    #[default]
    None,
    Backup,
    Check,
    CheckRepair,
    Test,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Configuration {
    pub password: String,
    #[cfg(not(target_arch = "arm"))]
    #[serde(
        rename = "isClamavEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_clamav_enabled: bool,
    #[serde(
        rename = "isDockerSocketProxyEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_docker_socket_proxy_enabled: bool,
    #[serde(
        rename = "isWhiteboardEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_whiteboard_enabled: bool,
    #[serde(
        rename = "isImaginaryEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_imaginary_enabled: bool,
    #[serde(
        rename = "isFulltextsearchEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_fulltextsearch_enabled: bool,
    #[serde(
        rename = "isOnlyofficeEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_onlyoffice_enabled: bool,
    #[serde(
        rename = "isCollaboraEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_collabora_enabled: bool,
    #[serde(
        rename = "isTalkEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_talk_enabled: bool,
    #[serde(
        rename = "isTalkRecordingEnabled",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub is_talk_recording_enabled: bool,
    #[serde(
        rename = "wasStartButtonClicked",
        with = "int_bool",
        skip_serializing_if = "not"
    )]
    pub was_start_button_clicked: bool,
    #[serde(with = "int_bool", skip_serializing_if = "not")]
    pub install_latest_major: bool,
    #[serde(with = "int_bool", skip_serializing_if = "not")]
    pub instance_restore_attempt: bool,
    #[serde(with = "string_bool", skip_serializing_if = "not")]
    pub collabora_seccomp_disabled: bool,
    #[serde(with = "string_bool", skip_serializing_if = "not")]
    pub disable_backup_section: bool,
    #[serde(with = "string_bool", skip_serializing_if = "not")]
    pub nextcloud_enable_dri_device: bool,

    #[serde(with = "int_string")]
    pub apache_port: u16,
    #[serde(with = "int_string")]
    pub talk_port: u16,
    #[serde(with = "int_string")]
    pub nextcloud_max_time: u16,

    #[serde(with = "string_vec", skip_serializing_if = "Vec::is_empty")]
    pub nextcloud_additional_apks: Vec<String>,
    #[serde(with = "string_vec", skip_serializing_if = "Vec::is_empty")]
    pub nextcloud_additional_php_extensions: Vec<String>,
    #[serde(with = "string_vec", skip_serializing_if = "Vec::is_empty")]
    pub aio_community_containers: Vec<String>,

    pub nextcloud_upload_limit: String,
    pub nextcloud_memory_limit: String,
    pub borg_retention_policy: String,
    pub docker_socket_path: String,
    pub nextcloud_datadir: String,

    pub nextcloud_mount: Option<String>,
    pub trusted_cacerts_dir: Option<String>,
    pub apache_ip_binding: Option<String>,
    pub nextcloud_keep_disabled_apps: Option<String>,
    pub borg_backup_host_location: Option<String>,
    #[serde(rename = "AIO_URL")]
    pub aio_url: Option<String>,
    pub aio_token: Option<String>,
    pub backup_mode: BackupMode,
    pub domain: Option<String>,
    pub nextcloud_password: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            password: String::new(),
            #[cfg(not(target_arch = "arm"))]
            is_clamav_enabled: false,
            is_docker_socket_proxy_enabled: false,
            is_whiteboard_enabled: false,
            is_imaginary_enabled: false,
            is_fulltextsearch_enabled: false,
            is_onlyoffice_enabled: false,
            is_collabora_enabled: false,
            is_talk_enabled: false,
            is_talk_recording_enabled: false,
            was_start_button_clicked: false,
            install_latest_major: false,
            instance_restore_attempt: false,
            collabora_seccomp_disabled: false,
            disable_backup_section: false,
            aio_community_containers: Vec::new(),
            nextcloud_datadir: String::from("nextcloud_aio_nextcloud_data"),
            nextcloud_mount: None,
            trusted_cacerts_dir: None,
            apache_ip_binding: None,
            nextcloud_keep_disabled_apps: None,
            nextcloud_enable_dri_device: false,
            apache_port: 443,
            talk_port: 3478,
            nextcloud_upload_limit: String::from("10G"),
            nextcloud_memory_limit: String::from("512M"),
            nextcloud_max_time: 3600,
            borg_retention_policy: String::from(
                "--keep-within=7d --keep-weekly=4 --keep-monthly=6",
            ),
            docker_socket_path: String::from("/var/run/docker.sock"),
            nextcloud_additional_apks: Vec::from([String::from("imagemagick")]),
            nextcloud_additional_php_extensions: Vec::from([String::from("imagick")]),
            borg_backup_host_location: None,
            aio_url: None,
            aio_token: None,
            backup_mode: BackupMode::None,
            domain: None,
            nextcloud_password: None,
        }
    }
}

impl Configuration {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        serde_json::from_slice(std::fs::read(path).map_err(Error::io)?.as_slice())
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        write(path, to_vec(self)?).map_err(Error::io)
    }

    pub fn update_from_env(&mut self) {
        if let Ok(Ok(data)) = var("APACHE_PORT").map(|x| x.parse()) {
            self.apache_port = data
        }
        if let Ok(Ok(data)) = var("TALK_PORT").map(|x| x.parse()) {
            self.talk_port = data
        }
        if let Ok(Ok(data)) = var("NEXTCLOUD_MAX_TIME").map(|x| x.parse()) {
            self.nextcloud_max_time = data
        }

        if let Ok(data) = var("AIO_DISABLE_BACKUP_SECTION") {
            self.disable_backup_section = data == "true";
        }
        if let Ok(data) = var("COLLABORA_SECCOMP_DISABLED") {
            self.collabora_seccomp_disabled = data == "true";
        }
        if let Ok(data) = var("NEXTCLOUD_ENABLE_DRI_DEVICE") {
            self.nextcloud_enable_dri_device = data == "true";
        }

        if let Ok(data) = var("NEXTCLOUD_MOUNT") {
            self.nextcloud_mount = Some(data);
        }
        if let Ok(data) = var("NEXTCLOUD_TRUSTED_CACERTS_DIR") {
            self.trusted_cacerts_dir = Some(data);
        }
        if let Ok(data) = var("APACHE_IP_BINDING") {
            self.apache_ip_binding = Some(data);
        }
        if let Ok(data) = var("NEXTCLOUD_KEEP_DISABLED_APPS") {
            self.nextcloud_keep_disabled_apps = Some(data);
        }

        if let Ok(data) = var("NEXTCLOUD_UPLOAD_LIMIT") {
            self.nextcloud_upload_limit = data;
        }
        if let Ok(data) = var("NEXTCLOUD_MEMORY_LIMIT") {
            self.nextcloud_memory_limit = data;
        }
        if let Ok(data) = var("BORG_RETENTION_POLICY") {
            self.borg_retention_policy = data;
        }
        if let Ok(data) = var("WATCHTOWER_DOCKER_SOCKET_PATH") {
            self.docker_socket_path = data;
        }

        if let Ok(data) = var("NEXTCLOUD_ADDITIONAL_APKS") {
            self.nextcloud_additional_apks = data
                .split(' ')
                .map(String::from)
                .filter(|x| !x.is_empty())
                .collect();
        }
        if let Ok(data) = var("NEXTCLOUD_ADDITIONAL_PHP_EXTENSIONS") {
            self.nextcloud_additional_php_extensions = data
                .split(' ')
                .map(String::from)
                .filter(|x| !x.is_empty())
                .collect();
        }
        if let Ok(data) = var("AIO_COMMUNITY_CONTAINERS") {
            self.aio_community_containers = data
                .split(' ')
                .map(String::from)
                .filter(|x| !x.is_empty())
                .collect();
        }
    }
}
