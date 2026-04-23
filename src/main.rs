use std::{
    fs,
    io::{self, IsTerminal},
    path::PathBuf,
};

use anyhow::{Result, bail};
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional: save destination key. (default: "default")
    #[clap(default_value = "default")]
    key: String,
}

fn main() -> Result<()> {
    let Args { key } = Args::parse();
    let key_regex = Regex::new("^[a-z]+$")?;
    if !key_regex.is_match(&key) {
        bail!("\"{}\" is invalid key. Keys must be match ^[a-z]+$", key);
    }
    let dir_to_save = PathBuf::from("/dev/shm/th/");
    let target_file = dir_to_save.join(key);
    if !dir_to_save.exists() {
        fs::create_dir(dir_to_save)?;
    }
    if !target_file.exists() {
        fs::write(&target_file, "")?;
    }
    let stdin = io::stdin();
    if stdin.is_terminal() {
        // no piped info
        // get mode
        let content = fs::read_to_string(target_file)?;
        println!("{}", content);
    } else {
        // with piped info
        // set mode
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        fs::write(target_file, buf)?;
    }
    Ok(())
}
