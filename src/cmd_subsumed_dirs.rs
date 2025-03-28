use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{dir_mapping, dupe_krill_report::DupeKrillReport};

pub fn handle_cmd(report: &DupeKrillReport, directories: &[PathBuf]) -> i32 {
    match subsumed_dirs(report, directories) {
        Ok(_) => 0,
        Err(msg) => {
            log::error!("{msg}");
            1
        }
    }
}

pub fn subsumed_dirs(report: &DupeKrillReport, directories: &[PathBuf]) -> Result<(), String> {
    let dir_mapping = dir_mapping::from_report(report)?;

    for directory in directories {
        let related_dirs = dir_mapping.get_related_dirs(directory).unwrap_or_default();
        for related_dir in related_dirs {
            if related_dir != directory
                && related_dir.exists()
                && is_directory_subsumed(directory, related_dir)?
            {
                println!("{}", related_dir.display());
            }
        }
    }
    Ok(())
}

/// Checks if the second directory is contained by the first directory
fn is_directory_subsumed(container: &Path, potential_subset: &Path) -> Result<bool, String> {
    for subset_file in find_files(potential_subset)? {
        let container_file = container.join(get_relative_path(&subset_file, potential_subset)?);
        if !container_file.exists() || !are_files_identical(&subset_file, &container_file)? {
            return Ok(false);
        }
    }

    // All files in the subset exist and are identical in the container
    Ok(true)
}

/// Lists all files in a directory recursively
fn find_files(directory: &Path) -> Result<Vec<PathBuf>, String> {
    let output = Command::new("find")
        .arg(directory)
        .arg("-type")
        .arg("f,l")
        .output();

    match output {
        Ok(output) => {
            if !output.status.success() {
                return Ok(vec![]);
            }
            Ok(String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(PathBuf::from)
                .collect())
        }
        Err(err) => Err(format!(
            "Failed to list files in {directory:?}. Error: {err}"
        )),
    }
}

/// Extracts the relative path of a file from a base directory
fn get_relative_path<'a>(file: &'a Path, base_dir: &'a Path) -> Result<&'a Path, String> {
    file.strip_prefix(base_dir)
        .map_err(|err| format!("Could not determine the relative path of {file:?}. Error: {err}"))
}

/// Checks if two files have identical content using the `cmp` tool
fn are_files_identical(file1: &Path, file2: &Path) -> Result<bool, String> {
    let cmp_result = Command::new("cmp")
        .arg("-s") // Silent mode, only return status
        .arg(file1)
        .arg(file2)
        .status();

    match cmp_result {
        Ok(status) => Ok(status.success()),
        Err(err) => Err(format!(
            "Failed to compare files {file1:?} and {file2:?}. Error: {err}"
        )),
    }
}
