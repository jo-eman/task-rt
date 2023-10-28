use crate::parser::{camera_file::Camera, light_file::Light, objects_file::Objects};

pub struct Scene {
  pub camera: Camera,
  pub light: Light,
  pub objects: Vec<Objects>,
}

impl Scene {
  pub fn new(camera: Camera, light: Light, objects: Vec<Objects>) -> Scene {
    Scene {
      camera,
      light,
      objects,
    }
  }
  
  /** vector of the direction of the camera */
  fn camera_vector(&self) -> [f64; 3] {
    let camera = &self.camera;
    let camera_position = camera.position;
    let camera_look_at = camera.look_at;
    [
      (camera_look_at[0] - camera_position[0]) as f64,
      (camera_look_at[1] - camera_position[1]) as f64,
      (camera_look_at[2] - camera_position[2]) as f64,
    ]
  }

  /**
    from this point x,y,z coordinates will be calculated displacement
    for the rays (ray direction end point), from the zoom point
    (zoom point - moved back from the camera position against the camera vector),
    to the pixel on the camera plane
  */
  pub fn left_top_corner_point_of_camera_pixels(&self) -> [f64; 3] {
    let camera = &self.camera;
    let zoom = camera.zoom as f64;
    let camera_position = camera.position;
    let camera_look_at = camera.look_at;
    let camera_up = camera.up;
    // camera vector
    let camera_vector = [
      camera_look_at[0] - camera_position[0],
      camera_look_at[1] - camera_position[1],
      camera_look_at[2] - camera_position[2],
    ];
    // move zoom point back from the camera position against the camera vector
    
    [3_f64,3_f64,3_f64] //todo dev gap

  }

  pub fn dummy_trace(&self) -> Vec<u8> {
    let width = self.camera.width;
    let height = self.camera.height;
    let mut data = vec![0; (width * height * 3) as usize];
    for row in 0..height {
      for col in 0..width {
        let index = ((row * width + col) * 3) as usize;
        data[index] = 0;
        data[index + 1] = 255;
        data[index + 2] = 0;
        
        if row == 0 || row == height - 1 || col == 0 || col == width - 1 {
          data[index] = 255;
          data[index + 1] = 0;
          data[index + 2] = 0;
        }
      }
    }
    data
    
  }
  
}