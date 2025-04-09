//! This file contains the data layout describing the encounters.json
//! file present in the src/data directory in emerald's decomp expansion
//! project.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterFile {
    pub wild_encounter_groups: Vec<WildEncounterGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WildEncounterGroup {
    pub label: String,
    pub for_maps: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<WildEncounterGroupFields>>,
    pub encounters: Vec<Encounters>,
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
pub struct Encounters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    pub base_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land_mons: Option<EncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_mons: Option<EncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rock_smash_mons: Option<EncounterSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fishing_mons: Option<EncounterSet>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterSet {
    pub encounter_rate: u8,
    pub mons: Vec<EncounterSetMon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncounterSetMon {
    pub min_level: u8,
    pub max_level: u8,
    pub species: String,
}
