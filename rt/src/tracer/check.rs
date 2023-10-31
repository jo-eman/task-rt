use crate::parser::objects_file::Objects;

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
          if self.check_mat(position, normal) { good_to_trace.push(object.clone());}
        }
        Objects::Ball { position, radius, .. } => {}
        Objects::Box { position, size, .. } => {}
        Objects::Roll { position, radius, height, .. } => {}
      }

    }

    good_to_trace
  }

  fn check_mat(&self, position: &[f64; 3], normal: &[f64; 3]) -> bool {
    
    false
  }

  fn check_ball(&self, position: &[f64; 3], radius: &f64) -> bool {todo!("check_ball")}

  fn check_box(&self, position: &[f64; 3], size: &f64) -> bool {todo!("check_box")}

  fn check_roll(&self, position: &[f64; 3], radius: &f64, height: &f64) -> bool {todo!("check_roll")}

}