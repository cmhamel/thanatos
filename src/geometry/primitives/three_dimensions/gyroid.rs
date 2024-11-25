use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Gyroid {
  box_length: f64,
  f: f64,
  t: f64
}

#[pymethods]
impl Gyroid {
  #[new]
  pub fn new(box_length: f64, f: f64, t: f64) -> Self {
    Gyroid {
      box_length: box_length,
      f: f,
      t: t
    }
  }
}

impl GeometricPrimitive for Gyroid {
  fn bounding_box(&self) -> BoundingBox {
    let p0 = Point::new(0., 0., 0.);
    let pf = Point::new(self.box_length, self.box_length, self.box_length);
    BoundingBox::new(&p0, &pf)
  }

  fn sdf(&self, v: &Point) -> f64 {
    let x = self.f * v.x;
    let y = self.f * v.y;
    let z = self.f * v.z;
    x.sin() * y.cos() + y.sin() * z.cos() + z.sin() * x.cos()
  }
}