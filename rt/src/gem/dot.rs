
pub struct Dot {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Dot {
  pub fn new(x: f64, y: f64, z: f64) -> Dot { Dot { x, y, z } }
  
  pub fn zero() -> Dot { Dot { x: 0.0, y: 0.0, z: 0.0 } }
  
  pub fn from_array(array: [f64; 3]) -> Dot {
    Dot { x: array[0], y: array[1], z: array[2], }
  }
  
  pub fn to_array(&self) -> [f64; 3] { [self.x, self.y, self.z] }
  
  pub fn from_vec(vec: Vec<f64>) -> Dot { Dot { x: vec[0], y: vec[1], z: vec[2], } }
  
  pub fn to_vec(&self) -> Vec<f64> { vec![self.x, self.y, self.z] }
  
  pub fn add(&self, other: &Dot) -> Dot {
    Dot { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z, }
  }
  
  pub fn sub(&self, other: &Dot) -> Dot {
    Dot { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z, }
  }
  
  pub fn mul(&self, other: &Dot) -> Dot {
    Dot { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z, }
  }
  
  //todo: not sure about this
  /** must return inf in case of division by zero , but manually changed to f64::MAX */
  pub fn div(&self, other: &Dot) -> Dot {
    Dot {
      x:if other.x == 0.0 { f64::MAX } else { self.x / other.x },
      y:if other.y == 0.0 { f64::MAX } else { self.y / other.y },
      z:if other.z == 0.0 { f64::MAX } else { self.z / other.z },
    }
  }

  pub fn mirror_x(&self) -> Dot { Dot { x: -self.x, y: self.y, z: self.z, } }
  pub fn mirror_y(&self) -> Dot { Dot { x: self.x, y: -self.y, z: self.z, } }
  pub fn mirror_z(&self) -> Dot { Dot { x: self.x, y: self.y, z: -self.z, } }
  pub fn mirror_xy(&self) -> Dot { Dot { x: -self.x, y: -self.y, z: self.z, } }
  pub fn mirror_xz(&self) -> Dot { Dot { x: -self.x, y: self.y, z: -self.z, } }
  pub fn mirror_yz(&self) -> Dot { Dot { x: self.x, y: -self.y, z: -self.z, } }
  pub fn mirror(&self) -> Dot { Dot { x: -self.x, y: -self.y, z: -self.z, } }
  /*wtf am i doing , facepalm, need pause */
  
}