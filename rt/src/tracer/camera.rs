use crate::gem::{spear::Spear, dot::Dot, mat::Mat};

use super::scene::Scene;

impl Scene {

  /// vector of the direction of the camera
  pub fn camera_vector(&self) -> Spear {
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
  pub fn camera_up_vector(&self) -> Spear {
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
  pub fn camera_left_vector(&self) -> Spear {
    let cv = self.camera_vector();
    let cuv = self.camera_up_vector();
    cuv.normal(&cv)
  }

  /// camera left plane, used for filtering the objects good to trace
  /// plane vector aimed inside the scene (to camera screen center)
  /// 
  /// based on
  /// - camera zoom position
  /// - camera left top pixel position
  /// - camera up vector
  /// - camera left vector
  /// 
  /// build vector v from zoom position to camera left top pixel position
  /// then build plane normal vector from v to camera up vector
  /// build plane from camera zoom position and plane normal
  pub fn camera_left_plane(&self) -> Mat {
    let camera_zoom_position = self.camera_zoom_position();
    let left_up_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_left_top_pixel(),
      ]
    );
    let mut plane_normal = left_up_vector.normal(&self.camera_up_vector());
    
    // check if the plane normal is zero, then simplify uses right vector
    if plane_normal.is_zero() { plane_normal = self.camera_left_vector().back(); }

    Mat::new(camera_zoom_position, plane_normal)
  }

  /// camera right plane, used for filtering the objects good to trace
  /// plane vector aimed inside the scene (to camera screen center)
  /// 
  /// based on
  /// - camera zoom position
  /// - camera right top pixel position
  /// - camera left vector
  /// - camera up vector
  /// 
  /// build vector v from zoom position to camera right top pixel position
  /// then build plane normal vector from camera up vector to v
  /// build plane from camera zoom position and plane normal
  pub fn camera_right_plane(&self) -> Mat {
    let camera_zoom_position = self.camera_zoom_position();
    let right_up_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_right_top_pixel(),
      ]
    );
    let mut plane_normal = self.camera_up_vector().normal(&right_up_vector);

    // check if the plane normal is zero, then simplify uses left vector
    if plane_normal.is_zero() { plane_normal = self.camera_left_vector(); }

    Mat::new(camera_zoom_position, plane_normal)

  }

  /// camera top plane, used for filtering the objects good to trace
  /// plane vector aimed inside the scene (to camera screen center)
  /// 
  /// based on
  /// - camera zoom position
  /// - camera left top pixel position
  /// - camera right top pixel position
  /// 
  /// build left_top_vector from zoom position to camera left top pixel position
  /// build right_top_vector from zoom position to camera right top pixel position
  /// build plane normal vector from left_top_vector to right_top_vector
  /// build plane from camera zoom position and plane normal
  pub fn camera_top_plane(&self) -> Mat {
    let camera_zoom_position = self.camera_zoom_position();
    let left_top_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_left_top_pixel(),
      ]
    );
    let right_top_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_right_top_pixel(),
      ]
    );
    let mut plane_normal = left_top_vector.normal(&right_top_vector);

    // check if the plane normal is zero, then simplify uses down vector
    if plane_normal.is_zero() { plane_normal = self.camera_up_vector().back(); }

    Mat::new(camera_zoom_position, plane_normal)
  }

  /// camera bottom plane, used for filtering the objects good to trace
  /// plane vector aimed inside the scene (to camera screen center)
  /// 
  /// based on
  /// - camera zoom position
  /// - camera left bottom pixel position
  /// - camera right bottom pixel position
  /// 
  /// build left_bottom_vector from zoom position to camera left bottom pixel position
  /// build right_bottom_vector from zoom position to camera right bottom pixel position
  /// build plane normal vector from right_bottom_vector to left_bottom_vector
  /// build plane from camera zoom position and plane normal
  pub fn camera_bottom_plane(&self) -> Mat {
    let camera_zoom_position = self.camera_zoom_position();
    let left_bottom_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_left_bottom_pixel(),
      ]
    );
    let right_bottom_vector = Spear::pp(
      &[
        camera_zoom_position.same(),
        self.camera_right_bottom_pixel(),
      ]
    );
    let mut plane_normal = right_bottom_vector.normal(&left_bottom_vector);

    // check if the plane normal is zero, then simplify uses up vector
    if plane_normal.is_zero() { plane_normal = self.camera_up_vector(); }

    Mat::new(camera_zoom_position, plane_normal)
  }

  /// camera_front_plane, used for filtering the objects good to trace
  /// 
  /// based on camera position, perpendicular to camera vector
  pub fn camera_front_plane(&self) -> Mat {
    Mat::new(self.camera_position(), self.camera_vector())
  }

  /// camera position
  pub fn camera_position(&self) -> Dot { Dot::from_array(self.camera.position) }

  /// camera zoom position
  pub fn camera_zoom_position(&self) -> Dot {
    let v = self.camera_vector();
    let dot = self.camera_position();
    // to move against vector, added minus, but also, it is possible to use vector.back() method, then minus is not needed
    dot.offset(&v, -(self.camera.zoom as f64))
  }

  /// from this point x,y,z coordinates will be calculated displacement
  /// 
  /// for the rays (ray direction end point), from the zoom point
  /// 
  /// (zoom point - moved back from the camera position against the camera vector),
  /// 
  /// to the pixel on the camera plane  
  pub fn camera_left_top_pixel(&self) -> Dot {
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

  fn camera_left_bottom_pixel(&self) -> Dot {
    self.camera_left_top_pixel().offset(&self.camera_up_vector().back(), (self.camera.height - 1) as f64)
  }

  fn camera_right_top_pixel(&self) -> Dot {
    self.camera_left_top_pixel().offset(&self.camera_left_vector().back(), (self.camera.width - 1) as f64)
  }

  fn camera_right_bottom_pixel(&self) -> Dot {
    self.camera_right_top_pixel().offset(&self.camera_up_vector().back(), (self.camera.height - 1) as f64)
  }

  /// ray from the camera zoom position to the pixel of the camera screen
  /// 
  /// calculated using displacement from the camera top left pixel
  /// 
  /// to down and right, according to the row and column
  pub fn camera_ray_to_pixel(&self, row: usize, col: usize) -> Mat {
    let camera_zoom_position = self.camera_zoom_position();
    let camera_left_top_pixel = self.camera_left_top_pixel();
    let camera_screen_pixel = camera_left_top_pixel
    .offset( // offset down
      &self.camera_up_vector().back(),
      row as f64
    )
    .offset( // offset right
      &self.camera_left_vector().back(),
      col as f64
    );
    
    Mat::pp(
      camera_zoom_position,
      camera_screen_pixel,
    )

  }

}