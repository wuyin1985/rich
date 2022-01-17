use std::env;
use std::fs::File;
use std::io::Write;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("CARGO_MANIFEST_DIR {}", env!("CARGO_MANIFEST_DIR"));
    let ron_dir = "../assets/config/ron";
    println!("cargo:rerun-if-changed=..{}", ron_dir);

    // std::fs::read_dir(ron_dir).context(format!("failed to read files from {}", ron_dir))?.into_iter().map(|dir| {
    //     //let d = dir.context("failed to get dir entry")?;
    //
    // });

    Ok(())
}