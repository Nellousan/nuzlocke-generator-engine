use rand::Rng;

use crate::{cli::ProjectOption, database::pokedex::Pokedex};

pub mod emerald_expansion;

pub trait Encounters<R: Rng + ?Sized> {
    fn randomize(&mut self, pokedex: &Pokedex, rng: &mut R);
    // Mandatory due to serde::Serialize not being dyn-compatible
    fn serialize(&self) -> Result<String, serde_json::Error>;
}

pub fn load_encounter<R: Rng + ?Sized>(
    project_options: &ProjectOption,
) -> eyre::Result<Box<dyn Encounters<R>>> {
    match project_options {
        ProjectOption::EmeraldExpansion(ee_options) => {
            let parties_file_path = ee_options
                .project_path
                .join(&ee_options.encounters_file_path);

            let content = std::fs::read_to_string(parties_file_path)?;

            let encounters: emerald_expansion::Encounters = serde_json::from_str(&content)?;
            Ok(Box::new(encounters))
        }
    }
}
