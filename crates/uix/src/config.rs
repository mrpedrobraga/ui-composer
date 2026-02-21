#![allow(unused)]
use std::path::PathBuf;
use std::sync::RwLock;
use std::time::SystemTime;
use toml::Table;

pub fn is_container(name: &str) -> bool {
    //get_containers().iter().any(|c| c == name)

    /* It's a container if its name is lowercase. */
    name.chars().next().is_some_and(|c| c.is_lowercase())
}

struct UixConfig {
    last_loaded: SystemTime,
    container_elements: Vec<String>,
}

static CONFIG_CACHE: RwLock<Option<UixConfig>> = RwLock::new(None);

fn get_containers() -> Vec<String> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    let manifest_path = manifest_dir.join("Cargo.toml");

    let mtime = std::fs::metadata(&manifest_path)
        .and_then(|m| m.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH);

    {
        let cache = CONFIG_CACHE.read().unwrap();
        if let Some(c) = &*cache {
            if c.last_loaded >= mtime {
                return c.container_elements.clone();
            }
        }
    }

    let mut cache = CONFIG_CACHE.write().unwrap();
    let mut containers = vec![];

    if let Ok(content) = std::fs::read_to_string(&manifest_path) {
        if let Ok(value) = toml::from_str::<Table>(&content) {
            // Look for [package.metadata.ui_composer]
            if let Some(list) = value
                .get("package")
                .and_then(|p| p.get("metadata"))
                .and_then(|m| m.get("ui_composer"))
                .and_then(|u| u.get("uix_containers"))
                .and_then(|c| c.as_array())
            {
                containers.extend(list.iter().filter_map(|v| v.as_str().map(|s| s.to_string())));
            }
        } else {
            eprintln!("Failed to parse content in {}", manifest_path.display());
            eprintln!("{}", content);
        }
    }

    *cache = Some(UixConfig {
        last_loaded: mtime,
        container_elements: containers.clone(),
    });

    containers
}

