use crate::geometry::{Ray, Primitive, Mesh};
use crate::scene::{Intersect, SceneNode, NonRefIntersection, AcousticMaterial};
use crate::scene::acoustic_material::AbsorptionData;
use crate::utils;
use nalgebra::{Point3, Vector3};
use nalgebra::{point, vector};
use pbr::ProgressBar;
use rand::{Rng, random, thread_rng};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::utils::convert::{p_2_i, lp_2_p, p_2_lp, i_2_p};
use crate::utils::attenuation::air_attenuation;
use crate::signals::reconstruction_filter;
use hound;
use gltf::{json};
use gltf::buffer::Data;
use std::error::Error;
use std::str::FromStr;
use gltf::mesh::util::ReadIndices;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};


const USE_RAYON: bool = true;
const SPEED_OF_SOUND: f32 = 343.0;
pub struct AcousticRaytracer {
    pub root_node: SceneNode,

    pub source: Point3<f32>,
    pub receiver: u32,

    pub ray_paths: Vec<RayPath>,

    // Settings
    pub max_order: u32,
    pub ray_count: u64,
}

impl Default for AcousticRaytracer {
    fn default() -> AcousticRaytracer {
        let mut root = SceneNode::new(0, "root node".to_string());
        let mut receiver = SceneNode::new(1, "receiver".to_string());
        receiver.primitive = Primitive::Sphere;
        root.add_child(receiver);
        point![0.0,0.0,0.0].coords[4];
        AcousticRaytracer {
            root_node: root,
            source: point![0.0, 0.0, 0.0],
            receiver: 1,
            ray_paths: Vec::new(),
            max_order: 100,
            ray_count: 10000
        }
    }
}

fn get_node_from_mesh(mesh: &gltf::Mesh, buffers: &Vec<Data>) -> Result<SceneNode, Box<dyn Error>> {
    let mesh_name = mesh.name().unwrap();
    let mut scene_node = SceneNode::new(rand::random::<u32>(), mesh_name.to_string());
    for primitive in mesh.primitives() {
        let primitive_index = primitive.index();
        
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

        let mut vertices: Vec<Vector3<f32>> = vec![];
        for vert in reader.read_positions().expect("has positions") {
            let [x, y, z] = vert;
            vertices.push(vector![x, y, z]);    
        } 

        let mut faces: Vec<[usize; 3]> = vec![];
        let mut face = [0_usize; 3];
        let mut i = 0_usize;
        match reader.read_indices().expect("has indices") {
            ReadIndices::U8(val) => {
                for index in val {
                    face[i] = index as usize;
                    i = (i+1) % 3;
                    if i == 0 {
                        faces.push(face.clone());
                    }
                }
            },
            ReadIndices::U16(val) => {
                for index in val {
                    face[i] = index as usize;
                    i = (i+1) % 3;
                    if i == 0 {
                        faces.push(face.clone());
                    }
                }
            },
            ReadIndices::U32(val) => {
                for index in val {
                    face[i] = index as usize;
                    i = (i+1) % 3;
                    if i == 0 {
                        faces.push(face.clone());
                    }
                }
            },
        }

        

        let m = Mesh::new(vertices, faces);

        let node_name = format!("{}-{}", mesh_name, primitive_index);

        let mut mesh_node = SceneNode::new(rand::random::<u32>(), node_name);
        // TODO: Better error handling
        mesh_node.primitive = Primitive::Mesh(m);

        if primitive.material().index().is_some() {
            let raw_val = primitive
                .material()
                .extras()
                .as_ref()
                .expect("has extras")
                .get();
            
            let parsed = json::Value::from_str(raw_val).unwrap();
            let extras_object = parsed.as_object().unwrap();
            let abs63 = &extras_object["abs63"];
            let abs125 = &extras_object["abs125"];
            let abs250 = &extras_object["abs250"];
            let abs500 = &extras_object["abs500"];
            let abs1000 = &extras_object["abs1000"];
            let abs2000 = &extras_object["abs2000"];
            let abs4000 = &extras_object["abs4000"];
            let abs8000 = &extras_object["abs8000"];
            let abs16000 = &extras_object["abs16000"];
            let absorption_data: AbsorptionData = AbsorptionData::new(vec![
                [63_f32, abs63.as_f64().unwrap_or(0.1_f64) as f32],
                [125_f32, abs125.as_f64().unwrap_or(0.1_f64) as f32],
                [250_f32, abs250.as_f64().unwrap_or(0.1_f64) as f32],
                [500_f32, abs500.as_f64().unwrap_or(0.1_f64) as f32],
                [1000_f32, abs1000.as_f64().unwrap_or(0.1_f64) as f32],
                [2000_f32, abs2000.as_f64().unwrap_or(0.1_f64) as f32],
                [4000_f32, abs4000.as_f64().unwrap_or(0.1_f64) as f32],
                [8000_f32, abs8000.as_f64().unwrap_or(0.1_f64) as f32],
                [16000_f32, abs16000.as_f64().unwrap_or(0.1_f64) as f32],
            ]);

            // let absorption_json_value = parsed.as_object().unwrap().get("absorption").unwrap();
            let acoustic_material = AcousticMaterial::from_absorption_data(absorption_data);

            mesh_node.acoustic_material = acoustic_material;
        }

        scene_node.add_child(mesh_node);
    }
    Ok(scene_node)
}

impl AcousticRaytracer {
    pub fn new(root_node: SceneNode, source: Point3<f32>, receiver: u32, max_order: u32, ray_count: u64) -> Self {
        Self {
            root_node,
            source,
            receiver,
            max_order,
            ray_count,
            ray_paths: Vec::new(),
        }
    }
    pub fn from_gltf(file_name: &str) -> Result<AcousticRaytracer, Box<dyn Error>> {
        let (gltf, buffers, _) = gltf::import(file_name)?;
        let mut root_node = SceneNode::new(rand::random::<u32>(), file_name.to_string());
        let mut source: Option<Point3<f32>> = None;
        let mut receiver: Option<u32> = None;

        let scene = gltf.default_scene().expect("has default scene");
        let scene_extras_str = scene
            .extras()
            .as_ref()
            .expect("scene has extras")
            .get();
        
        let parsed_scene_extras = json::Value::from_str(scene_extras_str).unwrap();
        let scene_extras_object = parsed_scene_extras.as_object().unwrap();
        let max_order_value = scene_extras_object.get("max_order").unwrap();
        let max_order = max_order_value.as_u64().unwrap_or(50);
        let ray_count_value = scene_extras_object.get("ray_count").unwrap();
        let ray_count = ray_count_value.as_u64().unwrap_or(10000);

        for node in gltf.nodes() {
            let node_extras_str = node
                .extras()
                .as_ref()
                .expect("has extras")
                .get();
            
            let parsed = json::Value::from_str(node_extras_str).unwrap();
            let extras_object = parsed.as_object().unwrap();
            let node_type = &extras_object["node_type"];
            let active = &extras_object["active"];

            if active.as_u64().unwrap_or(0) == 0 {
                continue
            }

            match node_type.as_u64() {
                Some(1) => {
                    println!("type is reflector");
                    let mesh = match node.mesh() {
                        Some(mesh) => mesh,
                        None => continue
                    };
                    match get_node_from_mesh(&mesh, &buffers) {
                        Ok(mesh_node) => {
                            for child in mesh_node.children {
                                root_node.add_child(child);
                            }
                        }
                        Err(_) => {
                            continue
                        }
                    }
                },
                Some(2) => {
                    println!("type is source");
                    let translation = node.transform().decomposed().0;
                    println!("{:?}", translation);
                    source = Some(point![translation[0], translation[1], translation[2]]);
                },
                Some(3) => {
                    println!("type is receiver");
                    let radius = &extras_object["radius"].as_f64().unwrap_or(0.5);
                    let mut receiver_node = SceneNode::new(rand::random::<u32>(), "receiver".to_string());
                    receiver_node.primitive = Primitive::Sphere;
                    let translation = node.transform().decomposed().0;
                    println!("{:?}", translation);
                    receiver_node.scale(*radius as f32, *radius as f32, *radius as f32);
                    receiver_node.translate(translation[0], translation[1], translation[2]);
                    receiver = Some(receiver_node.id);
                    root_node.add_child(receiver_node);
                    // let mesh = match node.mesh() {
                    //     Some(mesh) => mesh,
                    //     None => continue
                    // };
                    // match get_node_from_mesh(&mesh, &buffers) {
                    //     Ok(mesh_node) => {
                    //         for child in mesh_node.children {

                    //         }
                    //     }
                    //     Err(_) => {
                    //         continue
                    //     }
                    // }
                },
                Some(_) | None => {
                    println!("type is unknown");
                    continue
                },
            }
        }

        let acoustic_raytracer = AcousticRaytracer::new(root_node, source.expect("source is some"), receiver.expect("receiver is some"), max_order as u32, ray_count);
        Ok(acoustic_raytracer)
    }

    pub fn render(&mut self, file_name: String) -> Result<(), &str> {
        println!("Rendering");
        let t0 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.trace_rays();

        let t1 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        self.download_impulse_response(file_name);

        let t2 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        println!("trace_rays                = {}", t1-t0);
        println!("download_impulse_response = {}", t2-t1);
        println!("render                    = {}", t2-t0);
        Ok(())
    }


}

pub fn random_vector3() -> Vector3<f32> {
    let mut rng = thread_rng();
    let x: f32 = rng.gen();
    let y: f32 = rng.gen();
    let z: f32 = rng.gen();
    vector![x - 0.5, y - 0.5, z - 0.5].normalize()
}

pub fn probability(prob: f32) -> bool {
    let r: f32 = random();
    return r <= prob;
}

#[derive(Debug, Clone)]
pub struct RayPath {
    path: Vec<NonRefIntersection>,
    source: Point3<f32>,
    distance: f32,
}


impl fmt::Display for RayPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "src = ({:>5.1}, {:>5.1}, {:>5.1})\n", self.source.coords.x, self.source.coords.y, self.source.coords.z)?;
        for i in 0..self.path.len() {
            write!(f, "{}\n", self.path[i])?;
        }
        write!(f, "dist = ({:>5.1})\n", self.distance)?;
        Ok(write!(f, "\n")?)
    }
}



impl RayPath {
    pub fn get_total_distance(&self) -> f32 {
        let mut distance = 0_f32;
        for i in 0..self.path.len() {
            if i == 0 {
                distance += (self.path[i].point.coords - self.source.coords).magnitude();
            } else {
                distance += (self.path[i].point.coords - self.path[i-1].point.coords).magnitude();
            }
        }
        distance
    }
    pub fn get_total_time(&self) -> f32 {
        self.get_total_distance() / SPEED_OF_SOUND
    }
}

fn arrival_pressure(root_node: &SceneNode, initial_spl: &Vec<f32>, freqs: &Vec<f32>, ray_path: &RayPath) -> Vec<f32> {

    let mut intensities = p_2_i(lp_2_p(initial_spl.to_vec()), 400.0);

    // for each surface that the ray intersected
    for i in 0..(ray_path.path.len()-1) {
        
        // get the reflecting surface
        let surface = root_node.find_child_by_id(ray_path.path[i].node).expect("node id exists in scene");
        
        // multiply intensities by the frequency dependant reflection coefficient
        for index in 0..intensities.len() {
            let r = if freqs[index] > 8000.0 {
                1.0 - surface.acoustic_material.absorption_function(8000.0)
            } else {
                1.0 - surface.acoustic_material.absorption_function(freqs[index])
            };
            intensities[index] = intensities[index] * r; // multiply the intensity by the reflection coefficient
        }

    }

    // convert back to SPL 
    let mut arrival_lp = p_2_lp(i_2_p(intensities, 400.0));

    // apply air absorption (dB/m)
    let air_attenuation_db = air_attenuation(&freqs, 20.0, 40.0, 101325.0);
    for freq in 0..freqs.len() {
        arrival_lp[freq] -= air_attenuation_db[freq] * ray_path.distance;
    }

    // convert back to pressure
    lp_2_p(arrival_lp)
}

impl AcousticRaytracer {

    pub fn download_impulse_response(&mut self, path: String) {

        let impulse_response = self.calculate_impulse_response();

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec).unwrap();
        for i in 0..impulse_response.len() {
            let amplitude = i16::MAX as f32;
            writer.write_sample((impulse_response[i] * amplitude) as i16).unwrap();
        }
    }

    pub fn calculate_impulse_response(&mut self) -> Vec<f32> {
        let initial_spl = 100_f32; 
        let frequencies = utils::bands::octave(63.0, 8000.0);
        let sample_rate = 44100_u32;
        

    
        self.ray_paths.sort_by(|a,b| a.distance.partial_cmp(&b.distance).unwrap());
    
         // end time is latest time of arrival plus 0.1 seconds for safety
        let total_time = self.ray_paths[self.ray_paths.len() -1].get_total_time() + 0.05;
        println!("total_time: {}", total_time);
        let spls = vec![initial_spl; frequencies.len()];
    
        // doubled the number of samples to mitigate the signal reversing
        let number_of_samples = (f32::floor(sample_rate as f32 * total_time) * 2.0) as u32;
        println!("number_of_samples: {}", number_of_samples);
        let mut samples: Vec<Vec<f32>> = Vec::new();
        for _ in 0..frequencies.len() {
            samples.push(vec![0_f32; number_of_samples as usize]);
        }
      
        // add in raytracer paths 
        for i in 0..self.ray_paths.len() {
          let random_phase = if random() { 1.0 } else { -1.0 };
          let t = self.ray_paths[i].get_total_time();
          let p: Vec<f32> = arrival_pressure(&self.root_node, &spls, &frequencies, &self.ray_paths[i]).iter().map(|x| x * random_phase).collect(); 
          let rounded_sample = f32::floor(t * (sample_rate as f32)) as usize;
    
          for f in 0..frequencies.len() {
              samples[f][rounded_sample] += p[f];
          }
        }
        
        let filtered_samples = reconstruction_filter::filter_signals(samples);
        // let filtered_samples = samples;
    
            // make the new signal's length half as long, we dont need the reversed part
        let mut signal: Vec<f32> = vec![0.0; filtered_samples[0].len() / 2];
        
        let mut max = 0.0;
        for i in 0..filtered_samples.len() {
            for j in 0..signal.len() {
                signal[j] += filtered_samples[i][j];
                if f32::abs(signal[j]) > max {
                  max = f32::abs(signal[j]);
                }
            }
        }
        println!("max: {}", max);
        for i in 0..signal.len() {
            signal[i] /= max;
        }
        signal
    }
    pub fn trace_rays(&mut self){
        let count = self.ray_count;
        let valid_ray_count = Arc::new(AtomicUsize::new(0));
        let t_pr = valid_ray_count.clone();
        let completion_string = format!("traced {} valid rays!", count);
        let progress_thread = thread::spawn(move || {
            let mut progress_bar = ProgressBar::new(count);
            progress_bar.show_counter = false;
            progress_bar.show_speed = false;
            progress_bar.tick_format("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
            progress_bar.format("[▱▱ ]");
            let mut value = t_pr.load(Ordering::Relaxed);
            while (value as u64) < count {
                thread::sleep(Duration::from_millis(100));
                progress_bar.set(value as u64);
                value = t_pr.load(Ordering::Relaxed);
            }
            progress_bar.finish_print(&completion_string);
        });
        if USE_RAYON {
            println!("USING RAYON");
            while (valid_ray_count.load(Ordering::Relaxed) as u64) < count {
                let valid_ray_paths: Vec<RayPath> = (0..count)
                    .into_par_iter()
                    .filter_map(|_| {
                        let rp = self.trace_ray();
                        if rp.is_some() {
                            valid_ray_count.fetch_add(1, Ordering::Relaxed);
                            Some(rp.unwrap())
                        } else {
                            None
                        }
                    }).collect();
                for rp in valid_ray_paths.iter() {
                    self.ray_paths.push(rp.clone());
                }
            }
        } else {
            while (valid_ray_count.load(Ordering::Relaxed) as u64) < count {
                let valid_ray_paths: Vec<RayPath> = (0..count)
                    .into_iter()
                    .filter_map(|_| {
                        let rp = self.trace_ray();
                        if rp.is_some() {
                            valid_ray_count.fetch_add(1, Ordering::Relaxed);
                            Some(rp.unwrap())
                        } else {
                            None
                        }
                    }).collect();
                for rp in valid_ray_paths.iter() {
                    self.ray_paths.push(rp.clone());
                }
            }
        }
        progress_thread.join().unwrap();
    }

    pub fn trace_ray(&self) -> Option<RayPath> {
        let mut ray_path = RayPath {
            source: self.source,
            path: Vec::new(),
            distance: 0_f32
        };

        let mut ray = Ray::new(self.source, random_vector3());
        let mut collision = self.root_node.intersects(&ray);
        let mut order = 0_u32;
        let mut intersected_receiver = false;

        while order < self.max_order && collision.is_some() && !intersected_receiver {
            let intersection = collision.unwrap();
            intersected_receiver = intersection.node.id == self.receiver;
            // move the ray to the intersection point
            ray.src = intersection.point;
            // reflect the ray
            ray.dir = ray.dir-(intersection.normal.scale(ray.dir.dot(&intersection.normal)).scale(2.0_f32));
            ray.dir.normalize_mut();
            let scattering = 0.1;
            
            if probability(scattering) {
                ray.dir = random_vector3();
                if intersection.normal.dot(&ray.dir) < 0.0 {
                    ray.dir.scale_mut(-1.0);
                }
            }
            // add the intersection to the ray_path
            ray_path.path.push(intersection.get_non_ref());
            // increment the order
            order += 1;
            collision = self.root_node.intersects(&ray);
        }

        if intersected_receiver {
            ray_path.distance = ray_path.get_total_distance();
            Some(ray_path)
        } else {
            None
        }
    }
}