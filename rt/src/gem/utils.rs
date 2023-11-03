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
    let pmin = Dot::new(box_center.x - s, box_center.y - s, box_center.z - s,);
    let pmax = Dot::new(box_center.x + s, box_center.y + s, box_center.z + s,);

    let v = ray.normal;
    let o = ray.origin;
    let t1_min = (pmin.x - o.x) / v.x;
    let t1_max = (pmax.x - o.x) / v.x;

    let t2_min = (pmin.y - o.y) / v.y;
    let t2_max = (pmax.y - o.y) / v.y;

    let t3_min = (pmin.z - o.z) / v.z;
    let t3_max = (pmax.z - o.z) / v.z;

    let t_min = t1_min.max(t2_min).max(t3_min);
    let t_max = t1_max.min(t2_max).min(t3_max);

    if t_min > t_max || t_max < 0.0 {
      // println!("ray_x_box no intersections fires");
      return Dot::maximum()
    }

    let p1 = Dot::new(
      o.x + t_min * v.x,
      o.y + t_min * v.y,
      o.z + t_min * v.z,
    );

    let p2 = Dot::new(
      o.x + t_max * v.x,
      o.y + t_max * v.y,
      o.z + t_max * v.z,
    );

    println!("ray_x_box p1: {:#?}", p1);
    println!("ray_x_box p2: {:#?}", p2);

    if p1.d_dot(&o) < p2.d_dot(&o) {p1} else {p2}

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