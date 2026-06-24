use std::path::PathBuf;

use askama::Template;

use crate::parties::{Trainer, party};

#[derive(Template)]
#[template(path = "trainer.jinja", escape = "none")]
pub struct TrainerTemplate {
    name: String,
    pic: PathBuf,
    party: [Option<party::PokemonSet>; 6],
}

impl From<Trainer> for TrainerTemplate {
    fn from(value: Trainer) -> Self {
        Self {
            name: format!(
                "{} {}",
                value.class.unwrap_or("Trainer".to_string()),
                value.name
            ),
            pic: value.pic.to_lowercase().replace(' ', "_").into(),
            party: value.party,
        }
    }
}

#[derive(Template)]
#[template(path = "trainers.jinja", escape = "none")]
pub struct TrainerListTemplate {
    trainer_templates: Vec<TrainerTemplate>,
}

impl From<Vec<TrainerTemplate>> for TrainerListTemplate {
    fn from(value: Vec<TrainerTemplate>) -> Self {
        Self {
            trainer_templates: value,
        }
    }
}
