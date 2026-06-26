use std::{fs::File, io::Write, path::PathBuf};

use askama::Template;
use rand::Rng;

use crate::{
    bundles::{PokemonBundleSet, SetBundle},
    cli::EmeraldExpansionOption,
    database::pokedex::{Pokedex, PokemonDatabaseEntry},
    doc::{TrainerListTemplate, TrainerTemplate},
    encounters::Encounters,
    parties::{Parties, party::PokemonSet},
};

#[expect(dead_code)]
pub struct EngineBuilder<E> {
    parties: Parties,
    encounters: E,
}

pub struct Engine<R>
where
    R: Rng + ?Sized,
{
    pub parties: Parties,
    pub encounters: Box<dyn Encounters<R>>,
    pub pokedex: Pokedex,
    pub set_bundle: SetBundle,
    pub cli_options: crate::cli::Cli,
    pub rng: Box<R>,
}

impl<R: Rng + ?Sized> Engine<R> {
    fn get_random_mon_within_bst_range(&mut self, set: &PokemonSet) -> PokemonDatabaseEntry {
        let set_database_entry = self
            .pokedex
            .get(
                &set.species
                    .to_lowercase()
                    .replace('-', "")
                    .replace(' ', "")
                    .replace(':', ""),
            )
            .expect("pokemon should exist");
        let all_within_range =
            self.pokedex
                .get_all_within_bst_range(set_database_entry.base_stats.total(), 30, 30);
        all_within_range
            .get(self.rng.next_u32() as usize % all_within_range.len())
            .expect("modulo len")
            .clone()
    }

    fn get_random_bundle_set(
        &mut self,
        database_entry: &PokemonDatabaseEntry,
    ) -> Option<PokemonBundleSet> {
        let Some(mon_sets) = self.set_bundle.get(&database_entry.name) else {
            return None;
        };

        Some(
            mon_sets
                .get(self.rng.next_u32() as usize % mon_sets.len())
                .expect("modulo len")
                .clone(),
        )
    }

    fn generate_new_pokemon_set(&mut self, pkmn_set: &PokemonSet) -> PokemonSet {
        let mut database_entry = self.get_random_mon_within_bst_range(&pkmn_set);
        let mut random_bundle_set = self.get_random_bundle_set(&database_entry);

        while let None = random_bundle_set {
            tracing::debug!("Rerolling pokemon species");
            database_entry = self.get_random_mon_within_bst_range(&pkmn_set);
            random_bundle_set = self.get_random_bundle_set(&database_entry);
        }

        let random_bundle_set = random_bundle_set.expect("Cannot be None");

        random_bundle_set.generate_set(&database_entry.name, pkmn_set.level.unwrap(), &mut self.rng)
    }

    pub fn randomize_parties(&mut self) {
        let mut new_parties = std::mem::take(&mut self.parties);
        for party in new_parties.iter_mut() {
            for maybe_mon in party.party.iter_mut() {
                if let Some(mon) = maybe_mon {
                    *mon = self.generate_new_pokemon_set(mon);
                    if self.cli_options.disable_evs {
                        mon.evs = None;
                    }
                    tracing::debug!(?mon);
                }
            }
        }

        self.parties = new_parties;
    }

    pub fn randomize_encounters(&mut self) {
        let options = match self.cli_options.project {
            crate::cli::ProjectOption::EmeraldExpansion(ref options) => options,
        };

        self.encounters.randomize(
            &self.pokedex,
            &mut self.rng,
            options.global_encounter_randomization,
        );
    }

    fn generate_pokeemerald_documentation(
        &mut self,
        option: EmeraldExpansionOption,
    ) -> eyre::Result<()> {
        let html_assets_dir = self.cli_options.output_directory.join("assets");
        let html_assets_pkmn_dir = html_assets_dir.join("pkmn");
        let html_assets_trainer_dir = html_assets_dir.join("trainer");

        std::fs::create_dir_all(&html_assets_pkmn_dir)?;
        std::fs::create_dir_all(&html_assets_trainer_dir)?;

        let trainer_pics_dir = option.project_path.join("graphics/trainers/front_pics");

        for trainer in self.parties.iter() {
            let mut pic_filename = PathBuf::from(trainer.pic.to_lowercase().replace(' ', "_"));
            pic_filename.add_extension("png");

            // Ideally handle properly all the possible sprites.
            // Right now this is a wrokaround for may and brendan sprites needing an edge case
            if !std::fs::exists(trainer_pics_dir.join(&pic_filename))? {
                tracing::warn!("{} not found", pic_filename.display());
                continue;
            }

            std::fs::copy(
                trainer_pics_dir.join(&pic_filename),
                html_assets_trainer_dir.join(&pic_filename),
            )?;
        }

        let trainer_templates: Vec<TrainerTemplate> = self
            .parties
            .iter()
            .map(Clone::clone)
            .map(Into::into)
            .collect();
        let trainer_list_template: TrainerListTemplate = trainer_templates.into();

        let res = trainer_list_template.render()?;
        let mut file = File::create(self.cli_options.output_directory.join("trainers.html"))?;
        file.write_all(res.as_bytes())?;

        Ok(())
    }

    pub fn generate_documentation(&mut self) -> eyre::Result<()> {
        match &self.cli_options.project {
            crate::cli::ProjectOption::EmeraldExpansion(option) => {
                self.generate_pokeemerald_documentation(option.clone())?;
            }
        }

        Ok(())
    }
}
