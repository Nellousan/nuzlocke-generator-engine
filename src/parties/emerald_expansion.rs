//! This file contains functions to parse and write from/into the .parties
//! file format used in the pokemon emerald decomp expansion project.

use regex::{Captures, Regex};

use crate::parties::party::{PokemonGender, PokemonIVs};

use super::{Parties, Trainer, error::PartyError, party::PokemonSet};

//
// ------ Parsing stuff
//

static DEFAULT_TRAINER_REGEX_DELIMITER: &str =
    r"=== (?<name>[A-Z0-9_]+) ===\n(?<details>(?:[\w: /]+\n)+)\n+(?<mons>(?:[\w: /@\-\n]+)*)";

// TODO: Improve so that trailing spaces before \n are ignored
/// https://regex101.com/r/tuREWJ
static DEFAULT_TRAINER_FIELDS_REGEX: &str = r"(?:(?:Name: ?(?<name>[\w ]+)?\n?)|(?:Class: (?<class>[\w ]+)\n?)|(?:Pic: (?<pic>[\w ]+)\n?)|(?:^Gender: (?<gender>[\w ]+)\n?)|(?:Music: (?<music>[\w ]+)\n?)|(?:Items: (?<items>[\w /]+)\n?)|(?:Double Battle: (?<double_battle>[\w ]+)\n?)|(?:AI: (?<ai>[\w ]+)\n?)|(?:Mugshot: (?<mugshot>[\w ]+)\n?)|(?:Starting Status: (?<starting_status>[\w ]+)\n?))+";

// TODO: Improve to handle Nicknames and Happiness
/// https://regex101.com/r/oC2CeP
/// https://regex101.com/r/2v9kpN/1
static DEFAULT_POKEMON_FIELDS_REGEX: &str = r"(?<species>[\w :-]+)(?: (?:\((?<gender>[MF])\))? ?(?:@ (?<item>[\w\- ]+)))?\n(?:(?:Level+: (?<level>[0-9]+\s*))\n|(?:Happiness+: (?<happiness>[0-9]+\s*))\n|(?:Ability: (?<ability>[\w -]+\s*))\n|(?:Tera Type: (?<tera_type>[\w]+\s*))\n|(?:EVs: (?<effort_values>[\w/ ]+\s*))\n|(?:IVs: (?<individual_values>[\w/ ]+\s*))\n|(?:Shiny: (?<Shiny>[\w]+\s*))\n|(?:Ball: (?<Ball>[\w]+\s*))\n|(?:(?<nature>[\w]+) Nature[\s]*\n))+(?:- (?<move_1>[\w\- ]+)\n?)?(?:- (?<move_2>[\w\- ]+)\n?)?(?:- (?<move_3>[\w\- ]+)\n?)?(?:- (?<move_4>[\w\- ]+)\n?)?";

/// https://regex101.com/r/GMoBaW/1
static DEFAULT_POKEMON_IVS_EVS_FIELD_REGEX: &str = r"(?:(?<hp>[0-9]+) HP(?: / )?)?(?:(?<atk>[0-9]+) Atk(?: / )?)?(?:(?<def>[0-9]+) Def(?: / )?)?(?:(?<spa>[0-9]+) SpA(?: / )?)?(?:(?<spd>[0-9]+) SpD(?: / )?)?(?:(?<spe>[0-9]+) Spe(?: / )?)?";

pub struct ParsingConfig<'a> {
    trainer_regex: &'a str,
    trainer_fields_regex: &'a str,
    pokemon_fields_regex: &'a str,
    pokemon_ivs_evs_field_regex: &'a str,
}

impl<'a> Default for ParsingConfig<'a> {
    fn default() -> Self {
        Self {
            trainer_regex: DEFAULT_TRAINER_REGEX_DELIMITER,
            trainer_fields_regex: DEFAULT_TRAINER_FIELDS_REGEX,
            pokemon_fields_regex: DEFAULT_POKEMON_FIELDS_REGEX,
            pokemon_ivs_evs_field_regex: DEFAULT_POKEMON_IVS_EVS_FIELD_REGEX,
        }
    }
}

fn parse_ivs_evs(content: &str, re: &Regex) -> eyre::Result<PokemonIVs> {
    let Some(cap) = re.captures(content) else {
        let error = "Could not parse IVs or EVs fields, regex did not match";
        tracing::error!(error);
        return Err(PartyError::ParsingError(error.to_owned()))?;
    };

    let health = cap_get_or_none(&cap, "hp")
        .and_then(|v| Some(v.parse()))
        .transpose()?;
    let attack = cap_get_or_none(&cap, "atk")
        .and_then(|v| Some(v.parse()))
        .transpose()?;
    let defense = cap_get_or_none(&cap, "def")
        .and_then(|v| Some(v.parse()))
        .transpose()?;
    let sp_attack = cap_get_or_none(&cap, "spa")
        .and_then(|v| Some(v.parse()))
        .transpose()?;
    let sp_defense = cap_get_or_none(&cap, "spd")
        .and_then(|v| Some(v.parse()))
        .transpose()?;
    let speed = cap_get_or_none(&cap, "spe")
        .and_then(|v| Some(v.parse()))
        .transpose()?;

    Ok(PokemonIVs {
        health,
        attack,
        defense,
        sp_attack,
        sp_defense,
        speed,
    })
}

fn cap_get_or_none(cap: &Captures, field: &str) -> Option<String> {
    cap.name(field)
        .map_or(None, |c| Some(c.as_str().trim_end().to_owned()))
}

fn parse_mons_fields(
    content: &str,
    re: &Regex,
    ivs_evs_re: &Regex,
) -> eyre::Result<[Option<PokemonSet>; 6]> {
    let mut mons: [Option<PokemonSet>; 6] = Default::default();

    for (i, cap) in re.captures_iter(content).enumerate() {
        if i > 5 {
            let error = "More than 6 pokemon found in party";
            tracing::debug!(?mons);
            tracing::error!(error);
            return Err(PartyError::ParsingError(error.to_owned()))?;
        }
        let species = cap_get_or_none(&cap, "species").ok_or(PartyError::ParsingError(
            "Error parsing Pokemon species.".to_owned(),
        ))?;
        let species_normalized = unidecode::unidecode(&species)
            .to_lowercase()
            .replace('_', "")
            .replace('\'', "")
            .replace(". ", "_")
            .replace('-', "_")
            .replace(' ', "_")
            .replace('.', "")
            .replace(':', "");
        // This is weird, maybe rework the gender field into an option and remove
        // the None variant in PokemonGender
        let gender = cap_get_or_none(&cap, "gender")
            .unwrap_or("".to_owned())
            .as_str()
            .try_into()?;
        let held_item = cap_get_or_none(&cap, "held_item");
        let level = cap_get_or_none(&cap, "level")
            .and_then(|lv| Some(lv.parse()))
            .transpose()?;
        let ivs = cap_get_or_none(&cap, "individual_values")
            .and_then(|ivs| Some(parse_ivs_evs(&ivs, ivs_evs_re)))
            .transpose()?;
        let evs = cap_get_or_none(&cap, "effort_values")
            .and_then(|evs| Some(parse_ivs_evs(&evs, ivs_evs_re)))
            .transpose()?;
        let ball = cap_get_or_none(&cap, "ball");
        let ability = cap_get_or_none(&cap, "ability");
        let happiness = cap_get_or_none(&cap, "happiness")
            .and_then(|h| Some(h.parse()))
            .transpose()?;
        let nature = cap_get_or_none(&cap, "nature");
        let shiny = if cap_get_or_none(&cap, "shiny").unwrap_or("No".to_owned()) == "Yes" {
            true
        } else {
            false
        };
        // TODO: dynamax and gigantamax
        let tera_type = cap_get_or_none(&cap, "tera_type");

        let move_1 = cap_get_or_none(&cap, "move_1");
        let move_2 = cap_get_or_none(&cap, "move_2");
        let move_3 = cap_get_or_none(&cap, "move_3");
        let move_4 = cap_get_or_none(&cap, "move_4");

        let mon = PokemonSet {
            species,
            species_normalized,
            gender,
            held_item,
            level,
            ivs,
            evs,
            ball,
            ability,
            happiness,
            nature,
            shiny,
            dynamax_level: None,
            gigantamax: false,
            tera_type,
            move_1,
            move_2,
            move_3,
            move_4,
        };

        mons[i] = Some(mon);
    }

    Ok(mons)
}

fn parse_trainer_fields(content: &str, id: &str, re: &Regex) -> eyre::Result<Trainer> {
    let mut trainer = Trainer::default();
    trainer.id = id.to_owned();

    let Some(cap) = re.captures(content) else {
        let error = "Could not parse trainer fields, regex did not match";
        tracing::error!(error);
        return Err(PartyError::ParsingError(error.to_owned()).into());
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
) -> eyre::Result<Parties> {
    let trainer_re = Regex::new(config.trainer_regex).expect("regex is valid");
    let trainer_fields_re = Regex::new(config.trainer_fields_regex).expect("regex is valid");
    let pokemon_fields_re = Regex::new(config.pokemon_fields_regex).expect("regex is valid");
    let pokemon_iv_ev_re = Regex::new(config.pokemon_ivs_evs_field_regex).expect("regex is valid");

    let mut trainers = vec![];

    for (_, [id, fields, mons]) in trainer_re.captures_iter(file_content).map(|c| c.extract()) {
        let mut trainer = parse_trainer_fields(fields, id, &trainer_fields_re)?;
        let mons = parse_mons_fields(mons, &pokemon_fields_re, &pokemon_iv_ev_re)?;
        trainer.party = mons;

        trainers.push(trainer);
    }

    Ok(Parties::new(trainers))
}

pub fn from_emerald_expansion_format(file_content: &str) -> eyre::Result<Parties> {
    let parsing_config = ParsingConfig::default();
    from_emerald_expansion_format_config(file_content, &parsing_config)
}

//
// ------- Writing Stuff
//

/// Quick helper to not have to put newline on every append
pub trait PushLn {
    fn push_ln(&mut self, string: &str);
}

impl PushLn for String {
    fn push_ln(&mut self, string: &str) {
        let result = format!("{}\n", string);
        self.push_str(&result);
    }
}

fn write_mons_field(mons: &[Option<PokemonSet>; 6]) -> Result<String, PartyError> {
    let mut result = String::new();
    let push_field_if_some = |field: &Option<String>, name: &str, res: &mut String| {
        if let Some(value) = field {
            let line = format!("{}: {}", name.trim_end(), value.trim_end());
            res.push_ln(&line);
        }
    };

    for mon in mons {
        let Some(pokemon) = mon else {
            continue;
        };

        let mut mon_fields = String::new();

        let species = format!("{}", pokemon.species);
        mon_fields.push_str(&species);

        if pokemon.gender != PokemonGender::None {
            let gender = format!(" ({})", String::from(pokemon.gender));
            mon_fields.push_str(&gender);
        }

        if let Some(ref value) = pokemon.held_item {
            let held_item = format!(" @ {}", value);
            mon_fields.push_str(&held_item);
        }

        mon_fields.push_str("\n");

        if let Some(value) = pokemon.level {
            let level = format!("Level: {}", value);
            mon_fields.push_ln(&level);
        }

        let push_ivs_if_some = |ivs: &Option<PokemonIVs>, prepend: &str, res: &mut String| {
            if let Some(value) = ivs {
                let push_stat =
                    |stat: &Option<u8>, stat_str: &str, first: &mut bool, res: &mut String| {
                        if let Some(stat) = stat {
                            if *first {
                                *first = false;
                                *res = format!("{} {} {}", res, stat, stat_str);
                            } else {
                                *res = format!("{} / {} {}", res, stat, stat_str);
                            }
                        }
                    };

                let mut iv_result = format!("{}:", prepend);
                let mut first = true;
                push_stat(&value.health, "HP", &mut first, &mut iv_result);
                push_stat(&value.attack, "Atk", &mut first, &mut iv_result);
                push_stat(&value.defense, "Def", &mut first, &mut iv_result);
                push_stat(&value.sp_attack, "SpA", &mut first, &mut iv_result);
                push_stat(&value.sp_defense, "SpD", &mut first, &mut iv_result);
                push_stat(&value.speed, "Spe", &mut first, &mut iv_result);
                res.push_ln(&iv_result);
            }
        };

        // Temporary
        push_ivs_if_some(&pokemon.ivs, "IVs", &mut mon_fields);
        push_ivs_if_some(&pokemon.evs, "EVs", &mut mon_fields);

        push_field_if_some(&pokemon.ball, "Ball", &mut mon_fields);

        push_field_if_some(&pokemon.ability, "Ability", &mut mon_fields);

        if let Some(value) = pokemon.happiness {
            let happiness = format!("Happiness: {}", value);
            mon_fields.push_ln(&happiness);
        }

        if let Some(ref value) = pokemon.nature {
            let nature = format!("{} Nature", value);
            mon_fields.push_ln(&nature);
        }

        // TODO: dynamax & gigantamax
        push_field_if_some(&pokemon.tera_type, "Tera Type", &mut mon_fields);

        let push_move_if_some = |r#move: &Option<String>, res: &mut String| {
            if let Some(value) = r#move {
                let line = format!("- {}", value);
                res.push_ln(&line);
            }
        };

        push_move_if_some(&pokemon.move_1, &mut mon_fields);
        push_move_if_some(&pokemon.move_2, &mut mon_fields);
        push_move_if_some(&pokemon.move_3, &mut mon_fields);
        push_move_if_some(&pokemon.move_4, &mut mon_fields);

        result.push_ln(&mon_fields);
    }

    Ok(result)
}

fn write_trainer_fields(trainer: &Trainer) -> Result<String, PartyError> {
    let mut result = String::new();
    let push_field_if_some = |field: &Option<String>, name: &str, res: &mut String| {
        if let Some(value) = field {
            let line = format!("{}: {}", name.trim_end(), value.trim_end());
            res.push_ln(&line);
        }
    };

    let line = format!("Name: {}", trainer.name);
    result.push_ln(&line);

    let line = format!("Pic: {}", trainer.pic);
    result.push_ln(&line);

    push_field_if_some(&trainer.class, "Class", &mut result);
    push_field_if_some(&trainer.gender, "Gender", &mut result);
    push_field_if_some(&trainer.music, "Music", &mut result);
    push_field_if_some(&trainer.items, "Items", &mut result);

    result.push_ln("AI: Smart Trainer");

    let line = format!(
        "Double Battle: {}",
        if trainer.double_battle { "Yes" } else { "No" }
    );
    result.push_ln(&line);

    // push_field_if_some(&trainer.ai, "AI", &mut result);
    push_field_if_some(&trainer.mugshot, "Mugshot", &mut result);
    push_field_if_some(&trainer.starting_status, "Starting Status", &mut result);

    Ok(result)
}

pub fn to_emerald_expansion_format(parties: &Parties) -> Result<String, PartyError> {
    let mut result = String::new();
    for trainer in parties.iter() {
        let line = format!("=== {} ===", trainer.id);
        result.push_ln(&line);
        let fields = write_trainer_fields(trainer)?;
        result.push_ln(&fields);

        let mons = write_mons_field(&trainer.party)?;
        result.push_str(&mons);
    }

    Ok(result)
}
