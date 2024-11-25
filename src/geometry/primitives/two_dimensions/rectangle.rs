use crate::geometry::base::*;
use min_max::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Rectangle {
  lower_corner: Point,
  length: f64,
  height: f64
}

// TODO also add a center option
#[pymethods]
impl Rectangle {
  #[new]
  pub fn new(length: f64, height: f64) -> Self {
    let lower_corner = Point::new(0., 0., 0.);
    Rectangle{
      lower_corner: lower_corner, 
      length: length, 
      height: height
    }
  }
}

impl GeometricPrimitive for Rectangle {
  fn bounding_box(&self) -> BoundingBox {
    BoundingBox::new(
      &self.lower_corner,
      &Widths::new(self.length, self.height, 0.0)
    )
  }

  fn sdf(&self, v: &Point) -> f64 {
    let x = v[0];
    let y = v[1];
    let dx = self.length;
    let dy = self.height;
    let lbx = self.lower_corner[0];
    let lby = self.lower_corner[1];
    max_partial!(-x + lbx, x - dx - lbx, -y + lby, y - dy - lby)
  }
}
