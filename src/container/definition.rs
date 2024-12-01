use crate::container::models::{Container, Definition};
use crate::data::data_const::{get_community_containers_directory, get_containers_file};
use serde_json::from_str;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Deref;
use std::path::Path;
use tokio::sync::OnceCell;
use tracing::error;

pub struct ContainerDefinition(HashMap<String, Container>);

static INSTANCE: OnceCell<ContainerDefinition> = OnceCell::const_new();

async fn fetch_file<P: AsRef<Path>>(path: P) -> Definition {
    fs::read_to_string(path.as_ref())
        .map_err(|e| error!("Error reading file: {}", e))
        .ok()
        .and_then(|file_content| {
            from_str(&file_content)
                .map_err(|e| error!("Error deserializing file: {}", e))
                .ok()
        })
        .unwrap_or_default()
}

impl ContainerDefinition {
    async fn new() -> Self {
        let mut map = HashMap::new();
        let path = get_containers_file();
        for container in fetch_file(path).await.aio_services_v1 {
            map.insert(container.identifier.clone(), container);
        }
        //TODO: Add community containers
        ContainerDefinition(map)
    }

    // async fn new_community(name: &str) -> Self {
    //     let path = get_community_containers_directory()
    //         .join(name)
    //         .join(format!("{}.json", name));
    //     fetch_file(path).await
    // }

    pub async fn instance() -> &'static Self {
        INSTANCE
            .get_or_init(|| async { ContainerDefinition::new().await })
            .await
    }

    pub fn get(&self, id: &str) -> Option<&Container> {
        self.0.get(id)
    }

    pub fn dependency_list(&self, id: &str) -> Vec<&Container> {
        let mut acc = Vec::new();
        let mut done = HashSet::new();
        let mut todo = vec![id];
        while let Some(id) = todo.pop() {
            if !done.contains(id) {
                done.insert(id);
                let c = self.get(id);
                if let Some(c) = c {
                    c.depends_on.iter().for_each(|dep| todo.push(dep));
                    acc.push(c);
                }
            }
        }
        acc.reverse();
        acc
    }
}
