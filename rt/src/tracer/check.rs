use crate::{parser::objects_file::Objects, gem::{mat::Mat, dot::Dot, spear::Spear}};

use super::scene::Scene;

impl Scene {
  /// check if the object is good to trace, otherwise ignore it.
  /// Reasons to not trace:
  /// 1. object is not enough far from the camera (the object safe point is below the camera x5 planes).
  /// The object center point will be moved(offset) close to the plane, along plane normal vector, to some biggest distance, unique for each object properties.
  /// 2. object is too far from the light (from the light position, along the light power distance)
  /// 
  /// The Mat(flat plane) object, will be ignored, in case if the plane is below the camera pixels plane(for this the planes must be also parallel).
  pub fn good_to_trace(&self, objects: &Vec<Objects>) -> Vec<Objects> {
    let mut good_to_trace:Vec<Objects> = Vec::new();
    
    // iterate over the objects
    for object in objects {
      match object {
        Objects::Mat { position, normal, .. } => {
          if self.mat_is_good(position, normal) { good_to_trace.push(object.clone());}
        }
        Objects::Ball { position, radius, .. } => {}
        Objects::Box { position, size, .. } => {}
        Objects::Roll { position, radius, height, .. } => {}
      }

    }

    good_to_trace
  }

  fn mat_is_good(&self, position: &[f64; 3], normal: &[f64; 3]) -> bool {
    let p = Mat::new(
      Dot::from_array(*position),
      Spear::from_array(*normal)
    );
    
    let camera_front_plane = self.camera_front_plane();
    let camera_left_plane = self.camera_left_plane();
    let camera_right_plane = self.camera_right_plane();
    let camera_top_plane = self.camera_top_plane();
    let camera_bottom_plane = self.camera_bottom_plane();
    // if plane is below any camera planes, or the same as any camera plane, then ignore it
    p.is_zero() ||
    p.is_ll(&camera_front_plane) && !p.origin.is_above(&camera_front_plane) ||
    p.is_ll(&camera_left_plane) && !p.origin.is_above(&camera_left_plane) ||
    p.is_ll(&camera_right_plane) && !p.origin.is_above(&camera_right_plane) ||
    p.is_ll(&camera_top_plane) && !p.origin.is_above(&camera_top_plane) ||
    p.is_ll(&camera_bottom_plane) && !p.origin.is_above(&camera_bottom_plane)
    
  }

  fn check_ball(&self, position: &[f64; 3], radius: &f64) -> bool {todo!("check_ball")}

  fn check_box(&self, position: &[f64; 3], size: &f64) -> bool {todo!("check_box")}

  fn check_roll(&self, position: &[f64; 3], radius: &f64, height: &f64) -> bool {todo!("check_roll")}

}