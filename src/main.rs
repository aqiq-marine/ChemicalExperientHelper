#![feature(generic_const_exprs)]

mod substance;
use substance::*;

mod apparatus;
use apparatus::*;

mod unit_system;
use unit_system::*;

fn main() {
    let pipette = Pipette::<5>::new();
    let mut flask = VolumetricFlask::<250>::new()
        .add_substance(
            Substance::create(
                "EDTA".to_string(),
                391.0.into(),
                1.0.into(),
                Volume::milli_liter_from::<u32>(1)
            )
        ).fillup();
    let mut sub_flask = VolumetricFlask::<50>::new();
    let c = flask
        .into_pipette(pipette)
        .into_flask(sub_flask)
        .fillup()
        .get_concentration()
        .get("EDTA")
        .cloned()
        .unwrap_or(0.0.into());
    println!("{:?}", c);
}
