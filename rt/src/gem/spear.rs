use super::gem::Gem;

/**vector 3d implementation */
pub struct Spear {
  pub norm: f64,
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub alf: f64, //cosinus of angle between x axis
  pub bet: f64, //cosinus of angle between y axis
  pub gam: f64, //cosinus of angle between z axis
}

impl Spear {
  pub fn new(x: f64, y: f64, z: f64) -> Spear {
    /* length of vector */ 
    let norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    let alf = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(x / norm) };
    let bet = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(y / norm) };
    let gam = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(z / norm) };
    Spear { norm, x, y, z, alf, bet, gam }
    
  }
  
  /** zero vector */
  pub fn zero() -> Spear {
    Spear::new(0.0, 0.0, 0.0)
  }

  /** check the vector is zero */
  pub fn is_zero(&self) -> bool {
    self.norm == 0.0
  }
  
  /** unit vector with length 1 */
  pub fn unit(&self) -> Spear {
    if self.norm == 0.0 {Spear::zero()}
    else {
      Spear::new(
        self.x / self.norm,
        self.y / self.norm,
        self.z / self.norm
      )
    }
    
  }

  /** check the vector is unit with length 1 */
  pub fn is_unit(&self) -> bool {
    self.norm == 1.0
  }

  /** oposite vector */
  pub fn back(&self) -> Spear {
    Spear::new(-self.x, -self.y, -self.z)
  }

  /** check the vector is oposite directed */
  pub fn is_back(&self, other: &Spear) -> bool {
    let unit_other = other.unit();
    let unit_self = self.unit();
    unit_other.x == -unit_self.x &&
    unit_other.y == -unit_self.y &&
    unit_other.z == -unit_self.z
  }

  /** check the vector is same directed */
  pub fn is_same(&self, other: &Spear) -> bool {
    let unit_other = other.unit();
    let unit_self = self.unit();
    unit_other.x == unit_self.x &&
    unit_other.y == unit_self.y &&
    unit_other.z == unit_self.z
  }

  /** check the vector is parallel */
  pub fn is_ll(&self, other: &Spear) -> bool {
    self.is_same(other) || self.is_back(other)
  }

  /** check the vector is equal */
  pub fn is_eq(&self, other: &Spear) -> bool {
    self.x == other.x && self.y == other.y && self.z == other.z
  }
  
}