use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Torus {
  a: f64,
  c: f64
}

#[pymethods]
impl Torus {
  #[new]
  pub fn new(a: f64, c: f64) -> Self {
    Torus {
      a: a,
      c: c
    }
  }
}

impl GeometricPrimitive for Torus {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox::new(
      &Point::new(-self.c - self.a, -self.c - self.a, -self.a),
      &Point::new(self.c + self.a, self.c + self.a, self.a)
    )
  }

  fn sdf(&self, v: &Point) -> f64 {
    let term_1 = v.x * v.x + v.y * v.y;
    let term_2 = self.c - term_1.sqrt();
    let result = term_2 * term_2 + v.z * v.z;
    let result = result.sqrt() - self.a * self.a;
    result
  }
}