#![feature(generic_const_exprs)]

mod substance;
use substance::*;

mod apparatus;
use apparatus::*;

mod dim_sig_dig;
use dim_sig_dig::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mohr1() {
        let mut beaker = Beaker::<100>::new()
            .add_substance(Solid::create(
                Substance::create(
                    "Mohr".to_string(),
                    DimSigDig::molar_mass_from(392.1).set_sig_dig(4),
                ),
                DimSigDig::gram_from(0.4019).set_sig_dig(4),
                0.0.into(),
            ))
            .fillup_up_to(Volume::milli_liter_from(20).set_sig_dig(2));
        let mut flask1 = beaker
            .into_volumetric_flask(VolumetricFlask::<100>::new())
            .fillup();
        let mut flask2 = flask1
            .into_pipette(Pipette::<5>::new())
            .into_flask(VolumetricFlask::<200>::new())
            .fillup();
        let mohr_concentration = flask2
            .get_concentration_by_name("Mohr")
            .convert_to_molar()
            .normalized();
        let expected = DimSigDig::molar_from(2.562).pow10(-4).set_sig_dig(4);
        println!("{} is close to {}", mohr_concentration, expected);
        assert!(mohr_concentration.is_close_to(&expected));


        let mut pipette = Pipette::<5>::new();
        let into_flask = |flask: &mut VolumetricFlask<200>, pipette: &mut Pipette<5>, n_times: usize| -> VolumetricFlask<50> {
            let mut v_flask = VolumetricFlask::<50>::new();
            for _ in 0..n_times {
                flask.into_pipette_mut(pipette);
                pipette.into_flask_mut(&mut v_flask);
                println!("added to flask: {}", v_flask.get_mol_by_name("Mohr"));
            }
            v_flask.fillup()
        };
        let mut v_flasks = [0; 5].map(|_| VolumetricFlask::<50>::new());
        for i in 0..5 {
            v_flasks[i] = into_flask(&mut flask2, &mut pipette, i);
        }
        let v_flasks_c = v_flasks.map(|flask| flask.get_concentration_by_name("Mohr"));


        let v_expected = [0, 1, 2, 3, 4]
            .map(|i| expected * DimSigDig::nodim_from(i as f64 / 10.0));

        v_flasks_c.iter().zip(v_expected.iter())
            .for_each(|(c, e)| {
                println!("{} is close to {}", c.convert_to_molar().get_raw_num(), e.get_raw_num());
                assert!(c.is_close_to(e));
            });
    }

    #[test]
    fn mohr2() {
        let c = Beaker::<100>::new()
            .add_substance(Solid::create(
                Substance::create(
                    "Fe".to_string(),
                    DimSigDig::molar_mass_from(55.85).set_sig_dig(4),
                ),
                1.0.into(),
                0.0.into(),
            )).fillup_up_to(Volume::milli_liter_from(46))
            .into_volumetric_flask(VolumetricFlask::<100>::new())
            .get_mol_by_name("Fe");
        let expected = 0.017905.into();
        println!("{} is close to {}", c, expected);
        assert!(c.is_close_to(&expected));
    }
}
