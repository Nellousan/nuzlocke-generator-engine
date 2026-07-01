use std::path::Path;

use clap::Parser;
use rand::{SeedableRng, rngs::SmallRng};
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};

use crate::{database::pokedex, engine::Engine};

mod bundles;
mod cli;
mod database;
mod doc;
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

    let parties = parties::load_parties(&cli.project)?;

    let encounters = encounters::load_encounter(&cli.project)?;
    let rng: SmallRng = if let Some(seed) = cli.seed {
        SmallRng::seed_from_u64(seed)
    } else {
        rand::make_rng()
    };

    let mut engine = Engine {
        parties,
        encounters,
        pokedex,
        set_bundle,
        cli_options: cli,
        rng: Box::new(rng),
    };

    engine.randomize_parties();
    engine.randomize_encounters();

    parties::save_parties(&engine.cli_options.project, &engine.parties)?;

    encounters::save_encounter(&engine.cli_options.project, &engine.encounters)?;

    // let test_party = (*engine.parties)[3].clone();
    // let test_party_template: TrainerTemplate = test_party.into();
    // let res = test_party_template.render()?;
    // let mut file = File::create("osef.html")?;
    // file.write_all(res.as_bytes())?;
    //

    engine.generate_documentation()?;

    Ok(())
}
