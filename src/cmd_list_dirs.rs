use crate::dir_mapping;
use crate::dupe_krill_report::DupeKrillReport;

pub fn handle_cmd(report: &DupeKrillReport) -> i32 {
    match dir_mapping::from_report(report) {
        Ok(dir_mapping) => {
            for dir in dir_mapping.get_all_dirs() {
                let number_of_dupes = dir_mapping.dirs_to_dupe_ids[dir].len();
                let number_of_related_dirs = dir_mapping
                    .get_related_dirs(dir)
                    .map_or(0, |related_dirs| related_dirs.len());
                println!(
                    "{number_of_dupes} {number_of_related_dirs} {}",
                    dir.display()
                );
            }
            0
        }
        Err(err) => {
            log::error!("Error creating directory mapping: {}", err);
            1
        }
    }
}
