use crate::geometry::Primitive;
use crate::geometry::base::*;
use min_max::max;
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Difference {
  left: Box<Primitive>,
  right: Box<Primitive>
}

impl Difference {
  pub fn new(left: Primitive, right: Primitive) -> Self {
    Difference {
      left: Box::new(left),
      right: Box::new(right)
    }
  }
}

impl GeometricPrimitive for Difference {
  fn bounding_box(&self) -> BoundingBox {
    self.left.bounding_box()
  }

  fn sdf(&self, v: &Point) -> f64 {
    max(self.left.sdf(v), -self.right.sdf(v))
  }
}
