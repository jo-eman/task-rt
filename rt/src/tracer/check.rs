use crate::parser::objects_file::Objects;

use super::scene::Scene;

impl Scene {
  /// check if the object is good to trace, otherwise ignore it.
  /// Reasons to not trace:
  /// 1. object is not enough far from the camera (the object safe point is below the camera pixels plane)
  /// 2. object is to far from the light (from the light position, along the light power distance)
  /// 3. object is not in the camera view.
  /// Below the x4 planes, builded from the camera zoom position, through the camera pixels area corners.
  /// Each of x4 planes will be build on two vectors.
  /// After that, the object center point will be moved(offset) close to the plane, along plane normal vector, to some biggest distance, unique for each object properties.
  /// 
  /// The Mat(flat plane) object, will be ignored, in case if the plane is below the camera pixels plane(for this the planes must be also parallel).
  pub fn good_to_trace(&self, objects: Vec<Objects>) -> Vec<Objects> {
    todo!("good_to_trace")
  }
}