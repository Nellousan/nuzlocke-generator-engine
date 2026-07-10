use serde::{Deserialize, Serialize};

use crate::cli::ProjectOption;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainerOrderEntry {
    pub id: String, // Project-dependent identifier (in trainers.party for emerald expansion, for example)
    pub split: String, // Which split the trainer belongs to, used for documentation
    pub sets_level_cap: Option<u8>, // Whether or not the trainer sets a new level cap
    pub optional: Option<bool>, // Whether or not the trainer is skippable
    pub location: Option<String>, // Additional information on where the trainer is located.
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainerOrder {
    pub starter_level_cap: u8,
    pub trainers: Vec<TrainerOrderEntry>,
}

pub fn load_trainer_order(project_options: &ProjectOption) -> eyre::Result<Option<TrainerOrder>> {
    match project_options {
        ProjectOption::EmeraldExpansion(ee_options) => {
            if ee_options.no_trainer_order {
                return Ok(None);
            }

            let content = std::fs::read_to_string(
                ee_options.project_path.join(&ee_options.trainer_order_path),
            )?;
            let trainer_order: TrainerOrder = toml::from_str(&content)?;

            Ok(Some(trainer_order))
        }
    }
}
