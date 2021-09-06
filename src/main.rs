extern crate clap;
use raya::AcousticRaytracer;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Raya")
        .about("Acoustic raytracer written in rust")
        .version("0.1.1")
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
        .arg(Arg::with_name("max-order")
            .short("r")
            .long("max-order")
            .help("Overrides the max order defined in model")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("ray-count")
            .short("c")
            .long("ray-count")
            .help("Overrides the ray count defined in model")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("single-thread")
            .short("s")
            .long("single-thread")
            .help("Use a single thread"))
        .get_matches();
    
    let model = matches.value_of("model").unwrap();
    let output = matches.value_of("output").unwrap();
    let single_thread = matches.is_present("single-thread");
    let max_order = match matches.value_of("max-order") {
        Some(val) => {
            let parsed = val.parse::<u32>();
            if parsed.is_ok() {
                Some(parsed.unwrap())
            } else {
                None
            }
        }
        None => {
            None
        }
    };
    let ray_count = match matches.value_of("ray-count") {
        Some(val) => {
            let parsed = val.parse::<u64>();
            if parsed.is_ok() {
                Some(parsed.unwrap())
            } else {
                None
            }
        }
        None => {
            None
        }
    };
    
    match AcousticRaytracer::from_gltf(model) {
        Ok(mut acoustic_raytracer) => {
            if max_order.is_some() {
                acoustic_raytracer.max_order = max_order.unwrap();
            }
            if ray_count.is_some() {
                acoustic_raytracer.ray_count = ray_count.unwrap();
            }
            acoustic_raytracer.render(output.to_string(), !single_thread).expect("There was a problem rendering the scene");
        },
        Err(_) => {
            println!("There was a problem setting up the acoustic raytracer");
        }
    };
}