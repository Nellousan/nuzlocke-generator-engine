mod parties;

fn main() -> eyre::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let content = std::fs::read_to_string("pokeemerald-expansion/src/data/trainers.party")?;

    let parties = parties::emerald_expansion::from_emerald_expansion_format(&content);

    Ok(())
}
