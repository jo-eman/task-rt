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
  
  /// vector of the direction of the camera
  fn camera_vector(&self) -> Spear {
    let camera = &self.camera;
    let from = Dot::from_array(camera.position);
    let to = Dot::from_array(camera.look_at);

    let mut v = Spear::pp(&[from, to]);
    // check if the vector is zero, then increase the z component by 1.0
    // to make z the direction of the camera
    if v.is_zero() { v.z += 1.0; }
    v
  }

  /// vector of the up direction of the camera
  fn camera_up_vector(&self) -> Spear {
    let camera = &self.camera;
    let from = Dot::from_array(camera.position);
    let to = Dot::from_array(camera.up);

    let mut cuv = Spear::pp(&[from, to]);
    
    // check if the vector is zero or parallel to camera vector,
    // then increase the y component by 1.0, to make y the up direction
    let cv = self.camera_vector();
    if cuv.is_zero() || cuv.is_ll(&cv) { cuv.y += 1.0; }
    // now double calculate the normal vector, to make it perpendicular to the camera vector
    
    cv.normal(&cuv.normal(&cv))
  }

  /// vector of the left direction of the camera
  fn camera_left_vector(&self) -> Spear {
    let cv = self.camera_vector();
    let cuv = self.camera_up_vector();
    cuv.normal(&cv)
  }

  /// camera position
  fn camera_position(&self) -> Dot { Dot::from_array(self.camera.position) }

  /// camera zoom position
  fn camera_zoom_position(&self) -> Dot {
    let v = self.camera_vector();
    let dot = self.camera_position();
    // to move against vector, added minus, but also, it is possible to use vector.back() method, then minus is not needed
    dot.offset(&v, -(self.camera.zoom as f64))
  }

  /// from this point x,y,z coordinates will be calculated displacement
  /// for the rays (ray direction end point), from the zoom point
  /// (zoom point - moved back from the camera position against the camera vector),
  /// to the pixel on the camera plane  
  pub fn left_top_camera_pixel(&self) -> Dot {
    let mut camera_position = self.camera_position();
    let camera_left_vector = self.camera_left_vector();
    let camera_up_vector = self.camera_up_vector();

    // move camera position to the left
    camera_position = camera_position.offset(
      &camera_left_vector,
      self.camera.width as f64 / 2.0 - 0.5, // -0.5 it is like the center of the most left pixels column, facepalm
    );

    // move camera position to the top
    camera_position.offset(
      &camera_up_vector,
      self.camera.height as f64 / 2.0 - 0.5, // -0.5 it is like the center of the most top pixels row, facepalm
    )
    
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
    println!("left top camera pixel: {:?}", self.left_top_camera_pixel());
  }

  pub fn dev_check_dot_above_below_plane(&self){
    let origin = Dot::trione();
    let normal = Spear::trione();
    let p = Mat::new(origin, normal);
    let dot_below = Dot::new(-1.0,-1.0,-1.0);
    let dot_in = Dot::zero();
    let dot_above = Dot::new(1.0,1.0,1.0);

    println!("true expected {}",dot_below.is_below(&p));
    println!("true expected {}",dot_above.is_above(&p));
    println!("false expected {}",dot_above.is_below(&p));
    println!("false expected {}",dot_below.is_above(&p));
    println!("false expected {}",dot_in.is_above(&p));
    println!("false expected {}",dot_in.is_below(&p));

  }

}