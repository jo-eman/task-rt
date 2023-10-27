use std::fs::File;
use std::io::{BufRead, BufReader};

// Define Light struct
#[derive(Debug)]
pub struct Light {
  // Define fields for Light
  pub power: u8,
  pub color: [u8; 3],
  pub position: [i32; 3],
}

// Implement Light parser
impl Light {
  // Define a function to parse Light from file
  pub fn parse_from_file(file_path: &str) -> Result<Light, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    // Initialize variables to store parsed values
    let mut power = 0;
    let mut color = [0; 3];
    let mut position = [0; 3];
    
    // Flags to check if parameters are parsed
    let mut power_parsed = false;
    let mut color_parsed = false;
    let mut from_parsed = false;
    
    for line in reader.lines() {
      let line = line.map_err(|e| e.to_string())?;
      let words: Vec<&str> = line.split_whitespace().collect();
      if words.is_empty() {
        continue;
      }
      match words[0] {
        "power" if !power_parsed && words.len() == 2 => {
          power = words[1].parse::<u8>().map_err(|_| {
            "Power must be an integer 0 to 255".to_string()
          })?;
          power_parsed = true;
        }
        "color" if !color_parsed && words.len() == 4 => {
          color = [
          words[1].parse::<u8>().map_err(|_| "Color R must be an integer 0 to 255".to_string())?,
          words[2].parse::<u8>().map_err(|_| "Color G must be an integer 0 to 255".to_string())?,
          words[3].parse::<u8>().map_err(|_| "Color B must be an integer 0 to 255".to_string())?,
          ];
          color_parsed = true;
        }
        "from" if !from_parsed && words.len() == 4 => {
          position = [
          words[1].parse::<i32>().map_err(|_| "Position x must be an integer".to_string())?,
          words[2].parse::<i32>().map_err(|_| "Position y must be an integer".to_string())?,
          words[3].parse::<i32>().map_err(|_| "Position z must be an integer".to_string())?,
          ];
          from_parsed = true;
        }
        "#" => {}
        _ => {
          return Err(format!("Unknown command: {}", line));
        }
      }
    }
    
    if !power_parsed {
      return Err("Light power [power 0-255] is not specified".to_string());
    }
    
    if !color_parsed {
      return Err("Light color [color 0-255 0-255 0-255] is not specified".to_string());
    }
    
    if !from_parsed {
      return Err("Light position [from x y z] is not specified".to_string());
    }
    
    Ok(Light { power, color, position, })
    
  }
}
