use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// target file
    #[arg(required = true)]
    file: String,
    /// save the target file
    #[arg(long)]
    save: bool,
}

const APP_NAME: &str = "touchx";

fn main() {
    let cli = Cli::parse();

    if cli.save {
        save_file_to_data_dir(&cli.file).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        })
    } else {
        handle_file(&cli.file).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    }
}

fn save_file_to_data_dir(file: &str) -> Result<(), std::io::Error> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME)?;

    let target_file_path = xdg_dirs
        .place_data_file(file)?;

    let mut target_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&target_file_path)?;
    let source_data = fs::read(file)?;
    target_file.write_all(&source_data)?;
    Ok(())
}

fn handle_file(file: &str) -> io::Result<()> {
    if Path::new(file).exists() {
        return Ok(())
    }

    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME)?;
    if let Some(source_file_path) = xdg_dirs.find_data_file(file) {
        let source_data = fs::read(source_file_path)?;
        let mut file = File::create(file)?;
        file.write_all(&source_data)?;
    } else {
        File::create(file)?;
    }

    Ok(())
}
