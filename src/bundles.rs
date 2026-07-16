use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rand::Rng;
use serde::Deserialize;

use crate::{
    database::pokedex::PokemonDatabaseEntry,
    parties::party::{PokemonEVs, PokemonGender, PokemonIVs, PokemonSet},
};

pub type Species = String;

pub type SetBundle = HashMap<Species, Vec<PokemonBundleSet>>;

// TODO: Implement tera types, dynamax level
#[derive(Clone, Debug, Deserialize)]
pub struct PokemonBundleSet {
    #[expect(dead_code)]
    pub format: String,
    #[expect(dead_code)]
    pub name: String,
    pub moves: Vec<Vec<String>>,
    pub item: Vec<String>,
    pub nature: Option<Vec<String>>,
    pub ability: Option<Vec<String>>,
    #[serde(default)]
    pub evs: Option<Vec<PokemonEVs>>,
    #[serde(default)]
    pub ivs: Option<Vec<PokemonIVs>>,
    #[serde(rename = "teratypes")]
    pub tera_types: Option<Vec<String>>,
}

impl PokemonBundleSet {
    fn pick_one_if_some<R: Rng + ?Sized, T: Clone>(
        field: &Option<Vec<T>>,
        rng: &mut R,
    ) -> Option<T> {
        if let Some(content) = field {
            Some(
                content
                    .get(rng.next_u32() as usize % content.len())
                    .expect("modulo len")
                    .clone(),
            )
        } else {
            None
        }
    }

    pub fn generate_set<R: Rng + ?Sized>(
        &self,
        db_entry: &PokemonDatabaseEntry,
        level: u8,
        rng: &mut R,
        disable_evs: bool,
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
                _ => panic!("More than four moves in bundle set ???"),
            };
        }

        let held_item = if !self.item.is_empty() {
            self.item.get(rng.next_u32() as usize % self.item.len())
        } else {
            None
        };

        let evs = if !disable_evs {
            Self::pick_one_if_some(&self.evs, rng)
        } else {
            None
        };
        let ivs = Self::pick_one_if_some(&self.ivs, rng);

        let nature = Self::pick_one_if_some(&self.nature, rng);
        let mut ability = Self::pick_one_if_some(&self.ability, rng);
        if let None = ability {
            ability = Some(db_entry.abilities["0"].clone())
        }

        let tera_type = Self::pick_one_if_some(&self.tera_types, rng);

        PokemonSet {
            species: db_entry.name.clone(),
            species_normalized: unidecode::unidecode(&db_entry.name)
                .to_lowercase()
                .replace('_', "")
                .replace('\'', "")
                .replace(". ", "_")
                .replace('-', "_")
                .replace(' ', "_")
                .replace('.', "")
                .replace(':', ""),
            gender: PokemonGender::None,
            held_item: held_item.cloned(),
            level: Some(level),
            ivs,
            evs,
            ball: None,
            ability,
            happiness: Some(255),
            nature,
            shiny: false,
            dynamax_level: None,
            gigantamax: false,
            tera_type,
            move_1,
            move_2,
            move_3,
            move_4,
        }
    }
}

#[expect(dead_code)]
pub fn load_bundle(path: &Path) -> eyre::Result<SetBundle> {
    let content = std::fs::read_to_string(path)?;
    let bundle: SetBundle = serde_json::from_str(&content)?;

    Ok(bundle)
}

pub fn load_bundles(paths: impl AsRef<[PathBuf]>) -> eyre::Result<SetBundle> {
    let mut bundles = vec![];

    for path in paths.as_ref().iter() {
        let content = std::fs::read_to_string(path)?;
        let bundle: SetBundle = serde_json::from_str(&content)?;

        bundles.push(bundle);
    }

    let mut merged_bundles: SetBundle = HashMap::new();

    for bundle in bundles.iter() {
        for (key, value) in bundle.iter() {
            if merged_bundles.contains_key(key) {
                merged_bundles
                    .get_mut(key)
                    .expect("Contains checked")
                    .extend(value.clone());
            } else {
                merged_bundles.insert(key.to_owned(), value.clone());
            }
        }
    }

    Ok(merged_bundles)
}
