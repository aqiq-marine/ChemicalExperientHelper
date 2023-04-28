#![feature(generic_const_exprs)]

mod substance;
use substance::*;

mod apparatus;
use apparatus::*;

mod unit_system;
use unit_system::*;

fn main() {
    let mut beaker = Beaker::<100>::new()
        .add_substance(
            Substance::create(
                "Mohr".to_string(),
                392.1.into(),
                0.4019.into(),
                0.0.into(),
            )
        )
        .fillup_up_to(Volume::milli_liter_from(20));
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
