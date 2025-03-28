use std::path::PathBuf;

use crate::{dir_mapping, dupe_krill_report::DupeKrillReport, file_utils};

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
                && file_utils::is_directory_subsumed(directory, related_dir)?
            {
                println!("{}", related_dir.display());
            }
        }
    }
    Ok(())
}
