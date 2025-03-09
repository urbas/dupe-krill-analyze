use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::dir_mapping;
use crate::dupe_krill_report::DupeKrillReport;

pub fn handle_cmd(report: &DupeKrillReport, target_dir: &Path) -> i32 {
    let dir_mapping = match dir_mapping::from_report(report) {
        Ok(dir_mapping) => dir_mapping,
        Err(err) => {
            log::error!("Error analysing the dupe-krill report: {err}");
            return 1;
        }
    };
    match find_related_dirs(&dir_mapping, target_dir) {
        Ok(related_dirs) => {
            for dir in related_dirs {
                println!("{}", dir.display());
            }
            0
        }
        Err(err) => {
            log::error!("Error finding related directories: {err}");
            1
        }
    }
}

fn find_related_dirs(
    dir_mapping: &dir_mapping::DirMapping,
    target_dir: &Path,
) -> Result<HashSet<PathBuf>, String> {
    let dupes_in_target = dir_mapping
        .dirs_to_dupe_ids
        .get(target_dir)
        .ok_or_else(|| format!("Unknown directory {target_dir:?}."))?;

    let mut related_dirs = HashSet::new();
    for dupe_id in dupes_in_target {
        if let Some(dirs) = dir_mapping.dupe_ids_to_dirs.get(dupe_id) {
            related_dirs.extend(dirs.iter().cloned());
        }
    }

    Ok(related_dirs)
}
