use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::time::Duration;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DupeKrillReport {
    #[allow(unused)]
    pub creator: Option<String>,
    /// A multi-level vec, where:
    /// - first-level elements correspond to files with the same content.
    /// - second-level has 1 or more elements, where each element is a group of hard-linked files.
    pub dupes: Vec<Vec<Vec<Box<Path>>>>,
    #[allow(unused)]
    pub scan_duration: Option<Duration>,
    #[allow(unused)]
    pub stats: Option<Stats>,
}

#[derive(serde::Deserialize)]
pub struct Stats {
    #[allow(unused)]
    pub added: usize,
    #[allow(unused)]
    pub skipped: usize,
    #[allow(unused)]
    pub dupes: usize,
    #[allow(unused)]
    pub bytes_deduplicated: usize,
    #[allow(unused)]
    pub hardlinks: usize,
    #[allow(unused)]
    pub bytes_saved_by_hardlinks: usize,
}

pub fn load_report(report_path: Option<&Path>) -> Result<DupeKrillReport, io::Error> {
    match report_path {
        Some(path) => serde_json::from_reader(BufReader::new(File::open(path)?))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)),
        None => {
            // Check if environment variable is set
            if let Ok(env_path) = std::env::var("DUPE_KRILL_ANALYZER_INPUT_REPORT") {
                let path = Path::new(&env_path);
                serde_json::from_reader(BufReader::new(File::open(path)?))
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            } else {
                serde_json::from_reader(io::stdin())
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
        }
    }
}
