use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Ellipsoid {
  a: f64,
  b: f64,
  c: f64
}

#[pymethods]
impl Ellipsoid {
  #[new]
  pub fn new(a: f64, b: f64, c: f64) -> Self {
    Ellipsoid {
      a: a,
      b: b,
      c: c
    }
  }
}

impl GeometricPrimitive for Ellipsoid {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox::new(
      &Point::new(-self.a, -self.b, -self.c),
      &Point::new(self.a, self.b, self.c)
    )
  }

  fn sdf(&self, v: &Point) -> f64 {
    let a_term = v.x / self.a;
    let b_term = v.y / self.b;
    let c_term = v.z / self.c;
    let a_term = a_term * a_term;
    let b_term = b_term * b_term;
    let c_term = c_term * c_term;
    let result = a_term + b_term + c_term;
    result.sqrt() - 1.
  }
}
