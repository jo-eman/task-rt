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

pub fn parse_camera_file(file_name: &str) -> Result<Camera, String> {
    let file = File::open(file_name).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut camera = Camera {
        width: 0,
        height: 0,
        output_file_name: "".to_string(),
        view_angle: 0,
        position: [0; 3],
        look_at: [0; 3],
        up: [0; 3],
    };
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
                let view_angle = words[1].parse::<i32>().map_err(|_| {
                    "View angle must be an integer between 1 and 120".to_string()
                })?;
                if view_angle < 1 || view_angle > 120 {
                    return Err("View angle must be an integer between 1 and 120".to_string());
                }
                camera.view_angle = view_angle;
                view_parsed = true;
            }
            "from" if !from_parsed && words.len() == 4 => {
                let position = [
                    words[1].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
                    words[2].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
                    words[3].parse::<i32>().map_err(|_| "Position must be an integer".to_string())?,
                ];
                camera.position = position;
                from_parsed = true;
            }
            "to" if !to_parsed && words.len() == 4 => {
                let look_at = [
                    words[1].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
                    words[2].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
                    words[3].parse::<i32>().map_err(|_| "Look at point must be an integer".to_string())?,
                ];
                camera.look_at = look_at;
                to_parsed = true;
            }
            "up" if !up_parsed && words.len() == 4 => {
                let up = [
                    words[1].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
                    words[2].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
                    words[3].parse::<i32>().map_err(|_| "Up vector must be an integer".to_string())?,
                ];
                camera.up = up;
                up_parsed = true;
            }
            "#" => {}
            _ if words.len() == 3 && !output_parsed => {
                let width = words[0].parse::<i32>().map_err(|_| {
                    "Width must be an integer greater than 0".to_string()
                })?;
                if width <= 0 || width > MAX_OUTPUT_PICTURE_SIDE_SIZE {
                    return Err(format!(
                        "Width must be an integer between 1 and {}",
                        MAX_OUTPUT_PICTURE_SIDE_SIZE
                    ));
                }
                let height = words[1].parse::<i32>().map_err(|_| {
                    "Height must be an integer greater than 0".to_string()
                })?;
                if height <= 0 || height > MAX_OUTPUT_PICTURE_SIDE_SIZE {
                    return Err(format!(
                        "Height must be an integer between 1 and {}",
                        MAX_OUTPUT_PICTURE_SIDE_SIZE
                    ));
                }
                let output_file_name = words[2].to_string();
                if !output_file_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    return Err("Output file name must contain only English letters, Arabic numbers, and underscores".to_string());
                }
                camera.width = width;
                camera.height = height;
                camera.output_file_name = output_file_name;
                output_parsed = true;
            }
            _ => {
                return Err(format!("Unknown command: {}", words[0]));
            }
        }
    }
    if !view_parsed {
        return Err("View angle not specified".to_string());
    }
    if !from_parsed {
        return Err("Camera position not specified".to_string());
    }
    if !to_parsed {
        return Err("Look at point not specified".to_string());
    }
    if !up_parsed {
        return Err("Up vector not specified".to_string());
    }
    if !output_parsed {
        return Err("Output file name not specified".to_string());
    }
    Ok(camera)
}