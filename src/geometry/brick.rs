use min_max::*;
use serde::{Deserialize, Serialize};
use super::base::*;

#[derive(Clone, Debug)]
pub struct Brick {
  lower_corner: Point,
  widths: Point
}

impl Brick {
  pub fn new(length_x: f64, length_y: f64, length_z: f64) -> Self {
    Brick{
      lower_corner: Point::new(0., 0., 0.),
      widths: Point::new(length_x, length_y, length_z)
    }
  }
}

impl BoundingBox for Brick {
  fn bounding_box(&self) -> HyperRectangle {
    HyperRectangle{
      origin: self.lower_corner, 
      widths: self.widths
    }
  }
}

impl Frep for Brick {
  fn frep(&self, v: &Point) -> f64 {
    let (x, y, z) = (v[0], v[1], v[2]);
    let (dx, dy, dz) = (self.widths[0], self.widths[1], self.widths[2]);
    let (lbx, lby, lbz) = (self.lower_corner[0], self.lower_corner[1], self.lower_corner[2]);
    max_partial!(
      -x + lbx, x - dx - lbx,
      -y + lby, y - dy - lby,
      -z + lbz, z - dz - lbz
    )
  }
}
