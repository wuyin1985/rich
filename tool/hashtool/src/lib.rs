use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};

static mut tool: Option<HashTool> = None;

#[no_mangle]
pub extern fn c_get_hash(text: *const c_char) -> u64 {
    unsafe {
        if tool.is_none() {
            tool = Some(HashTool::new());
        }

        let text: String = CStr::from_ptr(text).to_str().expect("Can not read string argument.").to_string();
        return tool.as_mut().unwrap().hash(&text);
    }
}

#[no_mangle]
pub extern fn c_save_hash(text: *const c_char) {
    unsafe {
        if tool.is_none() {
            return;
        }
        let path: String = CStr::from_ptr(text).to_str().expect("Can not read string argument.").to_string();
        tool.as_mut().unwrap().save_reverse_dict_2_file(&path);
        tool = None;
    }
}

pub struct HashTool {
    reverse_dict: HashMap<u64, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HashToolSave {
    data: HashMap<u64, String>,
}

impl HashTool {
    pub fn new() -> Self {
        HashTool { reverse_dict: Default::default() }
    }

    pub fn hash(&mut self, value: &str) -> u64 {
        let mut d = DefaultHasher::default();
        value.hash(&mut d);
        let ret = d.finish();

        self.reverse_dict.insert(ret, value.to_string());
        ret
    }

    pub fn save_reverse_dict_2_file(&self, path: &str) {
        let dict = self.reverse_dict.clone();
        let save = HashToolSave { data: dict };
        let ret = serde_json::ser::to_string(&save).expect("failed to serialize to json");
        std::fs::write(path, ret).expect("unable to write file");
    }
}

