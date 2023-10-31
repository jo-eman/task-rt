use crate::{
  parser::{ camera_file::Camera, light_file::Light, objects_file::Objects },
  gem::{dot::Dot, spear::Spear, mat::Mat}
};

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
  
  pub fn dev_print(&self) {
    println!("camera position: {:?}", self.camera_position());
    println!("camera vector: {:?}", self.camera_vector());
    println!("camera up vector: {:?}", self.camera_up_vector());
    println!("camera left vector: {:?}", self.camera_left_vector());
    println!("camera zoom position: {:?}", self.camera_zoom_position());
    println!("left top camera pixel: {:?}", self.camera_left_top_pixel());
  }

  pub fn dev_check_dot_above_below_plane(&self){
    let origin = Dot::trione();
    let normal = Spear::trione();
    let p = Mat::new(origin, normal);
    let dot_below = Dot::new(-1.0,-1.0,-1.0);
    let dot_in = Dot::zero();
    let dot_above = Dot::new(1.0,1.0,1.0);

    println!("true is {}",dot_below.is_below(&p));
    println!("true is {}",dot_above.is_above(&p));
    println!("false is {}",dot_above.is_below(&p));
    println!("false is {}",dot_below.is_above(&p));
    println!("false is {}",dot_in.is_above(&p));
    println!("false is {}",dot_in.is_below(&p));

  }

}