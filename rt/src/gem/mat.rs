use super::utils::F64xyz;
use super::spear::Spear;
use super::dot::Dot;

/// 3d plane(flat surface) implementation
pub struct Mat {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64,
  pub origin: Dot,
  pub normal: Spear,
}

impl Mat {
  pub fn new(origin: Dot, normal: Spear) -> Mat {
    if normal.is_zero() {
      println!("Mat::new() normal is zero. Incorrect plane data.");
      Mat::zero()
    }
    else {
      let a = normal.x;
      let b = normal.y;
      let c = normal.z;
      let d = -a * origin.x - b * origin.y - c * origin.z;
      Mat { a, b, c, d, origin, normal }
    }

  }

  pub fn zero() -> Mat {
    Mat {
      a: 0.0,
      b: 0.0,
      c: 0.0,
      d: 0.0,
      origin: Dot::zero(),
      normal: Spear::zero(),
    }
  }

  pub fn is_zero(&self) -> bool { self.a == 0.0 && self.b == 0.0 && self.c == 0.0 && self.d == 0.0 }

  /// check the plane normal is back directed to the other plane normal
  pub fn is_back(&self, o: &Mat) -> bool { self.normal.is_back(&o.normal) }

  /// check the plane normal is same directed to the other plane normal
  pub fn is_same(&self, o: &Mat) -> bool { self.normal.is_same(&o.normal) }

  /// check the plane is parallel to the other plane
  pub fn is_ll(&self, o: &Mat) -> bool { self.normal.is_ll(&o.normal) }

}