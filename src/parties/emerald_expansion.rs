//! This file contains functions to parse and write from/into the .parties
//! file format used in the pokemon emerald decomp expansion project.

use regex::Regex;

use super::{Parties, Trainer};

static DEFAULT_TRAINER_REGEX_DELIMITER: &str =
    r"=== (?<name>[A-Z0-9_]+) ===\n(?<details>(?:[\w: /]+\n)+)\n+(?<mons>(?:[\w: /\n]+)*)";

static DEFAULT_TRAINER_FIELDS_REGEX: &str = r"(?<field>[\w ]+): (?<value>[\w ]+)";

pub struct ParsingConfig<'a> {
    TrainerRegex: &'a str,
    TrainerFieldsRegex: &'a str,
}

impl<'a> Default for ParsingConfig<'a> {
    fn default() -> Self {
        Self {
            TrainerRegex: DEFAULT_TRAINER_REGEX_DELIMITER,
            TrainerFieldsRegex: DEFAULT_TRAINER_FIELDS_REGEX,
        }
    }
}

fn parse_trainer_fields(content: &str, id: &str, config: &ParsingConfig) -> Trainer {
    let trainer_fields_re = Regex::new(config.TrainerFieldsRegex).unwrap();
    let mut trainer = Trainer::default();
    trainer.ID = id.to_owned();

    for (_, [field, value]) in trainer_fields_re
        .captures_iter(content)
        .map(|c| c.extract())
    {
        match field {
            "Name" => trainer.Name = value.to_owned(),
            "Class" => trainer.Class = Some(value.to_owned()),
            "Pic" => trainer.Pic = value.to_owned(),
            "Gender" => trainer.Gender = value.to_owned(),
            "Music" => trainer.Music = value.to_owned(),
            "Items" => trainer.Items = value.to_owned(),
            "Double Battle" => {
                trainer.DoubleBattle = match value {
                    "Yes" => true,
                    "No" => false,
                    _ => unreachable!(),
                }
            }
            "AI" => trainer.AI = Some(value.to_owned()),
            _ => tracing::error!(?field, "Unknown field found in trainer"),
        }
    }

    trainer
}

pub fn from_emerald_expansion_format_config(file_content: &str, config: &ParsingConfig) -> Parties {
    let trainer_re = Regex::new(config.TrainerRegex).unwrap();

    for (_, [id, fields, mons]) in trainer_re.captures_iter(file_content).map(|c| c.extract()) {
        let trainer = parse_trainer_fields(fields, id, config);
        tracing::debug!("{:?}", trainer);
        tracing::debug!("{}", mons);
    }

    unimplemented!()
}

pub fn from_emerald_expansion_format(file_content: &str) -> Parties {
    let parsing_config = ParsingConfig::default();
    from_emerald_expansion_format_config(file_content, &parsing_config)
}
