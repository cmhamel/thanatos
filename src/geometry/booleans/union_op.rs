use crate::geometry::base::*;
use min_max::min;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Union<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  left: A,
  right: B
}

impl<A, B> Union<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  pub fn new(left: A, right: B) -> Self {
    Union {
      left: left,
      right: right
    }
  }
}

impl<A, B> GeometricPrimitive for Union<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox {
    let bb = self.left.bounding_box().union(&self.right.bounding_box());
    bb
  }

  fn sdf(&self, v: &Point) -> f64 {
    min(self.left.sdf(v), self.right.sdf(v))
  }
}
