use rand::Rng;

use crate::{
    bundles::{PokemonBundleSet, SetBundle},
    database::pokedex::{Pokedex, PokemonDatabaseEntry},
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
        tracing::debug!(?set);
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

        tracing::debug!(?mon_sets, ?database_entry);

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
                    tracing::debug!("{:?}", *mon);
                }
            }
        }

        self.parties = new_parties;
    }

    pub fn randomize_encounters(&mut self) {
        self.encounters.randomize(&self.pokedex, &mut self.rng);
    }
}
