use crate::unit_system::*;
use crate::substance::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VolumetricFlask<const V: usize> {
    solution: Solution,
}

impl<const V: usize> VolumetricFlask<V> {
    pub fn new() -> Self {
        Self {
            solution: Solution::new()
        }
    }
    pub fn get_concentration(&self) -> HashMap<String, BasicDimSigDig<1, 0, -3>> {
        self.solution.get_concentration()
    }
    pub fn add_solution(mut self, l: Solution) -> Self {
        self.solution.add_solution(l);
        assert!(self.solution.get_volume() <= DimSigDig::milli_liter_from::<u32>(V as u32));
        self
    }
    pub fn add_substance(mut self, s: Substance) -> Self {
        self.solution.add_substance(s);
        assert!(self.solution.get_volume() <= DimSigDig::milli_liter_from::<u32>(V as u32));
        self
    }
    pub fn fillup(mut self) -> Self {
        self.solution.to_be_certain_volume(DimSigDig::milli_liter_from::<u32>(V as u32));
        self
    }
    pub fn into_pipette<const U: usize>(&mut self, pipette: Pipette<U>) -> Pipette<U> {
        let v = DimSigDig::milli_liter_from::<u32>(U as u32);
        assert!(v < self.solution.get_volume());
        // ここで正確にはかりとりたい
        let s = self.solution.dispense(v);
        pipette.aspirate_solution(s)
    }
}

pub struct Pipette<const V: usize> {
    solution: Option<Solution>,
}

impl<const V: usize> Pipette<V> {
    pub fn new() -> Self {
        Self {solution: None}
    }
    fn aspirate_solution(self, mut s: Solution) -> Self {
        Self {
            solution: Some(s),
        }
    }
    pub fn into_flask<const U: usize>(&mut self, flask: VolumetricFlask<U>) -> VolumetricFlask<U> {
        if let Some(s) = self.solution.take() {
            self.solution = None;
            flask.add_solution(s)
        } else {
            println!("you are trying to use empty pipette");
            flask
        }
    }
}
