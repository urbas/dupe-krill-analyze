use std::path::Path;

use crate::dir_mapping;
use crate::dupe_krill_report::DupeKrillReport;

pub fn handle_cmd(report: &DupeKrillReport, target_dir: &Path, show_shared_dupes: bool) -> i32 {
    match print_related_dirs(report, target_dir, show_shared_dupes) {
        Ok(()) => 0,
        Err(err) => {
            log::error!("{}", err);
            1
        }
    }
}

fn print_related_dirs(
    report: &DupeKrillReport,
    target_dir: &Path,
    show_shared_dupes: bool,
) -> Result<(), String> {
    let dir_mapping = dir_mapping::from_report(report)?;
    let related_dirs = dir_mapping.get_related_dirs(target_dir)?;
    for related_dir in related_dirs {
        if show_shared_dupes {
            let shared_dupes = dir_mapping.count_shared_dupes(target_dir, related_dir)?;
            println!("{shared_dupes} {}", related_dir.display());
        } else {
            println!("{}", related_dir.display());
        }
    }

    Ok(())
}
