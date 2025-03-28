use std::path::PathBuf;
use std::process::Command;

use crate::{dir_mapping, dupe_krill_report::DupeKrillReport};

pub fn handle_cmd(report: &DupeKrillReport, directories: &[PathBuf]) -> i32 {
    match dupe_dirs(report, directories) {
        Ok(_) => 0,
        Err(msg) => {
            log::error!("{msg}");
            1
        }
    }
}

pub fn dupe_dirs(report: &DupeKrillReport, directories: &[PathBuf]) -> Result<(), String> {
    let dir_mapping = dir_mapping::from_report(report)?;

    for directory in directories {
        let related_dirs = dir_mapping.get_related_dirs(directory).unwrap_or_default();
        for related_dir in related_dirs {
            if related_dir != directory && is_directory_identical(directory, related_dir) {
                println!("{}", related_dir.display());
            }
        }
    }
    Ok(())
}

/// Checks if two directories have identical content using the diff tool
fn is_directory_identical(dir1: &PathBuf, dir2: &PathBuf) -> bool {
    let output = Command::new("diff")
        .arg("-r")
        .arg("--brief")
        .arg(dir1)
        .arg(dir2)
        .output();

    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
