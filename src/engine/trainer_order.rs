use serde::{Deserialize, Serialize};

use crate::cli::ProjectOption;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainerOrderEntry {
    trainer_id: String, // Project-dependent identifier (in trainers.party for emerald expansion, for example)
    split: String,      // Which split the trainer belongs to, used for documentation
    sets_level_cap: Option<u8>, // Whether or not the trainer sets a new level cap
    optional: Option<bool>, // Whether or not the trainer is skippable
    location: Option<String>, // Additional information on where the trainer is located.
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrainerOrder(Vec<TrainerOrderEntry>);

impl std::ops::Deref for TrainerOrder {
    type Target = Vec<TrainerOrderEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for TrainerOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn load_trainer_order(project_options: &ProjectOption) -> eyre::Result<Option<TrainerOrder>> {
    match project_options {
        ProjectOption::EmeraldExpansion(ee_options) => {
            if ee_options.no_trainer_order {
                return Ok(None);
            }
            let content = std::fs::read_to_string(&ee_options.trainer_order_path)?;
            let trainer_order: TrainerOrder = toml::from_str(&content)?;

            Ok(Some(trainer_order))
        }
    }
}
