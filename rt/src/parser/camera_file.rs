use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_OUTPUT_PICTURE_SIDE_SIZE: i32 = 10000;

#[derive(Debug)]
pub struct Camera {
  width: i32,
  height: i32,
  output_file_name: String,
  view_angle: i32,
  position: [i32; 3],
  look_at: [i32; 3],
  up: [i32; 3],
}
impl Camera {
  pub fn parse_from_file(file_path: &str) -> Result<Camera, String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    // let mut camera = Camera {
    //   width: 0,
    //   height: 0,
    //   output_file_name: "".to_string(),
    //   view_angle: 0,
    //   position: [0; 3],
    //   look_at: [0; 3],
    //   up: [0; 3],
    // };

    let mut width = 0;
    let mut height = 0;
    let mut output_file_name = "".to_string();
    let mut view_angle = 0;
    let mut position = [0; 3];
    let mut look_at = [0; 3];
    let mut up = [0; 3];

    let mut view_parsed = false;
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
        "view" if !view_parsed && words.len() == 2 => {
          view_angle = words[1].parse::<i32>().map_err(|_| {
            "View angle must be an integer between 1 and 120".to_string()
          })?;
          if view_angle < 10 || view_angle > 120 {
            return Err("View angle must be an integer between 10 and 120".to_string());
          }
          view_parsed = true;
        }
        "from" if !from_parsed && words.len() == 4 => {
          position = [
          words[1].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
          words[2].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
          words[3].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
          ];
          from_parsed = true;
        }
        "to" if !to_parsed && words.len() == 4 => {
          look_at = [
          words[1].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
          words[2].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
          words[3].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
          ];
          to_parsed = true;
        }
        "up" if !up_parsed && words.len() == 4 => {
          up = [
          words[1].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
          words[2].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
          words[3].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
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
    if !view_parsed {
      return Err("Camera view angle [view 10-120] not specified".to_string());
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
      return Err("Output file [width height filename] not specified".to_string());
    }

    let camera = Camera {
      width, height, output_file_name, view_angle, position, look_at, up
    };

    Ok(camera)
  }
}