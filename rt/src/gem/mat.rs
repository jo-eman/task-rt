use super::utils::F64xyz;
use super::spear::Spear;
use super::dot::Dot;

/// 3d plane(flat surface) implementation
#[derive(Debug, Clone, Copy)]
pub struct Mat {
  pub a: f64,
  pub b: f64,
  pub c: f64,
  pub d: f64,
  /// origin point of the plane normal
  pub origin: Dot,
  /// normal vector of the plane
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

  /// create plane from 2 dots
  pub fn pp(origin: Dot, normal_direction: Dot) -> Mat {
    let normal = Spear::pp( &[ origin, normal_direction, ] );

    Mat::new(origin, normal)

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
  
  pub fn same(&self) -> Mat {Mat::new(self.origin, self.normal)}

  /// check the plane normal is same directed to the other plane normal
  pub fn is_same(&self, o: &Mat) -> bool { self.normal.is_same(&o.normal) }
  
  pub fn is_zero(&self) -> bool { self.a == 0.0 && self.b == 0.0 && self.c == 0.0 }
  
  /// check the plane normal is back directed to the other plane normal
  pub fn is_back(&self, o: &Mat) -> bool { self.normal.is_back(&o.normal) }
  
  /// check the plane is parallel to the other plane
  pub fn is_ll(&self, o: &Mat) -> bool { self.normal.is_ll(&o.normal) }
  
  /// check the plane is below the other plane (along the other plane normal direction)
  pub fn is_below(&self, o: &Mat) -> bool {
    todo!("Mat::is_below()" );
    
  }
  
  /// check the plane is above the other plane (along the other plane normal direction)
  pub fn is_above(&self, o: &Mat) -> bool {
    todo!("Mat::is_above()");
    
  }
  
  /// check the plane is on the other plane(belongs to the same flat surface)
  pub fn is_eq(&self, o: &Mat) -> bool {
    self.a * o.d == o.a * self.d &&
    self.b * o.d == o.b * self.d &&
    self.c * o.d == o.c * self.d
  }

  /// distance between planes.
  /// Set zero if planes are not parallel. Because i want it.
  fn d_mat(&self, o: &Mat) -> f64 {
    if self.is_ll(o) {
        let n = (self.d - o.d).abs(); // numerator
        let d = (self.a.powi(2) + self.b.powi(2) + self.c.powi(2)).sqrt(); // denominator
        (n / d).xyz() // distance, additionally limited
    } else {
        0.0
    }
}
  
}