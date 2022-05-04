extern crate clap;
use raya::AcousticRaytracer;
use clap::{Arg, App, SubCommand};
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()>{
    let matches = App::new("Raya")
        .about("Acoustic raytracer written in rust")
        .version("0.1.1")
        .subcommand(SubCommand::with_name("fs")
            .about("uses fs")
            .arg(Arg::with_name("model")
                .short("m")
                .long("model")
                .value_name("FILE")
                .help("The 3d model file used (.gltf)")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The file path for the calculated impulse response (.wav)")
                .takes_value(true)
                .required(true)
            )
        )
        .subcommand(SubCommand::with_name("io")
            .about("uses io")
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The file path for the calculated impulse response (.wav)")
                .takes_value(true)
                .required(true)
        )
        )
        
        .get_matches();
    

        match matches.subcommand() {
            ("fs",  Some(sub_matches)) => {
                let model = sub_matches.value_of("model").unwrap();
                let output = sub_matches.value_of("output").unwrap();
                
            
                match AcousticRaytracer::from_gltf(model) {
                    Ok(mut acoustic_raytracer) => {
                        acoustic_raytracer.render(output.to_string()).expect("There was a problem rendering the scene");
                        
                    },
                    Err(_) => {
                        println!("There was a problem setting up the acoustic raytracer");
                        
                    }
                }
            }, 
            ("io",   Some(sub_matches)) => {
                let output = sub_matches.value_of("output").unwrap();
                let mut buffer: Vec<u8> = Vec::new();
                let mut stdin = io::stdin(); // We get `Stdin` here.
                
                stdin.read_to_end(&mut buffer)?;
                match AcousticRaytracer::from_gltf_io(&mut buffer) {
                    Ok(mut acoustic_raytracer) => {
                        acoustic_raytracer.render(output.to_string()).expect("There was a problem rendering the scene");
                        
                    },
                    Err(_) => {
                        println!("There was a problem setting up the acoustic raytracer");
                        
                    }
                }
                
            },
            _                       => {}, 
        }

        Ok(())

}