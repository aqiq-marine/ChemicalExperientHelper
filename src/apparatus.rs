use crate::dim_sig_dig::*;
use crate::substance::*;

use std::collections::HashMap;

trait HasVolume {
    // err: 許容誤差 < 1.0
    fn calc_sig_dig(v: usize, err: f64) -> usize {
        let err = err.abs();
        let mut err_digit = 0;
        for i in 0..20 {
            if err * 10_f64.powi(i) >= 0.5  {
                err_digit = i as usize;
                break;
            }
        }
        (v as f64).log10().floor() as usize + 1 + err_digit
    }
    fn get_volume() -> Volume;
}

#[derive(Debug, Clone)]
pub struct Beaker<const V: usize> {
    solution: Solution,
}
impl<const V: usize> HasVolume for Beaker<V> {
    fn get_volume() -> Volume {
        DimSigDig::milli_liter_from(V as u32)
            .set_sig_dig(3)
    }
}

impl<const V: usize> Beaker<V> {
    pub fn new() -> Self {
        Self {
            solution: Solution::new(),
        }
    }
    pub fn add_solution(mut self, l: Solution) -> Self {
        self.solution.add_solution(l);
        assert!(self.solution.get_volume() <= Self::get_volume());
        self
    }
    pub fn add_substance(mut self, s: Solid) -> Self {
        self.solution.add_substance(s);
        assert!(self.solution.get_volume() <= Self::get_volume());
        self
    }
    pub fn fillup_up_to(mut self, v: Volume) -> Self {
        assert!(self.solution.get_volume() < v);
        assert!(v < Self::get_volume());
        self.solution.to_be(v);
        self
    }
    pub fn into_volumetric_flask<const U: usize>(
        &mut self,
        mut flask: VolumetricFlask<U>
    ) -> VolumetricFlask<U> {
        let v = self.solution.get_volume();
        assert!(v < VolumetricFlask::<U>::get_volume());
        flask.add_solution(self.solution.dispense(v))
    }
}

#[derive(Debug, Clone)]
pub struct VolumetricFlask<const V: usize> {
    solution: Solution,
}

impl<const V: usize> HasVolume for VolumetricFlask<V> {
    fn get_volume() -> Volume {
        let sig_dig = Self::calc_sig_dig(V, 0.2);
        DimSigDig::milli_liter_from(V as u32)
            .set_sig_dig(sig_dig)
    }
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
    pub fn get_concentration_by_name(&self, name: &str) -> BasicDimSigDig<1, 0, -3> {
        self.solution.get_concentration()
            .get(&name.to_string())
            .cloned()
            .unwrap_or(0.0.into())
    }
    pub fn get_mol_by_name(&self, name: &str) -> Mol {
        self.solution.get_mol_by_name(name)
    }
    pub fn add_solution(mut self, l: Solution) -> Self {
        self.solution.add_solution(l);
        assert!(self.solution.get_volume() <= Self::get_volume());
        self
    }
    pub fn fillup(mut self) -> Self {
        self.solution.to_be(Self::get_volume());
        self
    }
    pub fn into_pipette_mut<const U: usize>(&mut self, pipette: &mut Pipette<U>) {
        let v = Pipette::<U>::get_volume();
        assert!(v < self.solution.get_volume());
        let s = self.solution.dispense(v);
        pipette.aspirate_solution(s);
    }
    pub fn into_pipette<const U: usize>(&mut self, mut pipette: Pipette<U>) -> Pipette<U> {
        self.into_pipette_mut(&mut pipette);
        pipette
    }
}

pub struct Pipette<const V: usize> {
    solution: Option<Solution>,
}

impl<const V: usize> HasVolume for Pipette<V> {
    fn get_volume() -> Volume {
        // 許容誤差0.03
        let sig_dig = Self::calc_sig_dig(V, 0.03);
        DimSigDig::milli_liter_from(V as u32)
            .set_sig_dig(sig_dig)
    }
}

impl<const V: usize> Pipette<V> {
    pub fn new() -> Self {
        Self {solution: None}
    }
    fn aspirate_solution(&mut self, mut s: Solution) {
        assert!(self.solution.is_none());
        self.solution = Some(s);
    }
    pub fn into_flask_mut<const U: usize>(&mut self, flask: &mut VolumetricFlask<U>) {
        if let Some(s) = self.solution.take() {
            self.solution = None;
            *flask = flask.clone().add_solution(s);
        } else {
            panic!("you are trying to use empty pipette");
        }
    }
    pub fn into_flask<const U: usize>(&mut self, mut flask: VolumetricFlask<U>) -> VolumetricFlask<U> {
        self.into_flask_mut(&mut flask);
        flask
    }
}
