use std::fs::File;
use std::io::{BufRead, BufReader};

// Define Object struct
#[derive(Debug, Clone, Copy)]
pub enum Objects {
    Ball {
        color: [u8; 3],
        position: [f64; 3],
        radius: f64,
    },
    Box {
        color: [u8; 3],
        position: [f64; 3],
        size: f64,
    },
    Roll {
        color: [u8; 3],
        position: [f64; 3],
        radius: f64,
        height: f64,
    },
    Mat {
        color: [u8; 3],
        position: [f64; 3],
        normal: [f64; 3],
    },
}

// Implement Object parser
impl Objects {
    // Define a function to parse Object from file
    pub fn parse_from_file(file_path: &str) -> Result<Vec<Objects>, String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut objects = Vec::new();

        for line in reader.lines() {
            let line = line.map_err(|e| e.to_string())?;
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.is_empty() || words[0] == "#" || words.len() < 8 {
                continue;
            }
            match words[3] {
                "ball" if words.len() == 8 => {
                    let color = [
                        words[0].parse::<u8>().map_err(|_| {
                            "Color R must be an integer 0 to 255".to_string()
                        })?,
                        words[1].parse::<u8>().map_err(|_| {
                            "Color G must be an integer 0 to 255".to_string()
                        })?,
                        words[2].parse::<u8>().map_err(|_| {
                            "Color B must be an integer 0 to 255".to_string()
                        })?,
                    ];
                    let position = [
                        words[4].parse::<f64>().map_err(|_| {
                            "Position x must be an integer".to_string()
                        })?,
                        words[5].parse::<f64>().map_err(|_| {
                            "Position y must be an integer".to_string()
                        })?,
                        words[6].parse::<f64>().map_err(|_| {
                            "Position z must be an integer".to_string()
                        })?,
                    ];
                    let radius = words[7].parse::<f64>().map_err(|_| {
                        "Radius must be an integer".to_string()
                    })?;
                    objects.push(Objects::Ball {
                        color,
                        position,
                        radius,
                    });
                }
                "box" if words.len() == 8 => {
                    let color = [
                        words[0].parse::<u8>().map_err(|_| {
                            "Color R must be an integer 0 to 255".to_string()
                        })?,
                        words[1].parse::<u8>().map_err(|_| {
                            "Color G must be an integer 0 to 255".to_string()
                        })?,
                        words[2].parse::<u8>().map_err(|_| {
                            "Color B must be an integer 0 to 255".to_string()
                        })?,
                    ];
                    let position = [
                        words[4].parse::<f64>().map_err(|_| {
                            "Position x must be an integer".to_string()
                        })?,
                        words[5].parse::<f64>().map_err(|_| {
                            "Position y must be an integer".to_string()
                        })?,
                        words[6].parse::<f64>().map_err(|_| {
                            "Position z must be an integer".to_string()
                        })?,
                    ];
                    let size = words[7].parse::<f64>().map_err(|_| {
                        "Size must be an integer".to_string()
                    })?;
                    objects.push(Objects::Box {
                        color,
                        position,
                        size,
                    });
                }
                "roll" if words.len() == 9 => {
                    let color = [
                        words[0].parse::<u8>().map_err(|_| {
                            "Color R must be an integer 0 to 255".to_string()
                        })?,
                        words[1].parse::<u8>().map_err(|_| {
                            "Color G must be an integer 0 to 255".to_string()
                        })?,
                        words[2].parse::<u8>().map_err(|_| {
                            "Color B must be an integer 0 to 255".to_string()
                        })?,
                    ];
                    let position = [
                        words[4].parse::<f64>().map_err(|_| {
                            "Position x must be an integer".to_string()
                        })?,
                        words[5].parse::<f64>().map_err(|_| {
                            "Position y must be an integer".to_string()
                        })?,
                        words[6].parse::<f64>().map_err(|_| {
                            "Position z must be an integer".to_string()
                        })?,
                    ];
                    let radius = words[7].parse::<f64>().map_err(|_| {
                        "Radius must be an integer".to_string()
                    })?;
                    let height = words[8].parse::<f64>().map_err(|_| {
                        "Height must be an integer".to_string()
                    })?;
                    objects.push(Objects::Roll {
                        color,
                        position,
                        radius,
                        height,
                    });
                }
                "mat" if words.len() == 10 => {
                    let color = [
                        words[0].parse::<u8>().map_err(|_| {
                            "Color R must be an integer 0 to 255".to_string()
                        })?,
                        words[1].parse::<u8>().map_err(|_| {
                            "Color G must be an integer 0 to 255".to_string()
                        })?,
                        words[2].parse::<u8>().map_err(|_| {
                            "Color B must be an integer 0 to 255".to_string()
                        })?,
                    ];
                    let position = [
                        words[4].parse::<f64>().map_err(|_| {
                            "Position x must be an integer".to_string()
                        })?,
                        words[5].parse::<f64>().map_err(|_| {
                            "Position y must be an integer".to_string()
                        })?,
                        words[6].parse::<f64>().map_err(|_| {
                            "Position z must be an integer".to_string()
                        })?,
                    ];
                    let normal = [
                        words[7].parse::<f64>().map_err(|_| {
                            "Normal x must be an integer".to_string()
                        })?,
                        words[8].parse::<f64>().map_err(|_| {
                            "Normal y must be an integer".to_string()
                        })?,
                        words[9].parse::<f64>().map_err(|_| {
                            "Normal z must be an integer".to_string()
                        })?,
                    ];
                    objects.push(Objects::Mat { color, position, normal });
                }
                _ => {
                    return Err(format!("Unknown command: {}", line));
                }
            }
        }

        Ok(objects)
    }
}