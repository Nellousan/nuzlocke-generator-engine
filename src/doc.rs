use std::path::PathBuf;

use askama::Template;

use crate::parties::{Trainer, party};

#[derive(Template)]
#[template(path = "trainer.jinja")]
pub struct TrainerTemplate {
    name: String,
    pic: PathBuf,
    // pkmn_count: u8,
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
            pic: value.pic.into(),
            // pkmn_count: value.party.iter().count() as u8,
            party: value.party,
        }
    }
}
