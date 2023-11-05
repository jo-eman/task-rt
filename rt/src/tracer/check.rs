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
    let mut bad = 0;
    let mut good_to_trace:Vec<Objects> = Vec::new();
    
    // iterate over the objects
    for object in objects {
      match object {
        Objects::Mat { position, normal, .. } => {
          if self.mat_is_good(position, normal) { good_to_trace.push(object.clone());} else {self.print_bad(object); bad +=1;}
        }
        Objects::Ball { position, radius, .. } => {
          if self.ball_is_good(position, radius) { good_to_trace.push(object.clone());} else {self.print_bad(object); bad +=1;}
        }
        Objects::Box { position, size, .. } => {
          if self.box_is_good(position, size) { good_to_trace.push(object.clone());} else {self.print_bad(object); bad +=1;}
        }
        Objects::Roll { position, radius, height, .. } => {
          if self.roll_is_good(position, radius, height) { good_to_trace.push(object.clone());} else {self.print_bad(object); bad +=1;}
        }
      }
      
    }

    self.print_stats(objects.len(), good_to_trace.len(), bad);
    
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
    // if plane is zero, then ignore it
    // if plane is too far from the light, then ignore it
    // if plane is below any camera planes, or the same as any camera plane, then ignore it
    !(// negotiation of the conditions, to return false, if any of them is true
      p.is_zero()
      || Dot::from_array(self.light.position).d_mat(&p) > self.light.power
      || p.is_ll(&camera_front_plane) && !p.origin.is_above(&camera_front_plane)
      || p.is_ll(&camera_left_plane) && !p.origin.is_above(&camera_left_plane)
      || p.is_ll(&camera_right_plane) && !p.origin.is_above(&camera_right_plane)
      || p.is_ll(&camera_top_plane) && !p.origin.is_above(&camera_top_plane)
      || p.is_ll(&camera_bottom_plane) && !p.origin.is_above(&camera_bottom_plane)
    )
    
  }
  
  fn ball_is_good(&self, position: &[f64; 3], radius: &f64) -> bool {
    let c = Dot::from_array(*position);
    let sun = Dot::from_array(self.light.position);
    let v_light = Spear::pp( &[ c, sun, ] );
    let nearest_dot = c.offset(&v_light, *radius);
    let distance_to_sun = sun.d_dot(&nearest_dot);
    
    let camera_front_plane = self.camera_front_plane();
    let camera_left_plane = self.camera_left_plane();
    let camera_right_plane = self.camera_right_plane();
    let camera_top_plane = self.camera_top_plane();
    let camera_bottom_plane = self.camera_bottom_plane();

    !(// negotiation of the conditions, to return false, if any of them is true
      radius <= &0.0
      || distance_to_sun > self.light.power
      || c.d_mat(&camera_front_plane) < *radius
      || c.d_mat(&camera_front_plane) >= *radius && c.is_below(&camera_front_plane)
      || c.d_mat(&camera_left_plane) >= *radius && c.is_below(&camera_left_plane)
      || c.d_mat(&camera_right_plane) >= *radius && c.is_below(&camera_right_plane)
      || c.d_mat(&camera_top_plane) >= *radius && c.is_below(&camera_top_plane)
      || c.d_mat(&camera_bottom_plane) >= *radius && c.is_below(&camera_bottom_plane)
    )
  }
  
  fn box_is_good(&self, position: &[f64; 3], size: &f64) -> bool {
    let c = Dot::from_array(*position);
    let sun = Dot::from_array(self.light.position);
    let v_light = Spear::pp( &[ c, sun, ] );
    let d = (3.0 * (size / 2.0).powi(2)).sqrt(); // distance from box center to box corner
    let nearest_dot = c.offset(
      &v_light,
      d,
    );

    let camera_front_plane = self.camera_front_plane();
    let camera_left_plane = self.camera_left_plane();
    let camera_right_plane = self.camera_right_plane();
    let camera_top_plane = self.camera_top_plane();
    let camera_bottom_plane = self.camera_bottom_plane();

    !(// negotiation of the conditions, to return false, if any of them is true
      size <= &0.0
      || sun.d_dot(&nearest_dot) > self.light.power
      || c.d_mat(&camera_front_plane) < d
      || c.d_mat(&camera_front_plane) >= d && c.is_below(&camera_front_plane)
      || c.d_mat(&camera_left_plane) >= d && c.is_below(&camera_left_plane)
      || c.d_mat(&camera_right_plane) >= d && c.is_below(&camera_right_plane)
      || c.d_mat(&camera_top_plane) >= d && c.is_below(&camera_top_plane)
      || c.d_mat(&camera_bottom_plane) >= d && c.is_below(&camera_bottom_plane)
    )

  }
  
  /// just recall box_is_good() with the biggest size of the roll
  fn roll_is_good(&self, position: &[f64; 3], radius: &f64, height: &f64) -> bool {
    radius > &0.0 && height > &0.0 && self.box_is_good(
      position,
      &(radius * 2.0).max(*height),
    )
  }
  
  /// prints the bad objects, that are not good to trace
  fn print_bad(&self, object: &Objects) {
    println!("ignored from trace: {:?}", object);
  }

  fn print_stats(&self,
    objects_number:usize,
    good_number: usize,
    bad_number: usize
  ) {
    let stats = format!("= objects: {}, good: {}, bad: {} =", objects_number, good_number, bad_number);
    let line = "=".repeat(stats.len());
    println!("{}\n{}\n{}", line, stats, line);
  }

}