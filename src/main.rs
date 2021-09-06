extern crate clap;
use raya::AcousticRaytracer;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Raya")
        .about("Acoustic raytracer written in rust")
        .arg(Arg::with_name("model")
            .short("m")
            .long("model")
            .value_name("FILE")
            .help("The 3d model file used (.gltf)")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("The file path for the calculated impulse response (.wav)")
            .takes_value(true)
            .required(true))
        .get_matches();
    
    let model = matches.value_of("model").unwrap();
    let output = matches.value_of("output").unwrap();
    

    match AcousticRaytracer::from_gltf(model) {
        Ok(mut acoustic_raytracer) => {
            acoustic_raytracer.render(output.to_string()).expect("There was a problem rendering the scene");
        },
        Err(_) => {
            println!("There was a problem setting up the acoustic raytracer");
        }
    };
}