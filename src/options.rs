use std::path::PathBuf;

use thiserror::Error;
use tracing::Level;

#[derive(Debug, Error)]
pub enum OptionsError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    #[error("Getopt Error: {0}")]
    GetoptFail(#[from] getopts::Fail),
}

#[derive(Debug, Default)]
pub struct Options {
    pub project_options: BaseProjectOptions,
    pub log_level: Option<tracing::Level>,
    pub log_file: Option<PathBuf>,
    pub print_usage: bool,
}

#[derive(Debug)]
#[expect(dead_code)]
pub enum BaseProjectOptions {
    Expansion(ExpansionOptions),
}

impl Default for BaseProjectOptions {
    fn default() -> Self {
        Self::Expansion(ExpansionOptions::default())
    }
}

#[derive(Debug)]
#[expect(dead_code)]
pub struct ExpansionOptions {
    pub project_path: PathBuf,
    pub trainers_party_file_path: PathBuf,
}

impl Default for ExpansionOptions {
    fn default() -> Self {
        Self {
            project_path: PathBuf::from("pokeemerald-expansion"),
            trainers_party_file_path: PathBuf::from("/src/data/trainers.party"),
        }
    }
}

fn print_usage(opts: &getopts::Options, program: &str) {
    let brief = format!("{}: Nuzlocke Generator Engine", program);
    print!("{}", opts.usage(&brief));
}

fn parse_options_inner(
    opts: &getopts::Options,
    args: Vec<String>,
) -> Result<Options, OptionsError> {
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => return Err(e.into()),
    };

    let mut options = Options::default();
    options.log_level = matches
        .opt_str("log-level")
        .map(
            |log_level_opt| match log_level_opt.to_lowercase().as_str() {
                "debug" => Ok(Level::DEBUG),
                "info" => Ok(Level::INFO),
                "warn" => Ok(Level::WARN),
                "error" => Ok(Level::ERROR),
                _ => Err(OptionsError::InvalidArgument(log_level_opt)),
            },
        )
        .transpose()?;

    options.log_file = matches.opt_str("log-file").map(|s| s.into());

    options.print_usage = matches.opt_present("help");

    let mut project_options = ExpansionOptions::default();
    if let Some(path) = matches.opt_str("project") {
        project_options.project_path = path.into();
    }

    options.project_options = BaseProjectOptions::Expansion(project_options);

    Ok(options)
}

pub fn parse_options() -> Result<Options, OptionsError> {
    let args: Vec<String> = std::env::args().collect();

    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optopt("P", "project", "Specify base decomp project path", "PATH");
    opts.optopt(
        "L",
        "log-level",
        "Secify log level (DEBUG, INFO, WARN, ERROR)",
        "LEVEL",
    );
    opts.optopt("", "log-file", "Specify log file path", "PATH");
    opts.optflag("h", "help", "Print usage");

    let res = parse_options_inner(&opts, args);
    if let Err(err) = res {
        print_usage(&opts, &program);
        return Err(err);
    }

    res
}
