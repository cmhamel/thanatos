use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{self, Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Circle {
  radius: f64
}

impl Circle {
  pub fn new(radius: f64) -> Self {
    Circle {
      radius: radius
    }
  }
}

impl GeometricPrimitive for Circle {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox::new(
      &Point::new(-self.radius, -self.radius, 0.0),
      &Widths::new(self.radius, self.radius, 0.0)
    )
  }

  fn sdf(&self, v: &Point) -> f64 {
    // let temp = Point2::<f64>::new(v[0], v[1]);
    // v.coords.norm() - self.radius 
    let temp = v[0] * v[0] + v[1] * v[1];
    temp.sqrt() - self.radius
  }
}
