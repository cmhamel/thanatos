use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Sphere {
  radius: f64
}

#[pymethods]
impl Sphere {
  #[new]
  pub fn new(radius: f64) -> Self {
    Sphere {
      radius: radius
    }
  }

  // pub fn from_json(v: &Value) -> Self {
  //   let s_temp = serde_json::to_string(v);
  //   let s: Self = serde_json::from_str(&s_temp).expect("wtf");
  //   s
  // }
}

impl GeometricPrimitive for Sphere {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox::new(
      &Point::new(-self.radius, -self.radius, -self.radius),
      &Point::new(self.radius, self.radius, self.radius)
    )
  }

  fn sdf(&self, v: &Point) -> f64 {
    v.coords.norm() - self.radius
  }
}
