use std::io::Write;

use tracing::Level;
use tracing_subscriber::{Layer, filter, layer::SubscriberExt};

mod parties;

fn main() -> eyre::Result<()> {
    let log_file = std::fs::File::create("log.log")?;
    let registry = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(filter::LevelFilter::from_level(Level::DEBUG)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(log_file)
                .with_filter(filter::LevelFilter::from_level(Level::DEBUG)),
        );
    tracing::subscriber::set_global_default(registry)?;

    let content = std::fs::read_to_string("pokeemerald-expansion/src/data/trainers.party")?;

    let mut parties = parties::emerald_expansion::from_emerald_expansion_format(&content)?;

    for trainer in parties.iter_mut() {
        if let Some(ref mut pokemon) = trainer.party[0] {
            pokemon.species = "Beldum".to_owned();
        }
    }

    let result = parties::emerald_expansion::to_emerald_expansion_format(&parties)?;

    let mut file = std::fs::File::create("pokeemerald-expansion/src/data/trainers.party")?;
    file.write_all(result.as_bytes())?;
    Ok(())
}
