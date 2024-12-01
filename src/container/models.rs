use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Definition {
    pub aio_services_v1: Vec<Container>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Container {
    #[serde(rename(deserialize = "container_name"))]
    pub identifier: String,
    pub display_name: String,
    #[serde(rename(deserialize = "image"))]
    pub container_name: String,
    #[serde(rename(deserialize = "restart"))]
    pub restart_policy: String,
    #[serde(rename(deserialize = "stop_grace_period"))]
    pub max_shutdown_time: i64,
    pub ports: Vec<ContainerPort>,
    #[serde(rename(deserialize = "internal_port"))]
    pub internal_ports: String,
    pub volumes: Vec<ContainerVolume>,
    #[serde(rename(deserialize = "environment"))]
    pub container_environment_variables: Vec<String>,
    pub depends_on: Vec<String>,
    pub secrets: Vec<String>,
    pub devices: Vec<String>,
    pub cap_add: Vec<String>,
    pub cap_drop: Vec<String>,
    pub shm_size: i64,
    pub apparmor_unconfined: bool,
    pub backup_volumes: Vec<String>,
    pub nextcloud_exec_commands: Vec<String>,
    #[serde(rename(deserialize = "read_only"))]
    pub read_only_root_fs: bool,
    pub tmpfs: Vec<String>,
    pub init: bool,
    pub image_tag: String,
    pub aio_variables: Vec<String>,
    pub documentation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerPort {
    pub ip_binding: String,
    #[serde(rename(deserialize = "port_number"))]
    pub port: String,
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContainerVolume {
    #[serde(rename(deserialize = "source"))]
    pub name: String,
    #[serde(rename(deserialize = "destination"))]
    pub mount_point: String,
    #[serde(rename(deserialize = "writeable"))]
    pub is_writable: bool,
}

impl Default for Definition {
    fn default() -> Self {
        Definition {
            aio_services_v1: Vec::default(),
        }
    }
}

impl Default for Container {
    fn default() -> Self {
        Container {
            image_tag: String::from("%AIO_CHANNEL%"),
            init: true,
            read_only_root_fs: false,
            apparmor_unconfined: false,
            shm_size: -1,
            max_shutdown_time: 10,

            container_name: String::default(),
            cap_add: Vec::default(),
            cap_drop: Vec::default(),
            depends_on: Vec::default(),
            display_name: String::default(),
            container_environment_variables: Vec::default(),
            identifier: String::default(),
            internal_ports: String::default(),
            ports: Vec::default(),
            aio_variables: Vec::default(),
            restart_policy: String::default(),
            secrets: Vec::default(),
            documentation: String::default(),
            devices: Vec::default(),
            backup_volumes: Vec::default(),
            nextcloud_exec_commands: Vec::default(),
            tmpfs: Vec::default(),
            volumes: Vec::default(),
        }
    }
}

impl PartialEq for Container {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}
