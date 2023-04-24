use crate::unit_system::*;
use crate::substance::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct VolumetricFlask<const V: usize> {
    solution: MixtureLiquid,
}

impl<const V: usize> VolumetricFlask<V> {
    fn add_solid(self, s: PureSolid) -> Self {
        self.add_pure_liquid(s.into_liquid(0.0.into()))
    }
    fn add_pure_liquid(self, l: PureLiquid) -> Self {
        self.solution.add_pure_liquid(l);
        assert!(self.solution.get_volume() <= (V as f64).into());
        self
    }
    fn add_mixture(self, l: MixtureLiquid) -> Self {
        self.solution.add_mixture_liquid(l);
        assert!(self.solution.get_volume() <= (V as f64).into());
        self
    }
    fn fillup(self) -> Self {
        self.solution.to_be_certain_volume((V as f64).into());
        self
    }
    fn into_pipette<const U: usize>(&mut self, pipette: Pipette<U>) -> Pipette<U> {
        pipette
    }
}

struct Pipette<const V: usize> {
    solution: MixtureLiquid,
}

impl<const V: usize> Pipette<V> {
    fn into_flask<const U: usize>(self, flask: VolumetricFlask<U>) -> VolumetricFlask<U> {
        flask.add_mixture(self.solution)
    }
}
