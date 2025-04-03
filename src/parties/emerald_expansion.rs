//! This file contains functions to parse and write from/into the .parties
//! file format used in the pokemon emerald decomp expansion project.

use super::Parties;

static DEFAULT_TRAINER_REGEX_DELIMITER: &str = "=== ([A-Z0-9_]*) ===";

pub struct ParsingConfig<'a> {
    TrainerRegexDelimiter: &'a str,
}

pub fn from_emerald_expansion_format_config(file_content: &str, config: &ParsingConfig) -> Parties {
    unimplemented!()
}

pub fn from_emerald_expansion_format(file_content: &str) -> Parties {
    let parsing_config = ParsingConfig {
        TrainerRegexDelimiter: DEFAULT_TRAINER_REGEX_DELIMITER,
    };
    from_emerald_expansion_format_config(file_content, &parsing_config)
}
