use std::error::Error;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::MAIN_SEPARATOR;

// Define a function to generate a PPM file from image data
pub fn generate_ppm_file(width: u32, height: u32, file_name: &String, data: &[u8]) -> Result<(), Box<dyn Error>> {
  // Create "ppm" directory if it doesn't exist
  let dir_path = format!("ppm");
  create_dir_all(&dir_path)?;
  
  // Create PPM file
  let file_path = format!("{}{}{}", dir_path, MAIN_SEPARATOR, file_name);
  let mut file = File::create(&file_path)?;
  
  // Write PPM header
  writeln!(file, "P3")?;
  writeln!(file, "{} {}", width, height)?;
  writeln!(file, "255")?;
  
  // Write image data
  for row in 0..height {
    for col in 0..width {
      let index = ((row * width + col) * 3) as usize;
      let r = data[index];
      let g = data[index + 1];
      let b = data[index + 2];
      writeln!(file, "{} {} {}", r, g, b)?;
    }
  }
  
  Ok(())
}