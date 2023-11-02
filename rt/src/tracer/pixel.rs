use crate::{parser::objects_file::Objects, gem::{dot::Dot, mat::Mat}};

use super::scene::Scene;

pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl RGB {
  pub fn new(r: u8, g: u8, b: u8) -> RGB {
    RGB { r, g, b }
  }

  pub fn from_array(array: &[u8; 3]) -> RGB {
    RGB::new(array[0], array[1], array[2])
  }

  /// the same color
  pub fn same(&self) -> RGB {
    RGB::new(self.r, self.g, self.b)
  }

  /// cyan color was choosen as the background color, if the ray does not hit any object properly (f.e.: hit the plane outside the light power distance, or just miss any object)
  pub fn background() -> RGB {
    RGB::new(0, 255, 255)
  }

  /// decrease the color brightness to represent the back side of the object
  /// 
  /// just division by the dark factor, hardcoded into method (not a real simulation of the light)
  pub fn dark_side(&self) -> RGB {
    let dark = 2;
    RGB::new(self.r / dark, self.g / dark, self.b / dark)
  }

  /// crete color affected by the light power (simple simulation, not a proper one)
  pub fn power_affected(
    r:u8, g:u8, b:u8,
    position:Dot,
    light_position: Dot,
    light_power_distance: f64,
  ) -> RGB {
    let mut rgb = RGB::new(r, g, b);
    let distance = position.d_dot(&light_position);
    if distance < light_power_distance {
      let power = (light_power_distance - distance) / light_power_distance;

      rgb.r = (rgb.r as f64 * power) as u8;
      rgb.g = (rgb.g as f64 * power) as u8;
      rgb.b = (rgb.b as f64 * power) as u8;

      rgb
    } else { RGB::background() }

  }

}

impl Scene {
  pub fn pixel_color(&self, row: usize, col: usize, good_to_trace: &Vec<Objects>) -> RGB {
    let ray = self.camera_ray_to_pixel(row, col);
    
    let mut rgb = RGB::background();
    let mut nearest_position = Dot::maximum();
    
    // iterate through the objects to find the nearest intersection with the ray
    for (index, object) in good_to_trace.iter().enumerate() {
      match object {
        Objects::Mat { .. } => {
          (rgb,nearest_position) =
          self.check_mat(
            rgb,
            nearest_position,
            ray,
            object,
            index,
            good_to_trace,
          );
        }
        Objects::Ball { color, position, radius, } => {}
        Objects::Box { color, position, size, } => {}
        Objects::Roll { color, position, radius, height, } => {}
      }
    }

    rgb

  }

  fn check_mat(
    &self,
    old_color: RGB,
    nearest_position: Dot,
    ray: Mat,
    object: &Objects,
    index: usize,
    good_to_trace: &Vec<Objects>
  ) -> (RGB, Dot) {
    // drop object with index
    let other_objects:Vec<Objects> = good_to_trace.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, o)| o.clone()).collect();

    (RGB::background(), Dot::maximum()) //todo: remove. dev gap
  }

}