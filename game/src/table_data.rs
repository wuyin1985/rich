use bevy::utils::HashMap;

pub trait TableDataItem: serde::de::DeserializeOwned + Send + Sync + 'static {
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
            let id = hashtoollib::hash(name);
            (id, item)
        }).collect::<HashMap<_, _>>();

        Self { dict }
    }

    pub fn load_from_file(path: &str) -> Self {
        let data = std::fs::read(path).expect(format!("failed to load table config {}", path).as_str());
        Self::load_from_bytes(&data)
    }

    pub fn find(&self, name: u64) -> Option<&T> {
        self.dict.get(&name)
    }

    pub fn index(&self, name: u64) -> &T {
        match self.find(name) {
            None => {
                let mut print_name = name.to_string();

                #[cfg(feature = "debug")]
                    {
                        if let Some(s) = hashtoollib::un_hash(name) {
                            print_name = s;
                        }
                    }

                panic!("failed to find '{}' from table {}", print_name, std::any::type_name::<T>());
            }
            Some(t) => {
                t
            }
        }
    }
}
