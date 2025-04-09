use crate::geometry::Primitive;
use crate::geometry::base::*;
use min_max::min;
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Union {
  left: Box<Primitive>,
  right: Box<Primitive>
}

impl Union {
  pub fn new(left: Primitive, right: Primitive) -> Self {
    Union {
      left: Box::new(left),
      right: Box::new(right)
    }
  }
}

impl GeometricPrimitive for Union {
  fn bounding_box(&self) -> BoundingBox {
    let bb = self.left.bounding_box().union(&self.right.bounding_box());
    bb
  }

  fn sdf(&self, v: &Point) -> f64 {
    min(self.left.sdf(v), self.right.sdf(v))
  }
}
