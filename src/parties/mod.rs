//! This module provides the data representations for the list of trainers and
//! their Pokemon parties. This is the intermediate representation that stands
//! between the NGE config files and the game specific representation.

pub mod emerald_expansion;
pub mod error;

pub mod party {
    use super::error::PartyError;

    #[derive(Default, Debug)]
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
        pub level: Option<u8>,      // Defaults to 100
        pub ivs: Option<String>,    // PokemonIVs,
        pub evs: Option<String>,    // PokemonEVs,
        pub ball: Option<String>,   // Defaults to PokeBall
        pub happiness: Option<u8>,  // Defaults to 0
        pub nature: Option<String>, // Defaults to Hardy
        pub shiny: bool,            // Defaults to False

        pub dynamax_level: Option<u8>, // Defaults to 10
        pub gigantamax: bool,          // Defaults to False

        pub tera_type: Option<String>, // Defaults to Normal
    }
}

#[derive(Default, Debug)]
#[expect(dead_code)]
pub struct Trainer {
    id: String,
    name: String,
    pic: String,
    class: Option<String>, // Defaults to PkMn Trainer
    gender: Option<String>,
    music: Option<String>,
    items: Option<String>,
    double_battle: bool,
    ai: Option<String>, // If applicable
    mugshot: Option<String>,
    starting_status: Option<String>,
    party: [Option<party::Pokemon>; 6],
}

#[derive(Default, Debug)]
pub struct Parties(Vec<Trainer>);

impl std::ops::Deref for Parties {
    type Target = Vec<Trainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
