use crate::geometry::base::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sphere {
  radius: f64
}

impl Sphere {
  pub fn new(radius: f64) -> Self {
    Sphere {
      radius: radius
    }
  }
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
