use crate::unit_system::*;

#[derive(Debug, Clone)]
pub struct Substance {
    pub name: String,
    molar_mass: BasicDimSigDig<-1, 1, 0>,
    mass: Mass,
    volume: Volume,
}

#[derive(Debug, Clone)]
pub struct Solution {
    solute: Vec<Substance>,
    volume: Volume,
}

impl Solution {
    pub fn get_volume(&self) -> BasicDimSigDig<0, 0, 3> {
        self.volume
    }
    pub fn to_be_certain_volume(&mut self, v: BasicDimSigDig<0, 0, 3>) {
        assert!(self.volume <= v);
        self.volume = v;
    }
    pub fn dispense(&mut self, v: BasicDimSigDig<0, 0, 3>) -> Solution {
        let ratio = v / self.volume;
        let solution = self.clone();

        solution.solute.iter_mut().map(|s| {
            s.mass = s.mass * ratio;
            s.volume = s.volume * ratio;
        });
        solution.volume = v;

        self.solute.iter_mut().map(|s| {
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
