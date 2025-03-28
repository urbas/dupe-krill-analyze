use std::path::PathBuf;

use crate::{dir_mapping, dupe_krill_report::DupeKrillReport, file_utils};

pub fn handle_cmd(report: &DupeKrillReport, directories: &[PathBuf]) -> i32 {
    match containing_dirs(report, directories) {
        Ok(()) => 0,
        Err(err) => {
            log::error!("{err}");
            1
        }
    }
}

pub fn containing_dirs(report: &DupeKrillReport, directories: &[PathBuf]) -> Result<(), String> {
    let dir_mapping = dir_mapping::from_report(report)?;

    let mut potential_containing_dirs = std::collections::HashSet::new();
    for target_dir in directories {
        if let Ok(related) = dir_mapping.get_related_dirs(target_dir) {
            potential_containing_dirs.extend(related);
        }
    }

    for dir in directories {
        potential_containing_dirs.remove(dir);
    }

    for containing_dir in potential_containing_dirs {
        let mut contains_all = true;

        for target_dir in directories {
            if !target_dir.exists() || !containing_dir.exists() {
                contains_all = false;
                break;
            }

            if !file_utils::is_directory_subsumed(containing_dir, target_dir)? {
                contains_all = false;
                break;
            }
        }

        if contains_all {
            println!("{}", containing_dir.display());
        }
    }

    Ok(())
}
