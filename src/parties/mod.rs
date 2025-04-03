//! This module provides the data representations for the list of trainers and
//! their Pokemon parties. This is the intermediate representation that stands
//! between the NGE config files and the game specific representation.

mod emerald_expansion;

mod party {
    pub enum PokemonGender {
        Male,
        Female,
        None,
    }

    // All defaults to 31
    pub struct PokemonIVs {
        Health: Option<u8>,
        Attack: Option<u8>,
        Defense: Option<u8>,
        SpAttack: Option<u8>,
        SpDefense: Option<u8>,
        Speed: Option<u8>,
    }

    // All defaults to 0
    pub struct PokemonEVs {
        Health: Option<u8>,
        Attack: Option<u8>,
        Defense: Option<u8>,
        SpAttack: Option<u8>,
        SpDefense: Option<u8>,
        Speed: Option<u8>,
    }

    pub struct Pokemon {
        Species: String,
        Gender: PokemonGender,
        HeldItem: Option<String>,
        Level: Option<u8>, // Defaults to 100
        IVs: PokemonIVs,
        EVs: PokemonEVs,
        Ball: Option<String>,  // Defaults to PokeBall
        Happiness: Option<u8>, // Defaults to 0
        Nature: Option<u8>,    // Defaults to Hardy
        Shiny: bool,           // Defaults to False

        DynamaxLevel: Option<u8>, // Defaults to 10
        Gigantamax: bool,         // Defaults to False

        TeraType: Option<String>, // Defaults to Normal
    }
}

pub struct Trainer {
    Name: String,
    Class: Option<String>, // Defaults to PkMn Trainer
    Pic: String,
    Music: String,
    Items: String,
    DoubleBattle: bool, // Defaults to False
    AI: Option<String>, // If applicable
    Party: [Option<party::Pokemon>; 6],
}

pub struct Parties(Vec<Trainer>);

impl std::ops::Deref for Parties {
    type Target = Vec<Trainer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
