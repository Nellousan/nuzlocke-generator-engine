use std::{collections::HashMap, path::Path};

use rand::Rng;
use serde::Deserialize;

use crate::parties::party::{PokemonEVs, PokemonGender, PokemonIVs, PokemonSet};

pub type Species = String;

pub type SetBundle = HashMap<Species, Vec<PokemonBundleSet>>;

// TODO: Implement tera types, dynamax level
#[derive(Clone, Debug, Deserialize)]
pub struct PokemonBundleSet {
    pub format: String,
    pub name: String,
    pub moves: Vec<Vec<String>>,
    pub item: Vec<String>,
    pub nature: Vec<String>,
    #[serde(default)]
    pub evs: Option<Vec<PokemonEVs>>,
    #[serde(default)]
    pub ivs: Option<Vec<PokemonIVs>>,
}

impl PokemonBundleSet {
    pub fn generate_set<R: Rng + ?Sized>(
        &self,
        species: &str,
        level: u8,
        rng: &mut R,
    ) -> PokemonSet {
        let mut move_1 = None;
        let mut move_2 = None;
        let mut move_3 = None;
        let mut move_4 = None;

        for (i, moves) in self.moves.iter().enumerate() {
            let r#move = moves
                .get(rng.next_u32() as usize % moves.len())
                .expect("modulo len");
            match i {
                0 => move_1 = Some(r#move.clone()),
                1 => move_2 = Some(r#move.clone()),
                2 => move_3 = Some(r#move.clone()),
                3 => move_4 = Some(r#move.clone()),
                _ => unreachable!(),
            };
        }

        let held_item = if !self.item.is_empty() {
            self.item.get(rng.next_u32() as usize % self.item.len())
        } else {
            None
        };

        let _evs = if let Some(ref evs) = self.evs {
            Some(
                evs.get(rng.next_u32() as usize % evs.len())
                    .expect("modulo len")
                    .clone(),
            )
        } else {
            None
        };

        let nature = self
            .nature
            .get(rng.next_u32() as usize % self.nature.len())
            .expect("modulo len");

        PokemonSet {
            species: species.to_owned(),
            gender: PokemonGender::None,
            held_item: held_item.cloned(),
            level: Some(level),
            ivs: None, // TODO: Implement ivs
            evs: None, // TODO: implement evs
            ball: None,
            happiness: Some(255),
            nature: Some(nature.clone()),
            shiny: false,
            dynamax_level: None,
            gigantamax: false,
            tera_type: None,
            move_1,
            move_2,
            move_3,
            move_4,
        }
    }
}

pub fn load_bundle(path: &Path) -> eyre::Result<SetBundle> {
    let content = std::fs::read_to_string(path)?;
    let bundle: SetBundle = serde_json::from_str(&content)?;

    Ok(bundle)
}
