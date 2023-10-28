use super::{dot::Dot, spear::Spear};

pub struct Gem {}

impl Gem {
  pub fn new() -> Gem {
    Gem {}
  }

  pub fn dot(&self, x:f64, y:f64, z:f64) -> Dot {
    Dot::new(x, y, z)
  }

  pub fn spear(&self, x:f64, y:f64, z:f64) -> Spear {
    Spear::new(x, y, z)
  }
}