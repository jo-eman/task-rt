use super::mat::Mat;
use super::utils::F64xyz;
use super::spear::Spear;

/// 3d position implementation
pub struct Dot {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Dot {
  pub fn new(x: f64, y: f64, z: f64) -> Dot { Dot { x:x.xyz(), y:y.xyz(), z:z.xyz() } }
  
  pub fn zero() -> Dot { Dot::new(0.0, 0.0, 0.0) }

  pub fn is_zero(&self) -> bool { self.x == 0.0 && self.y == 0.0 && self.z == 0.0 }
  
  pub fn from_array(a: [f64; 3]) -> Dot {
    Dot::new(a[0], a[1], a[2])
  }
  pub fn to_array(&self) -> [f64; 3] { [self.x, self.y, self.z] }
  
  pub fn from_vec(v: Vec<f64>) -> Dot { Dot::new( v[0], v[1], v[2]) }
  pub fn to_vec(&self) -> Vec<f64> { vec![self.x, self.y, self.z] }

  pub fn from_spear(s: Spear) -> Dot { Dot::new(s.x, s.y, s.z) }
  pub fn to_spear(&self) -> Spear { Spear::new(self.x, self.y, self.z) }
  
  pub fn add(&self, o: &Dot) -> Dot {
    Dot::new( self.x + o.x, self.y + o.y, self.z + o.z )
  }
  
  pub fn sub(&self, o: &Dot) -> Dot {
    Dot::new( self.x - o.x, self.y - o.y, self.z - o.z )
  }
  
  pub fn mul(&self, o: &Dot) -> Dot {
    Dot::new( self.x * o.x, self.y * o.y, self.z * o.z )
  }
  
  //todo: not sure about this
  /// must return inf in case of division by zero , but additionally limited
  pub fn div(&self, o: &Dot) -> Dot {
    let x = if self.x == 0.0 && o.x == 0.0 { 1.0 } else {self.x};
    let y = if self.y == 0.0 && o.y == 0.0 { 1.0 } else {self.y};
    let z = if self.z == 0.0 && o.z == 0.0 { 1.0 } else {self.z};
    
    Dot::new(
      x / o.x,
      y / o.y,
      z / o.z,
    )

  }

  pub fn mirror_x(&self) -> Dot { Dot { x: -self.x, y: self.y, z: self.z, } }
  pub fn mirror_y(&self) -> Dot { Dot { x: self.x, y: -self.y, z: self.z, } }
  pub fn mirror_z(&self) -> Dot { Dot { x: self.x, y: self.y, z: -self.z, } }
  pub fn mirror_xy(&self) -> Dot { Dot { x: -self.x, y: -self.y, z: self.z, } }
  pub fn mirror_xz(&self) -> Dot { Dot { x: -self.x, y: self.y, z: -self.z, } }
  pub fn mirror_yz(&self) -> Dot { Dot { x: self.x, y: -self.y, z: -self.z, } }
  pub fn mirror(&self) -> Dot { Dot { x: -self.x, y: -self.y, z: -self.z, } }
  /*wtf am i doing , facepalm, need pause */

  /// offset position along vector
  pub fn offset(&self, s: &Spear, t: f64) -> Dot {
    let n = s.norm();
    if t == 0.0 || n == 0.0 {
      Dot::new(self.x, self.y, self.z) }
    else {
      Dot::new(
        self.x + s.x * t / n,
        self.y + s.y * t / n,
        self.z + s.z * t / n,
      )
    }

  }

  /// distance between positions
  pub fn d_dot(&self, o: &Dot) -> f64 {
    ((self.x - o.x).powi(2) + (self.y - o.y).powi(2) + (self.z - o.z).powi(2)).sqrt()
  }

  /// distance to plane (closest projection to flat surface).
  /// 
  /// If plane is zero, returns max_xyz limit, to avoid division by zero
  pub fn d_mat(&self, p: &Mat) -> f64 {
    let pn = p.normal.norm();
    if pn == 0.0 { f64::max_xyz() }
    else { (p.a * self.x + p.b * self.y + p.c * self.z + p.d).abs() / pn }
  }
  
}