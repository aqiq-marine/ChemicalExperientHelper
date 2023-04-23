use crate::unit_system::*;
use crate::substance::*;
use std::collections::HashMap;

struct VolumetricFlask<const V: usize> {
    v: BasicDimSigDig<0, 0, 3>,
    mol: HashMap<String, Mol>,
}

impl<const V: usize> VolumetricFlask<V> {
    fn add_solid(&mut self, s: Solid) {
        *self.mol.entry(s.name.clone()).or_insert(0.0.into()) += s.get_mol();
    }
}
