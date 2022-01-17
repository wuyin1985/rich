use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;

use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Serialize;

use super::attacker_config::AttackerConfig;

pub trait TableDataItem: serde::de::DeserializeOwned {
    fn get_name(&self) -> &str;
}

pub struct TableData<T> {
    dict: HashMap<u64, T>,
}

impl<T> TableData<T> where T: TableDataItem {
    pub fn load_from_bytes(data: &[u8]) -> Self {
        let list: Vec<T> = match ron::de::from_bytes(data) {
            Ok(x) => x,
            Err(e) => {
                panic!("failed to load table {}, error: {}", std::any::type_name::<T>(), e);
            }
        };

        let dict = list.into_iter().map(|item| {
            let name = item.get_name();
            let mut s = DefaultHasher::new();
            name.hash(&mut s);
            let id = s.finish();
            (id, item)
        }).collect::<HashMap<_, _>>();

        Self { dict }
    }

    pub fn find(&self, name: u64) -> Option<&T> {
        self.dict.get(&name)
    }

    pub fn index(&self, name: u64) -> &T {
        match self.find(name) {
            None => {
                panic!("failed to find {} from table {}", name, std::any::type_name::<T>());
            }
            Some(t) => {
                t
            }
        }
    }

    #[cfg(feature = "build_editor")]
    pub fn gen_bin(from_path: &str, to_path: &str) {
        use ron::de::from_reader;
        use ron::ser::{to_string_pretty, PrettyConfig};

    }

    //todo generate str id
    pub fn index_by_str(&self, name: &str) -> &T {
        let mut h = DefaultHasher::new();
        name.hash(&mut h);
        let id = h.finish();
        self.index(id)
    }
}
