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
    let x = Gem::limit_xyz(x);
    let y = Gem::limit_xyz(y);
    let z = Gem::limit_xyz(z);
    /* length of vector */
    let norm = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    let alf = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(x / norm) };
    let bet = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(y / norm) };
    let gam = if norm == 0.0 { 1.0 } else { Gem::sin_cos_cut(z / norm) };
    Spear { norm, x, y, z, alf, bet, gam }
    
  }

  /** max length vector, limited synthetically */
  pub fn maximum(&self) -> Spear { Spear::new(Gem::max_xyz(), Gem::max_xyz(), Gem::max_xyz()) }

  /** check the vector is max */
  pub fn is_maximum(&self) -> bool { self.norm == self.maximum().norm }
  
  /** zero vector */
  pub fn zero() -> Spear { Spear::new(0.0, 0.0, 0.0) }

  /** check the vector is zero */
  pub fn is_zero(&self) -> bool { self.norm == 0.0 }
  
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
  pub fn is_unit(&self) -> bool { self.norm == 1.0 }

  /** oposite vector */
  pub fn back(&self) -> Spear { Spear::new(-self.x, -self.y, -self.z) }

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

  /** scalar product of vectors */
  pub fn scalar(&self, other: &Spear) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  //todo: not sure about this
  /** vector product of vectors */
  pub fn normal(&self, other: &Spear) -> Spear {
    Spear::new(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x
    )
  }

  /** sum of vectors */
  pub fn add(&self, other: &Spear) -> Spear {
    let x =
    if self.x + other.x > Gem::max_xyz() { Gem::max_xyz() }
    else if self.x + other.x < Gem::min_xyz() { Gem::min_xyz() }
    else { self.x + other.x };

    let y =
    if self.y + other.y > Gem::max_xyz() { Gem::max_xyz() }
    else if self.y + other.y < Gem::min_xyz() { Gem::min_xyz() }
    else { self.y + other.y };

    let z =
    if self.z + other.z > Gem::max_xyz() { Gem::max_xyz() }
    else if self.z + other.z < Gem::min_xyz() { Gem::min_xyz() }
    else { self.z + other.z };

    Spear::new(x, y, z)
  }

  /** difference of vectors */
  pub fn sub(&self, other: &Spear) -> Spear {
    let x =
    if self.x - other.x > Gem::max_xyz() { Gem::max_xyz() }
    else if self.x - other.x < Gem::min_xyz() { Gem::min_xyz() }
    else { self.x - other.x };

    let y =
    if self.y - other.y > Gem::max_xyz() { Gem::max_xyz() }
    else if self.y - other.y < Gem::min_xyz() { Gem::min_xyz() }
    else { self.y - other.y };

    let z =
    if self.z - other.z > Gem::max_xyz() { Gem::max_xyz() }
    else if self.z - other.z < Gem::min_xyz() { Gem::min_xyz() }
    else { self.z - other.z };

    Spear::new(x, y, z)
  }

  /** multiply vector by scalar */
  pub fn mul(&self, scalar: f64) -> Spear {
    let x =
    if self.x * scalar > Gem::max_xyz() { Gem::max_xyz() }
    else if self.x * scalar < Gem::min_xyz() { Gem::min_xyz() }
    else { self.x * scalar };

    let y =
    if self.y * scalar > Gem::max_xyz() { Gem::max_xyz() }
    else if self.y * scalar < Gem::min_xyz() { Gem::min_xyz() }
    else { self.y * scalar };

    let z =
    if self.z * scalar > Gem::max_xyz() { Gem::max_xyz() }
    else if self.z * scalar < Gem::min_xyz() { Gem::min_xyz() }
    else { self.z * scalar };

    Spear::new(x, y, z)
  }

  /** divide vector by scalar */
  pub fn div(&self, scalar: f64) -> Spear {
    if scalar == 0.0 { self.maximum() }
    else { Spear::new(self.x / scalar, self.y / scalar, self.z / scalar) }
  }

  /** cosinus between . in case of at least one vector is zero, returns 1 */
  pub fn cos(&self, other: &Spear) -> f64 {
    if self.is_zero() || other.is_zero() { return 1.0 }
    Gem::sin_cos_cut(self.scalar(other) / (self.norm * other.norm))
  }

  /** angle between vectors in radians */
  pub fn angle_radians(&self, other: &Spear) -> f64 { self.cos(other).acos() }

  /** angle between vectors in degrees */
  pub fn angle_degrees(&self, other: &Spear) -> f64 {
    Gem::degrees(self.angle_radians(other))
  }
  
}