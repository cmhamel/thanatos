use crate::geometry::base::*;
use min_max::max;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Difference<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  left: A,
  right: B
}

impl<A, B> Difference<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  pub fn new(left: A, right: B) -> Self {
    Difference {
      left: left,
      right: right
    }
  }     
}

impl<A, B> GeometricPrimitive for Difference<A, B>
where A: GeometricPrimitive,  
      B: GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox {
    self.left.bounding_box()
  }

  fn sdf(&self, v: &Point) -> f64 {
    max(self.left.sdf(v), -self.right.sdf(v))
  }
}
