use crate::unit_system::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Substance {
    pub name: String,
    molar_mass: BasicDimSigDig<-1, 1, 0>,
    mass: Mass,
    volume: Volume,
}

impl Substance {
    pub fn create(name: String, molar_mass: BasicDimSigDig<-1, 1, 0>, mass: Mass, volume: Volume) -> Self {
        Self {name, molar_mass, mass, volume}
    }
    fn get_mol(&self) -> Mol {
        self.mass / self.molar_mass
    }
    fn add_same_substance(&mut self, s: Substance) {
        assert_eq!(self.name, s.name);
        assert_eq!(self.molar_mass, self.molar_mass);
        self.mass += s.mass;
        self.volume += s.volume;
    }
    fn zero(&self) -> Self {
        Self {
            mass: 0.0.into(),
            volume: 0.0.into(),
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Solution {
    solute: HashMap<String, Substance>,
    volume: Volume,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            solute: HashMap::new(),
            volume: 0.0.into(),
        }
    }
    pub fn get_volume(&self) -> BasicDimSigDig<0, 0, 3> {
        self.volume
    }
    pub fn get_mol(&self, name: &str) -> Mol {
        self.solute.get(name).map(|s| s.get_mol()).unwrap_or(0.0.into())
    }
    pub fn get_concentration(&self) -> HashMap<String, BasicDimSigDig<1, 0, -3>> {
        let mut result = HashMap::new();
        for name in self.solute.keys() {
            let mol = self.get_mol(name);
            result.insert(name.clone(), mol / self.volume);
        }
        result
    }
    pub fn add_substance(&mut self, s: Substance) {
        // to be uncertain
        self.volume += s.volume;
        self.solute.entry(s.name.clone())
            .or_insert(s.zero())
            .add_same_substance(s);
    }
    pub fn add_solution(&mut self, s: Solution) {
        self.volume += s.volume;

        for (_, solute) in s.solute.into_iter() {
            self.solute.entry(solute.name.clone())
                .or_insert(solute.zero())
                .add_same_substance(solute);
        }
    }
    pub fn to_be_certain_volume(&mut self, v: BasicDimSigDig<0, 0, 3>) {
        assert!(self.volume <= v);
        self.volume = v;
    }
    pub fn dispense(&mut self, v: BasicDimSigDig<0, 0, 3>) -> Solution {
        let ratio = v / self.volume;
        let mut solution = self.clone();

        solution.solute.iter_mut().map(|(_, s)| {
            s.mass = s.mass * ratio;
            s.volume = s.volume * ratio;
        });
        solution.volume = v;

        self.solute.iter_mut().map(|(_, s)| {
            let one: NoDim = 1.0.into();
            let r = one - ratio;
            s.mass = s.mass * r;
            s.volume = s.volume * r;
        });
        self.volume -= v;

        solution
    }
}

// pub trait Substance {
//     fn get_mol(&self) -> Mol;
// }
// 
// #[derive(Debug, Clone)]
// pub struct PureSolid {
//     pub name: String,
//     molar_mass: BasicDimSigDig<-1, 1, 0>,
//     mass: BasicDimSigDig<0, 1, 0>,
// }
// 
// impl PureSolid {
//     pub fn into_liquid(self, v: Volume) -> PureLiquid {
//         PureLiquid { name: self.name, molar_mass: self.molar_mass, mass: self.mass, volume: v}
//     }
// }
// 
// impl Substance for PureSolid {
//     fn get_mol(&self) -> Mol {
//         self.mass / self.molar_mass
//     }
// }
// 
// #[derive(Debug, Clone)]
// pub struct PureLiquid {
//     pub name: String,
//     molar_mass: BasicDimSigDig<-1, 1, 0>,
//     mass: Mass,
//     volume: Volume,
// }
// 
// impl PureLiquid {
//     pub fn get_volume(&self) -> Volume {
//         self.volume
//     }
// }
// 
// impl Substance for PureLiquid {
//     fn get_mol(&self) -> Mol {
//         self.mass / self.molar_mass
//     }
// }
// 
// #[derive(Debug, Clone)]
// pub struct MixtureLiquid {
//     c: Vec<PureLiquid>,
//     v: BasicDimSigDig<0, 0, 3>
// }
// 
// impl MixtureLiquid {
//     pub fn add_pure_liquid(&mut self, l: PureLiquid) {
//         self.c.push(l);
//         // 本当は不正確
//         // to be uncertainなど実装すること
//         self.v += l.get_volume();
//     }
//     pub fn add_mixture_liquid(&mut self, l: MixtureLiquid) {
//         // 単純なそれぞれの洋室に関する体積の和ではない
//         // メスアップなどの可能性を考慮しpure_liquidのfoldとしては定義しない
//         self.c.append(&mut l.c);
//         self.v += l.get_volume();
//     }
//     pub fn get_volume(&self) -> BasicDimSigDig<0, 0, 3> {
//         self.v
//     }
//     pub fn to_be_certain_volume(&mut self, v: BasicDimSigDig<0, 0, 3>) {
//         assert!(self.v <= v);
//         self.v = v;
//     }
//     pub fn dispense(&mut self, v: BasicDimSigDig<0, 0, 3>) {
//     }
// }
