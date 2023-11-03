use crate::debug::append_to_file;

use super::{gem::Gem, spear::Spear, mat::Mat, dot::Dot};

impl Gem {

  /// intersection of ray and plane
  pub fn ray_x_mat(ray: &Mat, mat: &Mat) -> Dot {
    let tup = -(
      mat.normal.x * ray.origin.x +
      mat.normal.y * ray.origin.y +
      mat.normal.z * ray.origin.z +
      mat.d
    );
    let tdn = (
      mat.normal.x * ray.normal.x +
      mat.normal.y * ray.normal.y +
      mat.normal.z * ray.normal.z
    );

    if tdn == 0.0 {return Dot::maximum()}
    if tup == 0.0 {return ray.origin}

    let t = tup / tdn;

    Dot::new(
      ray.origin.x + ray.normal.x * t,
      ray.origin.y + ray.normal.y * t,
      ray.origin.z + ray.normal.z * t,
    )

  }

  /// intersection of ray and sphere.
  /// 
  /// The first one from ray origin, to forward direction, along the ray vector.
  /// 
  /// If there is no intersection than return Dot::maximum(), to avoid brainfuck
  pub fn ray_x_ball(ray: &Mat, center:&Dot, radius:f64) -> Dot {
    // vector from the ball's center to the ray's origin. Just for case without unit autoconvertion, so Spear is not used. Just components separately
    let x = ray.origin.x - center.x;
    let y = ray.origin.y - center.y;
    let z = ray.origin.z - center.z;

    // Coefficients for the quadratic equation
    let a = ray.normal.x.powi(2) + ray.normal.y.powi(2) + ray.normal.z.powi(2);
    let b = 2f64 * (x * ray.normal.x + y * ray.normal.y + z * ray.normal.z);
    let c = x * x + y * y + z * z - radius.powi(2);

    // Calculate the discriminant
    let d = b * b - 4.0 * a * c;

    // If the discriminant is negative, there are no intersections
    if d < 0.0 { return Dot::maximum() }

    // Calculate the two possible solutions for t
    let t1 = (-b - d.sqrt()) / (2.0 * a);
    let t2 = (-b + d.sqrt()) / (2.0 * a);

    // Choose the closer to ray origin (which is zoom point) intersection point
    let p1 = Dot::new(
      ray.origin.x + t1 * ray.normal.x,
      ray.origin.y + t1 * ray.normal.y,
      ray.origin.z + t1 * ray.normal.z,
    );

    let p2 = Dot::new(
      ray.origin.x + t2 * ray.normal.x,
      ray.origin.y + t2 * ray.normal.y,
      ray.origin.z + t2 * ray.normal.z,
    );

    if p1.d_dot(&ray.origin) < p2.d_dot(&ray.origin) {p1} else {p2}
    
  }

  /// intersection of ray and cube.
  /// 
  /// The cube(box) is oriented along axes, it does not break any requirements of the task.
  /// The target is ray tracing, not cube rotating.
  /// 
  /// the size is the length of the edge of the cube. So it is width, height and depth, in one value.
  pub fn ray_x_box(ray: &Mat, box_center:&Dot, box_size:f64) -> Dot {
    let s = box_size.abs().half(); // just for case of an idiot. Lazy to check the difference

    // create two positions, which will restrict the values for x, y and z
    // from minimum to maximum
    let pmin = Dot::new(box_center.x - s, box_center.y - s, box_center.z - s,);
    let pmax = Dot::new(box_center.x + s, box_center.y + s, box_center.z + s,);

    // create x6 planes, to simulate the cube surface. Each plane is a Mat
    let px_min = Mat::new( pmin, Spear::ox().back());
    let px_max = Mat::new( pmax, Spear::ox());
    let py_min = Mat::new( pmin, Spear::oy().back());
    let py_max = Mat::new( pmax, Spear::oy());
    let pz_min = Mat::new( pmin, Spear::oz().back());
    let pz_max = Mat::new( pmax, Spear::oz());

    // check the ray intersection with each plane
    let hit_x_min = Gem::ray_x_mat(ray, &px_min);
    let hit_x_max = Gem::ray_x_mat(ray, &px_max);
    let hit_y_min = Gem::ray_x_mat(ray, &py_min);
    let hit_y_max = Gem::ray_x_mat(ray, &py_max);
    let hit_z_min = Gem::ray_x_mat(ray, &pz_min);
    let hit_z_max = Gem::ray_x_mat(ray, &pz_max);

    // check the intersection points are on the cube surface,
    // and return the nearest one(the valid area of each plane
    // restricted by x4 planes around, but in our case of placement the cube
    // along axes, enough just restrict the intersecion by min max coordinates)
    let mut hit = Dot::maximum();

    if hit_x_min.y >= pmin.y && hit_x_min.y <= pmax.y
    && hit_x_min.z >= pmin.z && hit_x_min.z <= pmax.z
    && hit.d_dot(&ray.origin) > hit_x_min.d_dot(&ray.origin)
    {hit = hit_x_min.same()}

    if hit_x_max.y >= pmin.y && hit_x_max.y <= pmax.y
    && hit_x_max.z >= pmin.z && hit_x_max.z <= pmax.z
    && hit.d_dot(&ray.origin) > hit_x_max.d_dot(&ray.origin)
    {hit = hit_x_max.same()}

    if hit_y_min.x >= pmin.x && hit_y_min.x <= pmax.x
    && hit_y_min.z >= pmin.z && hit_y_min.z <= pmax.z
    && hit.d_dot(&ray.origin) > hit_y_min.d_dot(&ray.origin)
    {hit = hit_y_min.same()}

    if hit_y_max.x >= pmin.x && hit_y_max.x <= pmax.x
    && hit_y_max.z >= pmin.z && hit_y_max.z <= pmax.z
    && hit.d_dot(&ray.origin) > hit_y_max.d_dot(&ray.origin)
    {hit = hit_y_max.same()}

    if hit_z_min.x >= pmin.x && hit_z_min.x <= pmax.x
    && hit_z_min.y >= pmin.y && hit_z_min.y <= pmax.y
    && hit.d_dot(&ray.origin) > hit_z_min.d_dot(&ray.origin)
    {hit = hit_z_min.same()}

    if hit_z_max.x >= pmin.x && hit_z_max.x <= pmax.x
    && hit_z_max.y >= pmin.y && hit_z_max.y <= pmax.y
    && hit.d_dot(&ray.origin) > hit_z_max.d_dot(&ray.origin)
    {hit = hit_z_max.same()}

    hit

  }

  /// convert radians to degrees
  pub fn degrees(angle_radians: f64) -> f64 {
    angle_radians * 180.0 / std::f64::consts::PI
  }

  /// convert degrees to radians
  pub fn radians(angle_degrees: f64) -> f64 {
    angle_degrees * std::f64::consts::PI / 180.0
  }
  
}

pub trait F64xyz {
  /// maximum value for 3d vector and 3d position projection to each axis
  fn max_xyz() -> f64 {(f64::MAX / 6.0).sqrt() -1.0}
  // (../6.0.. ) to try prevent overflow for 3d plane calculations

  /// minimum value for 3d vector and 3d position projection to each axis
  fn min_xyz() -> f64 {-Self::max_xyz()}

  /// additionally limit the coordinate for 3d space objects properties
  fn xyz(self) -> f64;

  /// For sin and cos safe calculations.
  /// 
  /// keep the value inside range from -1.0(inclusive) to 1.0(inclusive).
  /// 
  /// In the far past was detected some weird python3 calculatoins
  /// when sin or cos was more than 1.0 or less than -1.0
  /// it happened after calculations with no use std::f64::consts::PI,
  /// but with use division, multiplication and so on, of vector components.
  /// So this function was created keep value inside range
  /// from -1.0 to 1.0
  fn cut(self) -> f64;

  /// half of the value(multiply by 0.5)
  fn half(self) -> f64;
}

impl F64xyz for f64 {
  fn xyz(self) -> f64 { self.min(f64::max_xyz()).max(f64::min_xyz()) }

  fn cut(self) -> f64 { self.max(-1.0).min(1.0) }

  fn half(self) -> f64 { self * 0.5 }

}