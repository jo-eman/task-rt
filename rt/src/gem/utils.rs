use super::gem::Gem;

impl Gem {

  /** in the far past was detected some weird python3 calculatoins
    when sinus or cosinus was more than 1.0 or less than -1.0
    so this function was created to cut it to 1.0 or -1.0
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
  

}