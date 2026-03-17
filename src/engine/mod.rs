use rand::Rng;

use crate::parties::Parties;

#[expect(dead_code)]
type FnRndParties<P> = Box<dyn Fn(&P) -> P>;
type FnRndEncounters<E> = Box<dyn Fn(&E) -> E>;

#[expect(dead_code)]
pub struct EngineBuilder<E> {
    parties: Parties,
    encounters: E,
}

#[expect(dead_code)]
pub struct Engine<R, E>
where
    R: Rng + ?Sized,
{
    parties: Parties,
    encounters: E,
    rng: R,
    // Pokemon set lists
    // Pokemon encounter lists
    // Parties randomization funtion
    // Encounter randomization function
}

#[expect(dead_code)]
impl<R: Rng + ?Sized, E: 'static> Engine<R, E> {
    fn randomize_encounters(&mut self, rd_fn: FnRndEncounters<E>) {
        rd_fn(&self.encounters);
        unimplemented!()
    }

    fn randomize_all(&mut self) {
        let f: FnRndEncounters<E> = Box::new(expansion_encounter_randomizer);
        self.randomize_encounters(f);
        unimplemented!()
    }
}

fn expansion_encounter_randomizer<E>(_: &E) -> E {
    unimplemented!()
}
