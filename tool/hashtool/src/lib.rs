use std::collections::HashMap;
use std::ffi::CStr;
use std::fs::File;
use std::io::BufReader;
use std::os::raw::c_char;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

static mut TOOL: Option<HashTool> = None;

#[no_mangle]
pub extern fn c_get_hash(text: *const c_char) -> u64 {
    unsafe {
        if TOOL.is_none() {
            TOOL = Some(HashTool::new());
        }

        let text: String = CStr::from_ptr(text).to_str().expect("Can not read string argument.").to_string();
        return TOOL.as_mut().unwrap().hash(&text);
    }
}

#[no_mangle]
pub extern fn c_save_hash(text: *const c_char) {
    unsafe {
        if TOOL.is_none() {
            return;
        }
        #[cfg(feature = "save_reverse_hash")]
            {
                let path: String = CStr::from_ptr(text).to_str().expect("Can not read string argument.").to_string();
                TOOL.as_mut().unwrap().save_reverse_dict_2_file(&path);
            }
        TOOL = None;
    }
}


pub fn hash(value: &str) -> u64 {
    if cfg!(feature = "save_reverse_hash") {
        unsafe {
            if TOOL.is_none() {
                TOOL = Some(HashTool::new());
            }
            TOOL.as_mut().unwrap().hash(value)
        }
    } else {
        HashTool::hash_impl(value)
    }
}

#[cfg(feature = "save_reverse_hash")]
pub fn un_hash(hash: u64) -> Option<String> {
    unsafe {
        if TOOL.is_none() {
            None
        } else {
            TOOL.as_mut().unwrap().un_hash(hash)
        }
    }
}

#[cfg(feature = "save_reverse_hash")]
pub fn load_reverse_dict(path: &str) {
    unsafe {
        if TOOL.is_none() {
            TOOL = Some(HashTool::new());
        }
        TOOL.as_mut().unwrap().load_reverse_hash(path);
    }
}

pub struct HashTool {
    #[cfg(feature = "save_reverse_hash")]
    pub reverse_dict: Mutex<HashMap<u64, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HashToolSave {
    data: HashMap<u64, String>,
}

impl HashTool {
    pub fn new() -> Self {
        HashTool {
            #[cfg(feature = "save_reverse_hash")]
            reverse_dict: Default::default()
        }
    }

    #[inline]
    pub(self) fn hash_impl(value: &str) -> u64 {
        xxh3_64(value.as_bytes())
    }

    pub fn hash(&mut self, value: &str) -> u64 {
        let ret = Self::hash_impl(value);
        #[cfg(feature = "save_reverse_hash")]
            {
                let mut dict = self.reverse_dict.lock().unwrap();
                dict.insert(ret, value.to_string());
            }
        ret
    }

    #[cfg(feature = "save_reverse_hash")]
    pub fn save_reverse_dict_2_file(&self, path: &str) {
        let guard = self.reverse_dict.lock().unwrap();
        let dict = guard.clone();
        let save = HashToolSave { data: dict };
        let ret = serde_json::ser::to_string(&save).expect("failed to serialize to json");
        std::fs::write(path, ret).expect("unable to write file");
    }

    #[cfg(feature = "save_reverse_hash")]
    pub fn load_reverse_hash(&mut self, path: &str) {
        let file = File::open(path).expect(format!("failed to load file at {}", path).as_str());
        let reader = BufReader::new(file);
        let ret: HashToolSave = serde_json::de::from_reader(reader).unwrap();

        let mut g = self.reverse_dict.lock().unwrap();
        ret.data.into_iter().for_each(|(k, v)| {
            g.insert(k, v);
        });
    }

    #[cfg(feature = "save_reverse_hash")]
    pub fn un_hash(&mut self, hash: u64) -> Option<String> {
        let g = self.reverse_dict.lock().unwrap();
        g.get(&hash).cloned()
    }
}

