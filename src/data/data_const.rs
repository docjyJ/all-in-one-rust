use std::path::PathBuf;

#[cfg(not(feature = "development"))]
fn get_home_directory() -> PathBuf {
    PathBuf::from("/mnt/docker-aio-config")
}

#[cfg(feature = "development")]
fn get_home_directory() -> PathBuf {
    PathBuf::from(".")
}

pub fn get_data_directory() -> PathBuf {
    get_home_directory().join("data")
}

pub fn get_session_directory() -> PathBuf {
    get_home_directory().join("session")
}

pub fn get_config_file() -> PathBuf {
    get_data_directory().join("configuration.json")
}

pub fn get_backup_secret_file() -> PathBuf {
    get_data_directory().join("backupsecret")
}

pub fn get_daily_backup_time_file() -> PathBuf {
    get_data_directory().join("daily_backup_time")
}

pub fn get_additional_backup_directories_file() -> PathBuf {
    get_data_directory().join("additional_backup_directories")
}

pub fn get_daily_backup_block_file() -> PathBuf {
    get_data_directory().join("daily_backup_running")
}

pub fn get_backup_key_file() -> PathBuf {
    get_data_directory().join("borg.config")
}

pub fn get_backup_archives_list() -> PathBuf {
    get_data_directory().join("backup_archives.list")
}

pub fn get_session_date_file() -> PathBuf {
    get_data_directory().join("session_date_file")
}

pub fn get_community_containers_directory() -> PathBuf {
    PathBuf::from("../../../community-containers")
}

pub fn get_containers_file() -> PathBuf {
    PathBuf::from("containers.json")
}
