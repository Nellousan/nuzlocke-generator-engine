pub mod pokedex {
    use std::{collections::HashMap, fs::read_to_string, path::Path};

    use serde::{Deserialize, Serialize};

    pub struct Pokedex(HashMap<String, PokemonDatabaseEntry>);

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

        pub fn is_within_range(&self, bst: u32, plus: u32, minus: u32) -> bool {
            let diff = i32::abs(self.total() as i32 - bst as i32);
            (diff as u32) < (plus + minus)
        }
    }

    impl std::ops::Deref for Pokedex {
        type Target = HashMap<String, PokemonDatabaseEntry>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for Pokedex {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Pokedex {
        // TODO: Return an iterator over reference instead ?
        pub fn get_all_within_bst_range(
            &self,
            bst: u32,
            plus: u32,
            minus: u32,
        ) -> Vec<PokemonDatabaseEntry> {
            let mut mons = vec![];

            for (_, value) in self.0.iter() {
                if value.base_stats.is_within_range(bst, plus, minus) {
                    mons.push(value.clone());
                }
            }

            mons
        }
    }

    pub fn load_pokedex(path: &Path) -> eyre::Result<Pokedex> {
        let content = read_to_string(path)?;
        let result: HashMap<_, _> = serde_json::from_str(&content)?;

        Ok(Pokedex(result))
    }
}
