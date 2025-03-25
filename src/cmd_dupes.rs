use crate::dupe_krill_report::DupeKrillReport;
use std::collections::HashSet;
use std::path::Path;

pub fn handle_cmd(report: &DupeKrillReport, file: &Path) -> i32 {
    let mut dupes = HashSet::new();
    let mut found = false;

    for same_content_group in &report.dupes {
        let mut group_contains_file = false;

        for hardlink_group in same_content_group {
            for path in hardlink_group {
                if path.as_ref() == file {
                    group_contains_file = true;
                    found = true;
                    break;
                }
            }
            if group_contains_file {
                break;
            }
        }

        if group_contains_file {
            for hardlink_group in same_content_group {
                for path in hardlink_group {
                    dupes.insert(path.as_ref());
                }
            }
        }
    }

    dupes.remove(file);
    for dupe in dupes {
        println!("{}", dupe.display());
    }

    if found {
        0
    } else {
        eprintln!("No duplicaes of {file:?} found.");
        1
    }
}
