mod parser {
  pub mod camera_file;
  pub mod light_file;
  pub mod objects_file;
}

mod tracer {
  pub mod scene;
}

mod printer {
  pub mod ppm;
}

use std::env;
use std::process;
use parser::camera_file::Camera;
use parser::light_file::Light;
use parser::objects_file::Objects;
use printer::ppm::generate_ppm_file;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 4 {
    println!("Usage: ./we <camera_file> <light_file> <objects_file>");
    process::exit(1);
  }
  
  let camera_file = &args[1];
  let camera = match Camera::parse_from_file(camera_file) {
    Ok(camera) => camera,
    Err(error) => {
      panic!("Problem parsing the camera file: {}", error);
    }
  };
  
  let light_file = &args[2];
  let light = match Light::parse_from_file(light_file) {
    Ok(light) => light,
    Err(error) => {
      panic!("Problem parsing the light file: {}", error);
    }
  };
  
  let objects_file = &args[3];
  let objects = match Objects::parse_from_file(objects_file) {
    Ok(objects) => objects,
    Err(error) => {
      panic!("Problem parsing the objects file: {}", error);
    }
  };
  
  println!("Camera: {:?}", camera);
  println!("Light: {:?}", light);
  println!("Objects: {:?}", objects);

  let scene = tracer::scene::Scene::new(camera, light, objects);
  
  generate_ppm_file(
    scene.camera.width as u32,
    scene.camera.height as u32, 
    &scene.camera.output_file_name, 
    &scene.trace()
  ).unwrap();
  
}