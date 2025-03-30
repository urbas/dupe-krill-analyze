use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::dupe_krill_report::DupeKrillReport;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DirMapping {
    pub dirs_to_dupe_ids: HashMap<PathBuf, HashSet<u32>>,
    pub dupe_ids_to_dirs: HashMap<u32, HashSet<PathBuf>>,
}

impl DirMapping {
    /// Returns a HashSet of all directories that contain duplicate files
    pub fn get_all_dirs(&self) -> HashSet<&PathBuf> {
        self.dirs_to_dupe_ids.keys().collect()
    }

    /// Returns directories that share at least one duplicate file with the given directory.
    pub fn get_related_dirs(&self, target_dir: &Path) -> Result<HashSet<&PathBuf>, String> {
        let dupes_in_target = self
            .dirs_to_dupe_ids
            .get(target_dir)
            .ok_or_else(|| format!("Unknown directory {target_dir:?}."))?;
        let mut related_dirs = HashSet::new();
        for dupe_id in dupes_in_target {
            if let Some(dirs) = self.dupe_ids_to_dirs.get(dupe_id) {
                related_dirs.extend(dirs.iter());
            }
        }
        related_dirs.remove(&target_dir.to_path_buf());
        Ok(related_dirs)
    }

    /// Returns the number of duplicate files shared between two directories
    pub fn count_shared_dupes(&self, dir1: &Path, dir2: &Path) -> Result<usize, String> {
        let dupes_in_dir1 = self
            .dirs_to_dupe_ids
            .get(dir1)
            .ok_or_else(|| format!("Unknown directory {dir1:?}."))?;

        let dupes_in_dir2 = self
            .dirs_to_dupe_ids
            .get(dir2)
            .ok_or_else(|| format!("Unknown directory {dir2:?}."))?;

        Ok(dupes_in_dir1.intersection(dupes_in_dir2).count())
    }
}

pub fn from_report(report: &DupeKrillReport) -> Result<DirMapping, String> {
    let mut dupe_ids_to_dirs = HashMap::new();
    let mut dirs_to_dupe_ids = HashMap::new();

    let mut dupe_id = 0;
    for same_content_group in report.dupes.iter() {
        dupe_id += 1;
        for hardlink_group in same_content_group.iter() {
            for path in hardlink_group.iter() {
                if let Some(parent) = path.parent() {
                    dupe_ids_to_dirs
                        .entry(dupe_id)
                        .or_insert_with(HashSet::new)
                        .insert(parent.to_path_buf());
                    dirs_to_dupe_ids
                        .entry(parent.to_path_buf())
                        .or_insert_with(HashSet::new)
                        .insert(dupe_id);
                }
            }
        }
    }

    Ok(DirMapping {
        dirs_to_dupe_ids,
        dupe_ids_to_dirs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dupe_krill_report::DupeKrillReport;
    use std::path::PathBuf;

    #[test]
    fn test_analyse_dirs() {
        let mut report = DupeKrillReport {
            creator: None,
            dupes: Vec::new(),
            scan_duration: None,
            stats: None,
        };

        report.dupes = vec![
            vec![vec![
                PathBuf::from("/dir1/file1").into_boxed_path(),
                PathBuf::from("/dir2/file2").into_boxed_path(),
            ]],
            vec![
                vec![PathBuf::from("/dir1/file3").into_boxed_path()],
                vec![PathBuf::from("/dir2/file4").into_boxed_path()],
            ],
        ];

        let dir_mapping = from_report(&report);

        let mut expected_dirs_to_dupes = HashMap::new();
        let mut dir1_dupes = HashSet::new();
        dir1_dupes.insert(1);
        dir1_dupes.insert(2);
        expected_dirs_to_dupes.insert(PathBuf::from("/dir1"), dir1_dupes);
        let mut dir2_dupes = HashSet::new();
        dir2_dupes.insert(1);
        dir2_dupes.insert(2);
        expected_dirs_to_dupes.insert(PathBuf::from("/dir2"), dir2_dupes);

        let mut expected_dupes_to_dirs = HashMap::new();
        let mut dupe_dirs = HashSet::new();
        dupe_dirs.insert(PathBuf::from("/dir1"));
        dupe_dirs.insert(PathBuf::from("/dir2"));
        expected_dupes_to_dirs.insert(1, dupe_dirs);
        dupe_dirs = HashSet::new();
        dupe_dirs.insert(PathBuf::from("/dir1"));
        dupe_dirs.insert(PathBuf::from("/dir2"));
        expected_dupes_to_dirs.insert(2, dupe_dirs);

        let expected = DirMapping {
            dirs_to_dupe_ids: expected_dirs_to_dupes,
            dupe_ids_to_dirs: expected_dupes_to_dirs,
        };
        assert_eq!(dir_mapping, Ok(expected));
    }
}
