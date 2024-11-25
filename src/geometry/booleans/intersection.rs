use crate::geometry::base::*;
use min_max::max;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Intersection<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  left: A,
  right: B
}

impl<A, B> Intersection<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  pub fn new(left: A, right: B) -> Self {
    Intersection {
      left: left,
      right: right
    }
  }
}

impl<A, B> GeometricPrimitive for Intersection<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox {
    self.left.bounding_box().intersection(&self.right.bounding_box())
  }

  fn sdf(&self, v: &Point) -> f64 {
    max(self.left.sdf(v), self.right.sdf(v))
  }
}
