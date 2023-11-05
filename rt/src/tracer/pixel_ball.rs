use crate::{
  gem::{dot::Dot, spear::Spear, mat::Mat, gem::Gem, utils::F64xyz},
  parser::objects_file::Objects,
  tracer::{scene::Scene, pixel_::RGB}
};

impl Scene {

  /// check the hit point is on the dark side of the ball
  fn is_ball_dark_side(
    camera_ray_hit_xyz: Dot,
    light_position: Dot,
    center: Dot,
    radius: f64,
  ) -> bool {
    let light_ray = Mat::new(
      light_position,
      Spear::pp(
        &[
          light_position,
          camera_ray_hit_xyz,
        ]
      )
    );
    let light_xyz = Gem::ray_x_ball(&light_ray, &center, radius);
    
    light_xyz.d_dot(&light_position) < f64::Z9X9 * camera_ray_hit_xyz.d_dot(&light_position)

  }

  pub fn check_ball(
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
      Objects::Ball { color, position, radius } => {
        let center = Dot::from_array(position);
        let xyz = Gem::ray_x_ball(&ray, &center, radius);

        let mut rgb = RGB::power_affected(
          color,
          xyz,
          light_position,
          RGB::from_array(&self.light.color),
          self.light.power
        );

        if rgb.fresh
        && Scene::is_ball_dark_side( xyz, light_position, center, radius, )
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