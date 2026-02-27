pub mod pokedex {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    #[expect(dead_code)]
    pub type Pokedex = HashMap<String, PokemonDatabaseEntry>;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PokemonDatabaseEntry {
        pub num: u64,
        pub name: String,
        #[serde(rename = "baseSpecies")]
        pub base_species: Option<String>,
        pub forme: Option<String>,
        pub types: Vec<String>,
        #[serde(rename = "baseStats")]
        pub base_stats: PokemonDatabaseEntryBaseStats,
        pub abilities: HashMap<String, String>,
        pub height: Option<f32>,
        pub weightkg: f32,
        pub color: String,
        pub prevo: Option<String>,
        #[serde(rename = "evoLevel")]
        pub evo_level: Option<u8>,
        pub evos: Option<Vec<String>>,
        pub r#gen: Option<u8>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PokemonDatabaseEntryBaseStats {
        pub hp: u8,
        pub atk: u8,
        pub def: u8,
        pub spa: u8,
        pub spd: u8,
        pub spe: u8,
    }

    #[expect(dead_code)]
    impl PokemonDatabaseEntryBaseStats {
        pub fn total(&self) -> u32 {
            self.hp as u32
                + self.atk as u32
                + self.def as u32
                + self.spa as u32
                + self.spd as u32
                + self.spe as u32
        }
    }
}
