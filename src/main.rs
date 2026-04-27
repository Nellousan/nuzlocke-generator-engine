use std::{io::Write, path::Path};

use clap::Parser;
use encounters::emerald_expansion::Encounters;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};

use crate::{database::pokedex, engine::Engine};

mod bundles;
mod cli;
mod database;
mod encounters;
mod engine;
mod parties;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();
    let log_file = std::fs::File::create(&cli.log_file)?;
    let log_level = cli.log_level.into();
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
    let pokedex = pokedex::load_pokedex(Path::new(&cli.pokedex))?;

    let set_bundle = bundles::load_bundles(&cli.bundles)?;
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
