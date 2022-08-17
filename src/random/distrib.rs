use rand::prelude::*;

pub trait Distrib<T, R: Rng> {
    fn sample(&self, rng: &mut R) -> Option<T>;
}

impl<T: Clone, R: Rng> Distrib<T, R> for Option<T> {
    fn sample(&self, _rng: &mut R) -> Option<T> {
        self.clone()
    }
}

impl<T, R: Rng, F: Fn(&mut R) -> Option<T>> Distrib<T, R> for F {
    fn sample(&self, rng: &mut R) -> Option<T> {
        self(rng)
    }
}
