use rand::Rng;

use crate::database::pokedex::Pokedex;

pub mod emerald_expansion;

pub trait Encounters<R: Rng + ?Sized> {
    fn randomize(&mut self, pokedex: &Pokedex, rng: &mut R);
}
