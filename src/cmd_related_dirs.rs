use std::path::Path;

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
    match dir_mapping.get_related_dirs(target_dir) {
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
