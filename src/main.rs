extern crate clap;
use clap::{App, Arg, SubCommand};
use raya::{AcousticRaytracer, logger::{Phase}};
use serde_json::json;
use std::io;
use text_io::read;

fn main() -> io::Result<()> {
    let matches = App::new("Raya")
        .about("Acoustic raytracer written in rust")
        .version("0.1.1")
        .subcommand(
            SubCommand::with_name("fs")
                .about("uses fs")
                .arg(
                    Arg::with_name("model")
                        .short("m")
                        .long("model")
                        .value_name("FILE")
                        .help("The 3d model file used (.gltf)")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("FILE")
                        .help("The file path for the calculated impulse response (.wav)")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("io").about("uses io").arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .value_name("FILE")
                    .help("The file path for the calculated impulse response (.wav)")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("fs", Some(sub_matches)) => {
            let model = sub_matches.value_of("model").unwrap();
            let output = sub_matches.value_of("output").unwrap();

            match AcousticRaytracer::from_gltf(model) {
                Ok(mut acoustic_raytracer) => {
                    acoustic_raytracer.trace_rays(false);
                    acoustic_raytracer.download_impulse_response(output.to_string());
                }
                Err(_) => {
                    println!("There was a problem setting up the acoustic raytracer");
                }
            }
        }
        ("io", Some(sub_matches)) => {
            println!("{}", json!({ "phase": Phase::Initializing, "progress": 0 }).to_string());
            let output = sub_matches.value_of("output").unwrap();
            let source: String = read!("{}\n");
            match AcousticRaytracer::from_gltf_io(source.as_bytes()) {
                Ok(mut acoustic_raytracer) => {
                    println!("{}", json!({ "phase": Phase::Initializing, "progress": 100 }).to_string());
                    acoustic_raytracer.trace_rays(true);
                    
                    acoustic_raytracer.download_impulse_response(output.to_string());

                    println!("{}", json!({ "phase": Phase::Finishing, "progress": 100 }).to_string());

                    // acoustic_raytracer
                    //     .render(output.to_string())
                    //     .expect("There was a problem rendering the scene");
                }
                Err(_) => {
                    println!("There was a problem setting up the acoustic raytracer");
                }
            }
        }
        _ => {}
    }

    Ok(())
}
