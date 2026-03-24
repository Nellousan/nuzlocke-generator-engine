//! This file contains the data layout describing the encounters.json
//! file present in the src/data directory in emerald's decomp expansion
//! project.

use std::collections::{HashMap, HashSet};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::database::pokedex::Pokedex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Encounters {
    pub wild_encounter_groups: Vec<WildEncounterGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WildEncounterGroup {
    pub label: String,
    pub for_maps: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WildEncounterGroupFields>>,
    pub encounters: Vec<MapEncounters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WildEncounterGroupFields {
    #[serde(rename = "type")]
    pub r#type: String,
    pub encounter_rates: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<WildEncounterGroupFieldsFishingGroups>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WildEncounterGroupFieldsFishingGroups {
    pub old_rod: Vec<u8>,
    pub good_rod: Vec<u8>,
    pub super_rod: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapEncounters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    pub base_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_mons: Option<MapEncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_mons: Option<MapEncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rock_smash_mons: Option<MapEncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fishing_mons: Option<MapEncounterSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapEncounterSet {
    pub encounter_rate: u8,
    pub mons: Vec<MapEncounterSetMon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapEncounterSetMon {
    pub min_level: u8,
    pub max_level: u8,
    pub species: String,
}

impl<R: Rng + ?Sized> crate::encounters::Encounters<R> for Encounters {
    fn randomize(&mut self, pokedex: &Pokedex, rng: &mut R) {
        for encounter_group in self.wild_encounter_groups.iter_mut() {
            for map_encouters in encounter_group.encounters.iter_mut() {
                if let Some(ref mut encounter_set) = map_encouters.land_mons {
                    let mut hash_set = HashSet::new();
                    for mon in encounter_set.mons.iter() {
                        hash_set.insert(mon.species.clone());
                    }

                    let mut replace_mon = vec![];
                    for species in hash_set.iter() {
                        // tracing::debug!(?species);
                        let mon_db_entry = pokedex
                            .get(
                                &species
                                    .replace("SPECIES_", "")
                                    .to_lowercase()
                                    .replace('-', "")
                                    .replace(' ', ""),
                            )
                            .unwrap();
                        let candidates = pokedex.get_all_within_bst_range(
                            mon_db_entry.base_stats.total(),
                            30,
                            30,
                        );

                        let chosen = candidates
                            .get(rng.next_u32() as usize % candidates.len())
                            .expect("modulo len");

                        replace_mon.push(chosen.clone());
                    }

                    let mut replace_map = HashMap::new();
                    for (to_replace, replacement) in hash_set.iter().zip(replace_mon.iter()) {
                        replace_map.insert(to_replace, replacement.name.clone());
                    }

                    for set in encounter_set.mons.iter_mut() {
                        set.species = format!(
                            "SPECIES_{}",
                            replace_map[&set.species]
                                .replace("é", "e")
                                .replace('’', "_")
                                .replace(". ", "_")
                                .replace('-', "_")
                                .replace(' ', "_")
                                .replace('.', "")
                                .to_uppercase()
                        );
                        if set.species.contains("FLAB") {
                            tracing::debug!(?set.species);
                        }
                    }
                }
            }
        }
    }
}
