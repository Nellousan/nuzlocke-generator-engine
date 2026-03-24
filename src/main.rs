use std::{io::Write, path::Path};

use encounters::emerald_expansion::Encounters;
use tracing::Level;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};

use crate::{database::pokedex, engine::Engine};

mod bundles;
mod database;
mod encounters;
mod engine;
mod options;
mod parties;

fn main() -> eyre::Result<()> {
    let options = options::parse_options()?;
    let log_file = std::fs::File::create(&options.log_file.unwrap_or("log.log".into()))?;
    let log_level = options.log_level.unwrap_or(Level::DEBUG);
    let registry = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(filter::LevelFilter::from_level(log_level)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(log_file)
                .with_filter(filter::LevelFilter::from_level(log_level)),
        );
    tracing::subscriber::set_global_default(registry)?;

    ////
    let pokedex = pokedex::load_pokedex(Path::new("pokedex.json"))?;

    let set_bundle = bundles::load_bundles(vec![
        Path::new("bundles/default/gen6.bundle.json"),
        Path::new("bundles/default/gen7.bundle.json"),
        Path::new("bundles/default/gen8.bundle.json"),
        // Path::new("bundles/default/gen9.bundle.json"),
    ])?;
    let content = std::fs::read_to_string("pokeemerald-expansion/src/data/trainers.party")?;
    let parties = parties::emerald_expansion::from_emerald_expansion_format(&content)?;

    let content = std::fs::read_to_string("pokeemerald-expansion/src/data/wild_encounters.json")?;
    let encounters: Encounters = serde_json::from_str(&content)?;
    let rng = rand::rng();

    let mut engine = Engine {
        parties,
        encounters,
        pokedex,
        set_bundle,
        rng,
    };

    engine.randomize_parties();
    engine.randomize_encounters();

    let result = parties::emerald_expansion::to_emerald_expansion_format(&engine.parties)?;

    let mut file = std::fs::File::create("pokeemerald-expansion/src/data/trainers.party")?;
    file.write_all(result.as_bytes())?;

    let result = serde_json::to_string(&engine.encounters)?;

    let mut file = std::fs::File::create("pokeemerald-expansion/src/data/wild_encounters.json")?;
    file.write_all(result.as_bytes())?;

    Ok(())
}
