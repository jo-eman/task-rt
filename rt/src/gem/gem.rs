use super::{dot::Dot, spear::Spear};

pub struct Gem {}

impl Gem {
  pub fn new() -> Gem {
    Gem {}
  }

  /** create 3d position */
  pub fn dot(&self, x:f64, y:f64, z:f64) -> Dot {
    Dot::new(x, y, z)
  }

  /** create 3d vector */
  pub fn spear(&self, x:f64, y:f64, z:f64) -> Spear {
    Spear::new(x, y, z)
  }
}