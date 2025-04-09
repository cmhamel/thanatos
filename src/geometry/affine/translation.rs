use crate::geometry::Primitive;
use crate::geometry::base::*;
use nalgebra;
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Translation {
  translation: nalgebra::Translation3<f64>,
  primitive: Box<Primitive>
}

impl Translation {
  pub fn new(primitive: Primitive, x: f64, y: f64, z: f64) -> Self {
    Translation {
      translation: nalgebra::Translation3::new(x, y, z),
      primitive: Box::new(primitive)
    }
  }
}

impl GeometricPrimitive for Translation {
  fn bounding_box(&self) -> BoundingBox {
    self.primitive.bounding_box().transform(&self.translation.to_homogeneous())
  }

  fn sdf(&self, v: &Point) -> f64 {
    let v_temp = self.translation.inverse().transform_point(v);
    self.primitive.sdf(&v_temp)
  }
}

pub fn translate(primitive: Primitive, x: f64, y: f64, z: f64) -> Primitive {
  Primitive::Translation(Translation::new(primitive, x, y, z))
}
