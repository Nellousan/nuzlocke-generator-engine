use crate::parties::Parties;

#[expect(dead_code)]
type FnRndEncounters<E> = fn(E) -> E;
#[expect(dead_code)]
type FnRndParties<E> = fn(E) -> E;

#[expect(dead_code)]
pub struct EngineBuilder<E> {
    parties: Parties,
    encounters: E,
}

#[expect(dead_code)]
pub struct Engine<E> {
    parties: Parties,
    encounters: E,
    // Random number generator
    // Pokemon set lists
    // Pokemon encounter lists
    // Parties randomization funtion
    // Encounter randomization function
}

impl<E> Engine<E> {
    fn randomize_encounters(&mut self, rd_fn: FnRndEncounters<E>) {
        unimplemented!()
    }
}
