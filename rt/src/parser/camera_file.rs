use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_OUTPUT_PICTURE_SIDE_SIZE: i32 = 1024;

#[derive(Debug)]
pub struct Camera {
  pub width: i32,
  pub height: i32,
  pub output_file_name: String,
  pub zoom: usize, // displacement of rays start point from camera position to back
  pub position: [f64; 3],
  pub look_at: [f64; 3], // point in the front of the camera, to build camera vector
  pub up: [f64; 3],
}
impl Camera {
  pub fn parse_from_file(file_path: &str) -> Result<Camera, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    
    let mut width = 0;
    let mut height = 0;
    let mut output_file_name = "".to_string();
    let mut zoom = 0;
    let mut position = [0f64; 3];
    let mut look_at = [0f64; 3];
    let mut up = [0f64; 3];

    let mut zoom_parsed = false;
    let mut from_parsed = false;
    let mut to_parsed = false;
    let mut up_parsed = false;
    let mut output_parsed = false;
    
    for line in reader.lines() {
      let line = line.map_err(|e| e.to_string())?;
      let words: Vec<&str> = line.split_whitespace().collect();
      if words.is_empty() {
        continue;
      }
      match words[0] {
        "zoom" if !zoom_parsed && words.len() == 2 => {
          zoom = words[1].parse::<usize>().map_err(|_| {
            "Zoom must be greater than zero".to_string()
          })?;
          if zoom < 1 || zoom > usize::MAX {
            return Err(
              format!(
                "Zoom must be between 1 and {}", MAX_OUTPUT_PICTURE_SIDE_SIZE.pow(2)
              )
            );
          }
          zoom_parsed = true;
        }
        "from" if !from_parsed && words.len() == 4 => {
          position = [
          words[1].parse::<f64>().map_err(|_| "Position x must be an integer".to_string())?,
          words[2].parse::<f64>().map_err(|_| "Position y must be an integer".to_string())?,
          words[3].parse::<f64>().map_err(|_| "Position z must be an integer".to_string())?,
          ];
          from_parsed = true;
        }
        "to" if !to_parsed && words.len() == 4 => {
          look_at = [
          words[1].parse::<f64>().map_err(|_| "to x must be an integer".to_string())?,
          words[2].parse::<f64>().map_err(|_| "to y must be an integer".to_string())?,
          words[3].parse::<f64>().map_err(|_| "to z must be an integer".to_string())?,
          ];
          to_parsed = true;
        }
        "up" if !up_parsed && words.len() == 4 => {
          up = [
          words[1].parse::<f64>().map_err(|_| "Up vector x must be an integer".to_string())?,
          words[2].parse::<f64>().map_err(|_| "Up vector y must be an integer".to_string())?,
          words[3].parse::<f64>().map_err(|_| "Up vector z must be an integer".to_string())?,
          ];
          up_parsed = true;
        }
        "#" => {}
        _ if words.len() == 3 && !output_parsed => {
          width = words[0].parse::<i32>().map_err(|_| {
            "Width must be an integer greater than 0".to_string()
          })?;
          if width <= 0 || width > MAX_OUTPUT_PICTURE_SIDE_SIZE {
            return Err(format!(
              "Width must be an integer between 1 and {}",
              MAX_OUTPUT_PICTURE_SIDE_SIZE
            ));
          }
          height = words[1].parse::<i32>().map_err(|_| {
            "Height must be an integer greater than 0".to_string()
          })?;
          if height <= 0 || height > MAX_OUTPUT_PICTURE_SIDE_SIZE {
            return Err(format!(
              "Height must be an integer between 1 and {}",
              MAX_OUTPUT_PICTURE_SIDE_SIZE
            ));
          }
          output_file_name = words[2].to_string();
          if !output_file_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err("Output file name must contain only English letters, Arabic numbers, and underscores".to_string());
          }
          output_parsed = true;
        }
        _ => {
          return Err(format!("Unknown command: {}", line));
        }
      }
    }
    if !zoom_parsed {
      return Err(
        format!(
          "Camera zoom [1..{}] not specified", MAX_OUTPUT_PICTURE_SIDE_SIZE.pow(2)
        )
      );
    }
    if !from_parsed {
      return Err("Camera position [from x y z] not specified".to_string());
    }
    if !to_parsed {
      return Err("Look at point [to x y z] not specified".to_string());
    }
    if !up_parsed {
      return Err("Up vector end point [up x y z] not specified".to_string());
    }
    if !output_parsed {
      return Err("Output file [width height name] not specified".to_string());
    }

    let camera = Camera {
      width, height, output_file_name, zoom, position, look_at, up
    };

    Ok(camera)
  }
}