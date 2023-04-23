use crate::unit_system::*;

pub struct Solid {
    pub name: String,
    molar_mass: BasicDimSigDig<-1, 1, 0>,
    mass: BasicDimSigDig<0, 1, 0>,
}

impl Solid {
    pub fn get_mol(&self) -> BasicDimSigDig<1, 0, 0> {
        self.mass / self.molar_mass
    }
}

pub struct Liquid {
    pub name: String,
    concentration: Vec<BasicDimSigDig<1, 0, -3>>,
}
