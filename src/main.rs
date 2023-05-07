#![feature(generic_const_exprs)]

mod substance;
use substance::*;

mod apparatus;
use apparatus::*;

mod dim_sig_dig;
use dim_sig_dig::*;

fn main() {
    let mut beaker = Beaker::<100>::new()
        .add_substance(
            Substance::create(
                "Mohr".to_string(),
                DimSigDig::molar_mass_from(392.1).set_sig_dig(4),
                DimSigDig::gram_from(0.4019).set_sig_dig(4),
                0.0.into(),
            )
        )
        .fillup_up_to(Volume::milli_liter_from(20).set_sig_dig(2));
    let mut flask1 = beaker.into_volumetric_flask(VolumetricFlask::<100>::new())
        .fillup();
    let flask2 = flask1.into_pipette(Pipette::<5>::new())
        .into_flask(VolumetricFlask::<200>::new())
        .fillup();
    println!(
        "{:?}",
        flask2
            .get_concentration()
            .get(&"Mohr".to_string())
            .cloned()
            .unwrap_or(0.0.into())
            .convert_to_molar()
    );

}
