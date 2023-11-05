use crate::{
  parser::objects_file::Objects,
  gem::{
    dot::Dot, mat::Mat,
    gem::Gem, utils::F64xyz,
    spear::Spear
  }
};

use super::scene::Scene;

pub struct RGB {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub fresh: bool,
}

impl RGB {
  pub fn new(r: u8, g: u8, b: u8) -> RGB {
    RGB { r, g, b, fresh: true }
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
  /// just division by the dark factor,
  /// hardcoded into method (not a real simulation of the light)
  ///   
  /// FIRES ONLY ONCE, ON FRESH CREATED COLOR,
  /// after that, the same color is returned
  pub fn dark_side(&self) -> RGB {
    if self.fresh{
    let dark = 2;
    let mut rgb = RGB::new(self.r / dark, self.g / dark, self.b / dark);
    rgb.fresh = false;
    rgb
    } else {self.same()}
  }
  
  /// crete color affected by the light power (simple simulation, not a proper one)
  pub fn power_affected(
    rgb:[u8;3],
    color_position:Dot,
    light_source_position: Dot,
    light_color: RGB,
    light_power_distance: f64,
  ) -> RGB {
    let mut rgb = RGB::from_array(&rgb);
    let distance = color_position.d_dot(&light_source_position);
    if distance < light_power_distance {
      let power_coef =
      (light_power_distance - distance) / light_power_distance;
      
      let shade_r = light_color.r as f64 / 255_f64;
      let shade_g = light_color.g as f64 / 255_f64;
      let shade_b = light_color.b as f64 / 255_f64;
      
      rgb.r = (rgb.r as f64 * power_coef * shade_r) as u8;
      rgb.g = (rgb.g as f64 * power_coef * shade_g) as u8;
      rgb.b = (rgb.b as f64 * power_coef * shade_b) as u8;
      
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
        //todo: implement
        Objects::Ball { .. } => {
          (rgb,nearest_position) =
          self.check_ball(
            rgb,
            nearest_position,
            ray,
            object,
            index,
            good_to_trace,
          );
        }
        Objects::Box { .. } => {
          (rgb,nearest_position) =
          self.check_box(
            rgb,
            nearest_position,
            ray,
            object,
            index,
            good_to_trace,
          );
        }
        Objects::Roll { .. } => {
          (rgb,nearest_position) =
          self.check_roll(
            rgb,
            nearest_position,
            ray,
            object,
            index,
            good_to_trace,
          );
        }
      }
    }
    
    rgb
    
  }
  
  
  /// check the hit point is on the dark side of the box
  fn is_box_dark_side(
    camera_ray_hit_xyz: Dot,
    light_position: Dot,
    center: Dot,
    size: f64,
  ) -> bool {

    let light_ray = Mat::new(
      light_position,
      Spear::pp(
        &[
          light_position,
          camera_ray_hit_xyz,
        ]
      ),
    );
    let light_xyz = Gem::ray_x_box(&light_ray, &center, size);

    light_xyz.d_dot(&light_position) < f64::Z9X9 * camera_ray_hit_xyz.d_dot(&light_position)
  }

  fn check_box(
    &self,
    old_color: RGB,
    nearest_position: Dot,
    ray: Mat,
    object: &Objects,
    index: usize,
    good_to_trace: &Vec<Objects>
  ) -> (RGB, Dot) {
    let light_position = Dot::from_array(self.light.position);
    let object = object.clone(); // to avoid borrow checker
    // drop object with index, which is incoming object
    let other_objects:Vec<Objects> = good_to_trace.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, o)| o.clone()).collect();
    
    // find the object intersection point and color, or set color to background, and intersection to must far point, to avoid any rust "magic"
    let (obj_pixel_color, obj_pixel_position) = match object {
      Objects::Box { color, position, size } => {
        let center = Dot::from_array(position);
        let xyz = Gem::ray_x_box(&ray, &center, size);
        let mut rgb = RGB::power_affected(
          color,
          xyz,
          light_position,
          RGB::from_array(&self.light.color),
          self.light.power
        );

        if rgb.fresh
        && Scene::is_box_dark_side( xyz, light_position, center, size, )
        {rgb = rgb.dark_side();}
        
        (
          rgb,
          xyz
        )
        
      }
      _ => {(RGB::background(), Dot::maximum())}
    };
    
    let (mut pixel_color, mut pixel_position) = (RGB::background(), Dot::maximum());
    
    if obj_pixel_position.d_dot(&light_position) <= self.light.power{
      (pixel_color, pixel_position) = (obj_pixel_color, obj_pixel_position);
    }
    
    // here, build the ray to light source, iterate the other_objects , and
    // if there is some other intersection closer to light source than
    // obj_pixel_position, than implement dark_side method to slow down the color
    // and break the loop, because there is no need to check other objects
    
    let ray_to_light = Mat::new(
      obj_pixel_position,
      Spear::pp(
        &[
          obj_pixel_position,
          Dot::from_array(self.light.position),
        ]
      )
    );
    
    for object in other_objects {
      match object {
        Objects::Mat { position, normal, .. } => {
          let mat_origin = Dot::from_array(position);
          let mat_normal = Spear::from_array(normal);
          let xyz = Gem::ray_x_mat(&ray_to_light, &Mat::new(mat_origin, mat_normal));
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Ball { position, radius, .. } => {
          let ball_center = Dot::from_array(position);
          let xyz = Gem::ray_x_ball(&ray_to_light, &ball_center, radius);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Box { position, size, .. } => {
          let box_center = Dot::from_array(position);
          let xyz = Gem::ray_x_box(&ray_to_light, &box_center, size);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Roll { position, radius, height, .. } => {
          let roll_center = Dot::from_array(position);
          let xyz = Gem::ray_x_roll(&ray_to_light, &roll_center, radius, height);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
      }
    }
    
    // compare with nearest_position, and return the nearest one (with color)
    if pixel_position.d_dot(&ray.origin) < nearest_position.d_dot(&ray.origin) { (pixel_color, pixel_position) } else { (old_color, nearest_position) }

  }
  
  /// check the hit point is on the dark side of the roll
  fn is_roll_dark_side(
    camera_ray_hit_xyz: Dot,
    light_position: Dot,
    center: Dot,
    radius: f64,
    height: f64,
  ) -> bool {

    let light_ray = Mat::new(
      light_position,
      Spear::pp(
        &[
          light_position,
          camera_ray_hit_xyz,
        ]
      ),
    );
    let light_xyz = Gem::ray_x_roll(&light_ray, &center, radius, height);

    light_xyz.d_dot(&light_position) < f64::Z9X9 * camera_ray_hit_xyz.d_dot(&light_position)

  }

  fn check_roll(
    &self,
    old_color: RGB,
    nearest_position: Dot,
    ray: Mat,
    object: &Objects,
    index: usize,
    good_to_trace: &Vec<Objects>
  ) -> (RGB, Dot) {
    let light_position = Dot::from_array(self.light.position);
    let object = object.clone(); // to avoid borrow checker
    // drop object with index, which is incoming object
    let other_objects:Vec<Objects> = good_to_trace.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, o)| o.clone()).collect();
    
    // find the object intersection point and color, or set color to background, and intersection to must far point, to avoid any rust "magic"
    let (obj_pixel_color, obj_pixel_position) = match object {
      Objects::Roll { color, position, radius, height } => {
        let center = Dot::from_array(position);
        let xyz = Gem::ray_x_roll(&ray, &center, radius, height);
        let mut rgb = RGB::power_affected(
          color,
          xyz,
          light_position,
          RGB::from_array(&self.light.color),
          self.light.power
        );

        if rgb.fresh
        && Scene::is_roll_dark_side( xyz, light_position, center, radius, height, )
        {rgb = rgb.dark_side();}

        (
          rgb,
          xyz
        )
        
      }
      _ => {(RGB::background(), Dot::maximum())}
    };
    
    let (mut pixel_color, mut pixel_position) = (RGB::background(), Dot::maximum());
    
    if obj_pixel_position.d_dot(&light_position) <= self.light.power{
      (pixel_color, pixel_position) = (obj_pixel_color, obj_pixel_position);
    }
    
    // here, build the ray to light source, iterate the other_objects , and
    // if there is some other intersection closer to light source than
    // obj_pixel_position, than implement dark_side method to slow down the color
    // and break the loop, because there is no need to check other objects
    
    let ray_to_light = Mat::new(
      obj_pixel_position,
      Spear::pp(
        &[
          obj_pixel_position,
          Dot::from_array(self.light.position),
        ]
      )
    );
    
    for object in other_objects {
      match object {
        Objects::Mat { position, normal, .. } => {
          let mat_origin = Dot::from_array(position);
          let mat_normal = Spear::from_array(normal);
          let xyz = Gem::ray_x_mat(&ray_to_light, &Mat::new(mat_origin, mat_normal));
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Ball { position, radius, .. } => {
          let ball_center = Dot::from_array(position);
          let xyz = Gem::ray_x_ball(&ray_to_light, &ball_center, radius);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Box { position, size, .. } => {
          let box_center = Dot::from_array(position);
          let xyz = Gem::ray_x_box(&ray_to_light, &box_center, size);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
        Objects::Roll { position, radius, height, .. } => {
          let roll_center = Dot::from_array(position);
          let xyz = Gem::ray_x_roll(&ray_to_light, &roll_center, radius, height);
          if xyz.d_dot(&light_position) < obj_pixel_position.d_dot(&light_position) {pixel_color = pixel_color.dark_side(); break;}
        }
      }
    }
    
    // compare with nearest_position, and return the nearest one (with color)
    if pixel_position.d_dot(&ray.origin) < nearest_position.d_dot(&ray.origin) { (pixel_color, pixel_position) } else { (old_color, nearest_position) }

  }
  

}