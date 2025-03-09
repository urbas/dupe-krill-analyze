use std::path::PathBuf;

use clap::{Parser, Subcommand};
mod cmd_related_dirs;
mod dir_mapping;
mod dupe_krill_report;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Increase verbosity level (can be used multiple times)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Decrease verbosity level (can be used multiple times)
    #[arg(short, long, action = clap::ArgAction::Count)]
    quiet: u8,

    /// The path to the dupe-krill JSON report. If not given, the report is read from the path specified
    /// in the `DUPE_KRILL_ANALYZER_INPUT_REPORT` environment variable. If the env var is not given, then
    /// the report is read from stdin.
    #[arg(short, long)]
    dupe_krill_report: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints all directories that share at least one duplicate file with the given directory.
    RelatedDirs {
        /// The directory for which to find related directories
        #[arg(required = true)]
        directory: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let log_level = match cli.verbose as i8 - cli.quiet as i8 {
        i8::MIN..=-2 => log::LevelFilter::Off,
        -1 => log::LevelFilter::Error,
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let report = match dupe_krill_report::load_report(cli.dupe_krill_report.as_deref()) {
        Ok(report) => report,
        Err(err) => {
            log::error!("Error loading report: {err}");
            std::process::exit(1);
        }
    };

    env_logger::Builder::new().filter_level(log_level).init();

    let exit_code = match &cli.command {
        Commands::RelatedDirs { directory } => cmd_related_dirs::handle_cmd(&report, directory),
    };
    std::process::exit(exit_code);
}
