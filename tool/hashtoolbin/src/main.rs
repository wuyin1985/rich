use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use hashtoollib::HashTool;
use clap::Parser;
use game::prelude::*;
use anyhow::{Context, Result};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    save_hash_from_config: bool,
}

fn save_dict_2_rust_file(map: HashMap<u64, String>) -> Result<()> {
    let full_path  = format!("{}/../../game/src/str_gen.rs", env!("CARGO_MANIFEST_DIR"));
    let mut f = File::create(&full_path).context(format!("[{}]", &full_path))?;
    f.write_all(b"use std::collections::HashMap;\n")?;
    f.write_all(b"\n")?;
    for (k, v) in map {
        let word = format!("pub const {}:u64 = {};\n", v, k);
        f.write_all(word.as_bytes())?;
    }

    Ok(())
}

fn main() {
    println!("<hashlib tool>");
    let mut app = App::new();
    game::load_battle_tables(&mut app);
    let dict = hashtoollib::get_reverse_dict();
    match save_dict_2_rust_file(dict) {
        Ok(_) => {
            println!("save complete");
        }
        Err(e) => {
            panic!("error: {:?}", e);
        }
    }
}
