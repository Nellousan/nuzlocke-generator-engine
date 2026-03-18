use rand::Rng;

use crate::{bundles::SetBundle, database::pokedex::Pokedex, parties::Parties};

#[expect(dead_code)]
type FnRndParties<P> = Box<dyn Fn(&P) -> P>;
type FnRndEncounters<E> = Box<dyn Fn(&E) -> E>;

#[expect(dead_code)]
pub struct EngineBuilder<E> {
    parties: Parties,
    encounters: E,
}

#[expect(dead_code)]
pub struct Engine<R>
where
    R: Rng + ?Sized,
{
    pub parties: Parties,
    // encounters: E,
    pub pokedex: Pokedex,
    pub set_bundle: SetBundle,
    pub rng: R,
    // Pokemon set lists
    // Parties randomization funtion
    // Encounter randomization function
}

impl<R: Rng + ?Sized> Engine<R> {
    // pub fn randomize_encounters(&mut self, rd_fn: FnRndEncounters<E>) {
    //     rd_fn(&self.encounters);
    //     unimplemented!()
    // }

    // pub fn randomize_all(&mut self) {
    //     let f: FnRndEncounters<E> = Box::new(expansion_encounter_randomizer);
    //     self.randomize_encounters(f);
    //     unimplemented!()
    // }

    pub fn randomize_parties(&mut self) {
        let squirtle_sets = self.set_bundle.get("Squirtle").unwrap();
        let set = squirtle_sets
            .get(self.rng.next_u32() as usize % squirtle_sets.len())
            .expect("modulo len");

        for party in self.parties.iter_mut() {
            for maybe_mon in party.party.iter_mut() {
                if let Some(mon) = maybe_mon {
                    *mon =
                        set.generate_set("Squirtle".to_owned(), mon.level.unwrap(), &mut self.rng);
                }
            }
        }
    }
}

fn expansion_encounter_randomizer<E>(_: &E) -> E {
    unimplemented!()
}
