use raya::AcousticRaytracer;
use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let gltf_file_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "bench/auditorium/raya/auditorium.gltf".to_string()
    };
    let out_file_name = if args.len() > 2 {
        args[2].clone()
    } else {
        "bench/auditorium/raya/auditorium.wav".to_string()
    };

    match AcousticRaytracer::from_gltf(gltf_file_name.as_str()) {
        Ok(mut acoustic_raytracer) => {
            acoustic_raytracer.render(out_file_name).expect("should have rendered");
        },
        Err(_) => {
            println!("there was a problem");
        }
    };
}
