mod parser {
  pub mod camera_file;
}

use std::env;
use std::process;
use parser::camera_file::parse_camera_file;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
      eprintln!("Usage: {} <camerafile>", args[0]);
      process::exit(1);
  }

  let camera_file = &args[1];
  let camera = match parse_camera_file(camera_file) {
      Ok(camera) => camera,
      Err(error) => {
          panic!("Problem parsing the camera file: {}", error);
      }
  };

  println!("Camera: {:?}", camera);
}