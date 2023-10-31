use super::dot::Dot;
use super::gem::Gem;
use super::utils::F64xyz;

/// vector 3d implementation
#[derive(Debug)]
pub struct Spear {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Spear {
  /// new vector. from zero to position. Will be recalculated to absolute length 1
  pub fn new(x: f64, y: f64, z: f64) -> Spear {
    let x = (x).xyz();
    let y = (y).xyz();
    let z = (z).xyz();
    /* length of vector */
    let norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    
    /* recalculate to unit vector of length 1 */
    let x = if norm == 0.0 { 0.0 } else { (x / norm).cut() };
    let y = if norm == 0.0 { 0.0 } else { (y / norm).cut() };
    let z = if norm == 0.0 { 0.0 } else { (z / norm).cut() };
    
    Spear { x, y, z,}
    
  }

  /// new vector. \[ from_position, to_position \]. Will be recalculated to absolute length 1
  pub fn pp(pp: &[Dot;2]) -> Spear {
    Spear::new(
      pp[1].x - pp[0].x,
      pp[1].y - pp[0].y,
      pp[1].z - pp[0].z,
    )
  }
  
  /// absolute length of vector
  pub fn norm(&self) -> f64 { (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt() }
  
  /// cosinus of the angle with the x axis
  pub fn alf(&self) -> f64 { if self.is_zero() { 1.0 } else { self.x / self.norm() } }
  
  /// cosinus of the angle with the y axis
  pub fn bet(&self) -> f64 { if self.is_zero() { 1.0 } else { self.y / self.norm() } }
  
  /// cosinus of the angle with the z axis
  pub fn gam(&self) -> f64 { if self.is_zero() { 1.0 } else { self.z / self.norm() } }
  
  /// same directed max length vector, limited additionally
  pub fn maximum() -> Spear {
    let mut v = Spear::zero();
    v.x = f64::max_xyz();
    v.y = f64::max_xyz();
    v.z = f64::max_xyz();
    v
  }
  
  /// check the vector is max
  pub fn is_maximum(&self) -> bool { self.norm() == Spear::maximum().norm() }
  
  /// zero vector
  pub fn zero() -> Spear { Spear::new(0.0, 0.0, 0.0) }
  /// check the vector is zero
  pub fn is_zero(&self) -> bool { self.norm() == 0.0 }

  /// vector (1.0, 1.0, 1.0)
  pub fn trione() -> Spear { Spear::new(1.0, 1.0, 1.0) }
  /// check the vector is trione
  pub fn is_trione(&self) -> bool { self.x == 1.0 && self.y == 1.0 && self.z == 1.0 }
  
  /// same directed scaled unit vector (with length 1) or zero vector(if x y z are zeros)
  pub fn unit(&self) -> Spear {
    if self.norm() == 0.0 {Spear::zero()}
    else { Spear::new( self.x, self.y, self.z ) }
    
  }
  
  /// check the vector is unit with length 1
  pub fn is_unit(&self) -> bool { self.norm() == 1.0 }
  
  /// oposite vector
  pub fn back(&self) -> Spear {
    let mut v = Spear::zero();
    v.x = -self.x;
    v.y = -self.y;
    v.z = -self.z;
    v
  }
  
  /// check the vector is oposite directed
  pub fn is_back(&self, other: &Spear) -> bool {
    let unit_other = other.unit();
    let unit_self = self.unit();
    unit_other.x == -unit_self.x &&
    unit_other.y == -unit_self.y &&
    unit_other.z == -unit_self.z
  }
  
  /// check the vector is same directed
  pub fn is_same(&self, other: &Spear) -> bool {
    let unit_other = other.unit();
    let unit_self = self.unit();
    unit_other.x == unit_self.x &&
    unit_other.y == unit_self.y &&
    unit_other.z == unit_self.z
  }
  
  /// check the vector is parallel
  pub fn is_ll(&self, other: &Spear) -> bool {
    self.is_same(other) || self.is_back(other)
  }
  
  //todo: not sure about this
  /// check the vector is equal
  pub fn is_eq(&self, other: &Spear) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
  
  /// scalar product of vectors, additionally limited to max_xyz
  pub fn scalar(&self, other: &Spear) -> f64 {
    (self.x * other.x + self.y * other.y + self.z * other.z).xyz()
  }
  
  /// vector product of vectors. Scaled to unit vector with length 1.
  /// 
  /// Vector perpendicular to self and other.
  /// Normal vector, directed to position, to see ccw from self to other.
  /// 
  /// Can return zero vector, in case of self and other are parallel,
  /// or one of them is zero vector.
  pub fn normal(&self, other: &Spear) -> Spear {
    Spear::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x
    )
  }
  
  /// sum of vectors, scaled to unit vector with length 1
  pub fn add(&self, s: &Spear) -> Spear {
    Spear::new( self.x + s.x, self.y + s.y, self.z + s.z, )
    
  }
  
  /// difference of vectors, scaled to unit vector with length 1
  pub fn sub(&self, s: &Spear) -> Spear {
    Spear::new( self.x - s.x, self.y - s.y, self.z - s.z, )
    
  }
  
  /// multiply vector by vector(limited additionally), scaled to unit vector with length 1
  pub fn mul(&self, s: &Spear) -> Spear {
    Spear::new( self.x * s.x, self.y * s.y, self.z * s.z, )
    
  }
  
  /// divide vector by vector(limited additionally), scaled to unit vector with length 1
  /// must return inf in case of ohter vector has zero components , but additionally limited
  pub fn div(&self, s: &Spear) -> Spear {
    let x = if self.x == 0.0 && s.x == 0.0 { 1.0 } else {self.x};
    let y = if self.y == 0.0 && s.y == 0.0 { 1.0 } else {self.y};
    let z = if self.z == 0.0 && s.z == 0.0 { 1.0 } else {self.z};
    
    Spear::new( x / s.x, y / s.y, z / s.z, )
    
  }
  
  /// cosine between vectors . in case of at least one vector is zero, returns 1.0
  pub fn cos(&self, other: &Spear) -> f64 {
    if self.is_zero() || other.is_zero() { return 1.0 }
    (self.scalar(other) / (self.norm() * other.norm())).cut()
  }
  
  /// angle between vectors in radians */
  pub fn ang_rad(&self, other: &Spear) -> f64 { self.cos(other).acos() }
  
  /// angle between vectors in degrees */
  pub fn ang_deg(&self, other: &Spear) -> f64 {
    Gem::degrees(self.ang_rad(other))
  }
  
}