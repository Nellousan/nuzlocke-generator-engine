//! This file contains functions to parse and write from/into the .parties
//! file format used in the pokemon emerald decomp expansion project.

use regex::{Captures, Regex};

use super::{Parties, Trainer, error::PartyError, party::Pokemon};

static DEFAULT_TRAINER_REGEX_DELIMITER: &str =
    r"=== (?<name>[A-Z0-9_]+) ===\n(?<details>(?:[\w: /]+\n)+)\n+(?<mons>(?:[\w: /\n]+)*)";

// TODO: Improve so that trailing spaces before \n are ignored
/// https://regex101.com/r/tuREWJ
static DEFAULT_TRAINER_FIELDS_REGEX: &str = r"(?:(?:Name: ?(?<name>[\w ]+)?\n?)|(?:Class: (?<class>[\w ]+)\n?)|(?:Pic: (?<pic>[\w ]+)\n?)|(?:^Gender: (?<gender>[\w ]+)\n?)|(?:Music: (?<music>[\w ]+)\n?)|(?:Items: (?<items>[\w /]+)\n?)|(?:Double Battle: (?<double_battle>[\w ]+)\n?)|(?:AI: (?<ai>[\w ]+)\n?)|(?:Mugshot: (?<mugshot>[\w ]+)\n?)|(?:Starting Status: (?<starting_status>[\w ]+)\n?))+";

// TODO: Improve to handle Nicknames and Happiness
/// https://regex101.com/r/oC2CeP
static DEFAULT_POKEMON_FIELDS_REGEX: &str = r"(?<species>[\w-]+)(?: (?:\((?<gender>[MF])\))? ?(?:@ (?<item>[\w ]+)))?\n(?:(?:Level+: (?<level>[0-9]+\s*))\n|(?:Ability: (?<ability>[\w ]+\s*))\n|(?:Tera Type: (?<tera_type>[\w]+\s*))\n|(?:EVs: (?<effort_values>[\w/ ]+\s*))\n|(?:IVs: (?<individual_values>[\w/ ]+\s*))\n|(?:Shiny: (?<Shiny>[\w]+\s*))\n|(?:Ball: (?<Ball>[\w]+\s*))\n|(?:(?<nature>[\w]+) Nature[\s]*\n))+(?:- (?<move_1>[\w ]+)\n?)?(?:- (?<move_2>[\w ]+)\n?)?(?:- (?<move_3>[\w ]+)\n?)?(?:- (?<move_4>[\w ]+)\n?)?";

pub struct ParsingConfig<'a> {
    trainer_regex: &'a str,
    trainer_fields_regex: &'a str,
    pokemon_fields_regex: &'a str,
}

impl<'a> Default for ParsingConfig<'a> {
    fn default() -> Self {
        Self {
            trainer_regex: DEFAULT_TRAINER_REGEX_DELIMITER,
            trainer_fields_regex: DEFAULT_TRAINER_FIELDS_REGEX,
            pokemon_fields_regex: DEFAULT_POKEMON_FIELDS_REGEX,
        }
    }
}

fn cap_get_or_none(cap: &Captures, field: &str) -> Option<String> {
    cap.name(field)
        .map_or(None, |c| Some(c.as_str().to_string()))
}

fn parse_mons_fields(content: &str, re: &Regex) -> Result<[Option<Pokemon>; 6], PartyError> {
    let mut mons: [Option<Pokemon>; 6] = Default::default();

    for (i, cap) in re.captures_iter(content).enumerate() {
        if i > 5 {
            let error = "More than 6 pokemon found in party";
            tracing::error!(error);
            return Err(PartyError::ParsingError(error.to_owned()));
        }
        let mut mon = Pokemon::default();
        mon.species = cap_get_or_none(&cap, "species").ok_or(PartyError::ParsingError(
            "Error parsing Pokemon species.".to_owned(),
        ))?;
        // This is weird, maybe rework the gender field into an option and remove
        // the None variant in PokemonGender
        mon.gender = cap_get_or_none(&cap, "gender")
            .unwrap_or("".to_owned())
            .as_str()
            .try_into()?;
        mon.held_item = cap_get_or_none(&cap, "held_item");
        mon.level = cap_get_or_none(&cap, "level")
            .and_then(|lv| Some(lv.parse()))
            .transpose()?;
        mon.ivs = cap_get_or_none(&cap, "individual_values");
        mon.evs = cap_get_or_none(&cap, "effort_values");
        mon.ball = cap_get_or_none(&cap, "ball");
        mon.happiness = cap_get_or_none(&cap, "happiness")
            .and_then(|h| Some(h.parse()))
            .transpose()?;
        mon.nature = cap_get_or_none(&cap, "nature");
        mon.shiny = if cap_get_or_none(&cap, "shiny").unwrap_or("No".to_owned()) == "Yes" {
            true
        } else {
            false
        };
        // TODO: dynamax and gigantamax
        mon.tera_type = cap_get_or_none(&cap, "tera_type");

        mons[i] = Some(mon);
    }

    Ok(mons)
}

fn parse_trainer_fields(content: &str, id: &str, re: &Regex) -> Result<Trainer, PartyError> {
    let mut trainer = Trainer::default();
    trainer.id = id.to_owned();

    let Some(cap) = re.captures(content) else {
        let error = "Could not parse trainer fields, regex did not match";
        tracing::error!(error);
        return Err(PartyError::ParsingError(error.to_owned()));
    };

    trainer.name = cap_get_or_none(&cap, "name").unwrap_or_default();
    trainer.pic = cap_get_or_none(&cap, "pic").unwrap_or_default();
    trainer.class = cap_get_or_none(&cap, "class");
    trainer.gender = cap_get_or_none(&cap, "gender");
    trainer.music = cap_get_or_none(&cap, "music");
    trainer.items = cap_get_or_none(&cap, "items");
    trainer.double_battle =
        if cap_get_or_none(&cap, "double_battle").unwrap_or("No".to_owned()) == "Yes" {
            true
        } else {
            false
        };
    trainer.ai = cap_get_or_none(&cap, "ai");
    trainer.mugshot = cap_get_or_none(&cap, "mugshot");
    trainer.starting_status = cap_get_or_none(&cap, "starting_status");

    Ok(trainer)
}

pub fn from_emerald_expansion_format_config(
    file_content: &str,
    config: &ParsingConfig,
) -> Result<Parties, PartyError> {
    let trainer_re = Regex::new(config.trainer_regex).unwrap();
    let trainer_fields_re = Regex::new(config.trainer_fields_regex).unwrap();
    let pokemon_fields_re = Regex::new(config.pokemon_fields_regex).unwrap();

    let mut parties = vec![];

    for (_, [id, fields, mons]) in trainer_re.captures_iter(file_content).map(|c| c.extract()) {
        let mut trainer = parse_trainer_fields(fields, id, &trainer_fields_re)?;
        let mons = parse_mons_fields(mons, &pokemon_fields_re)?;
        trainer.party = mons;
        tracing::debug!("{:?}", trainer);

        parties.push(trainer);
    }

    Ok(Parties(parties))
}

pub fn from_emerald_expansion_format(file_content: &str) -> Result<Parties, PartyError> {
    let parsing_config = ParsingConfig::default();
    Ok(from_emerald_expansion_format_config(
        file_content,
        &parsing_config,
    )?)
}
