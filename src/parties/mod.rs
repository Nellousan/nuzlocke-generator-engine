//! This module provides the data representations for the list of trainers and
//! their Pokemon parties. This is the intermediate representation that stands
//! between the NGE config files and the game specific representation.

use std::collections::HashMap;

pub mod emerald_expansion;
pub mod error;

pub mod party {
    use super::error::PartyError;

    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PokemonGender {
        #[default]
        None,
        Male,
        Female,
    }

    impl TryFrom<&str> for PokemonGender {
        type Error = PartyError;
        fn try_from(value: &str) -> Result<Self, PartyError> {
            match value {
                "" => Ok(Self::None),
                "M" => Ok(Self::Male),
                "F" => Ok(Self::Female),
                _ => Err(PartyError::ParsingError(format!(
                    "Could not convert {} to PokemonGender",
                    value
                ))),
            }
        }
    }

    impl From<PokemonGender> for String {
        fn from(value: PokemonGender) -> Self {
            match value {
                PokemonGender::None => "".to_owned(),
                PokemonGender::Male => "M".to_owned(),
                PokemonGender::Female => "F".to_owned(),
            }
        }
    }

    // All defaults to 31
    #[derive(Default, Debug)]
    #[expect(dead_code)]
    pub struct PokemonIVs {
        health: Option<u8>,
        attack: Option<u8>,
        defense: Option<u8>,
        sp_attack: Option<u8>,
        sp_defense: Option<u8>,
        speed: Option<u8>,
    }

    // All defaults to 0
    #[derive(Default, Debug)]
    #[expect(dead_code)]
    pub struct PokemonEVs {
        health: Option<u8>,
        attack: Option<u8>,
        defense: Option<u8>,
        sp_attack: Option<u8>,
        sp_defense: Option<u8>,
        speed: Option<u8>,
    }

    #[derive(Default, Debug)]
    #[expect(dead_code)]
    pub struct Pokemon {
        // nickname: String,
        pub species: String,
        pub gender: PokemonGender,
        pub held_item: Option<String>,
        pub level: Option<u8>, // Defaults to 100
        // TODO: This is temporary. Parse ivs and evs properly.
        pub ivs: Option<String>,    // PokemonIVs,
        pub evs: Option<String>,    // PokemonEVs,
        pub ball: Option<String>,   // Defaults to PokeBall
        pub happiness: Option<u8>,  // Defaults to 0
        pub nature: Option<String>, // Defaults to Hardy
        pub shiny: bool,            // Defaults to False

        pub dynamax_level: Option<u8>, // Defaults to 10
        pub gigantamax: bool,          // Defaults to False

        pub tera_type: Option<String>, // Defaults to Normal

        pub move_1: Option<String>,
        pub move_2: Option<String>,
        pub move_3: Option<String>,
        pub move_4: Option<String>,
    }
}

#[derive(Default, Debug)]
pub struct Trainer {
    pub id: String,
    pub name: String,
    pub pic: String,
    pub class: Option<String>, // Defaults to PkMn Trainer
    pub gender: Option<String>,
    pub music: Option<String>,
    pub items: Option<String>,
    pub double_battle: bool,
    pub ai: Option<String>, // If applicable
    pub mugshot: Option<String>,
    pub starting_status: Option<String>,
    pub party: [Option<party::Pokemon>; 6],
}

#[derive(Default, Debug)]
pub struct Parties {
    parties: Vec<Trainer>,
    name_map: HashMap<String, usize>, // Map of trainer id to the index
}

impl Parties {
    pub fn new(trainers: Vec<Trainer>) -> Self {
        let mut res = Self {
            parties: trainers,
            name_map: HashMap::new(),
        };

        for (i, trainer) in res.parties.iter().enumerate() {
            res.name_map.insert(trainer.id.clone(), i);
        }

        res
    }

    #[expect(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Trainer> {
        self.parties.get(*self.name_map.get(key)?)
    }

    #[expect(dead_code)]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Trainer> {
        self.parties.get_mut(*self.name_map.get(key)?)
    }
}

impl std::ops::Index<&str> for Parties {
    type Output = Trainer;
    fn index(&self, index: &str) -> &Self::Output {
        self.parties
            .get(*self.name_map.get(index).unwrap())
            .unwrap()
    }
}

impl std::ops::IndexMut<&str> for Parties {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.parties
            .get_mut(*self.name_map.get(index).unwrap())
            .unwrap()
    }
}

impl std::ops::Deref for Parties {
    type Target = Vec<Trainer>;

    fn deref(&self) -> &Self::Target {
        &self.parties
    }
}

impl std::ops::DerefMut for Parties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parties
    }
}
