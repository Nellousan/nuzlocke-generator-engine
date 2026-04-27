use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub project: ProjectOption,
    #[arg(long, value_enum, default_value_t = LogLevel::Debug)]
    pub log_level: LogLevel,
    /// Path to log file.
    #[arg(long, default_value = "log.log")]
    pub log_file: PathBuf,
    /// Path to pokedex file
    #[arg(long, default_value = "pokedex.json")]
    pub pokedex: PathBuf,
    #[arg(short, long, default_values = vec![        
        clap::builder::OsStr::from("bundles/default/gen6.bundle.json"),
        clap::builder::OsStr::from("bundles/default/gen7.bundle.json"),
        clap::builder::OsStr::from("bundles/default/gen8.bundle.json"),
        clap::builder::OsStr::from("bundles/default/gen9.bundle.json"),
    ])]
    pub bundles: Vec<PathBuf>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warning => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum ProjectOption {
    #[command(version, about, long_about = None)]
    EmeraldExpansion {
        /// Path to the decompilation project
        #[arg(value_name = "path", default_value = "pokeemerald-expansion")]
        project_path: PathBuf,
        /// Path to trainers.party file (relative to project path)
        #[arg(value_name = "trainers", default_value = "src/data/trainers.party")]
        trainers_party_file_path: PathBuf,
        /// Path to wild_encounters.json file (relative to project path)
        #[arg(
            value_name = "encounters",
            default_value = "src/data/wild_encounters.json"
        )]
        encounters_file_path: PathBuf,
        /// Encounters will be replaced globally instead of locally.
        #[arg(long, value_name = "global-encounter-rng", default_value_t = false)]
        global_encounter_randomization: bool,
    },
}
