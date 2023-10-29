use super::gem::Gem;

impl Gem {

  /** in the far past was detected some weird python3 calculatoins
    when sinus or cosinus was more than 1.0 or less than -1.0
    it happened after calculations of sin and cos with no use std::f64::consts::PI,
    but with use division, multiplication and so on, of vector components.
    So this function was created to cut it to 1.0 or -1.0
  */
  pub fn sin_cos_cut(x: f64) -> f64 {
    match x {
      x if x > 1.0 => 1.0,
      x if x < -1.0 => -1.0,
      _ => x
    }
  }

  /** convert radians to degrees */
  pub fn degrees(angle_radians: f64) -> f64 {
    angle_radians * 180.0 / std::f64::consts::PI
  }

  /** convert degrees to radians */
  pub fn radians(angle_degrees: f64) -> f64 {
    angle_degrees * std::f64::consts::PI / 180.0
  }
  
  /** maximum value for 3d vector and 3d position projection to each axis */
  pub fn max_xyz() -> f64 { f64::MAX.sqrt()-1.0 }

  /** minimum value for 3d vector and 3d position projection to each axis */
  pub fn min_xyz() -> f64 { -Gem::max_xyz() }

  /** limit x, y, z value if incoming is greater than max_xyz or less than min_xyz */
  pub fn limit_xyz(value: f64) -> f64 {
    if value > Gem::max_xyz() { Gem::max_xyz() }
    else if value < Gem::min_xyz() { Gem::min_xyz() }
    else { value }
  }

}

trait F64xyz {
  /** maximum value for 3d vector and 3d position projection to each axis */
  fn max_xyz() -> f64 {f64::MAX.sqrt() / 3.0 -1.0}

  /** minimum value for 3d vector and 3d position projection to each axis */
  fn min_xyz() -> f64 {-Self::max_xyz()}

  /** additionally limit the coordinate for 3d space objects properties */
  fn xyz(self) -> f64;

  /** 
    For sin and cos safe calculations.
    
    keep the value inside range from -1.0(inclusive) to 1.0(inclusive).
    
    In the far past was detected some weird python3 calculatoins
    when sinus or cosinus was more than 1.0 or less than -1.0
    it happened after calculations of sin and cos with no use std::f64::consts::PI,
    but with use division, multiplication and so on, of vector components.
    So this function was created to cut it to 1.0 or -1.0
  */
  fn cut(self) -> f64;
}

impl F64xyz for f64 {
  fn xyz(self) -> f64 { self.min(f64::max_xyz()).max(f64::min_xyz()) }

  fn cut(self) -> f64 { self.max(-1.0).min(1.0) }

}