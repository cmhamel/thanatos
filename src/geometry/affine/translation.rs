use crate::geometry::base::*;
use nalgebra;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Translation<T: GeometricPrimitive> {
  translation: nalgebra::Translation3<f64>,
  primitive: T
}

impl<T> Translation<T> 
where T: GeometricPrimitive {
  pub fn new(primitive: T, x: f64, y: f64, z: f64) -> Self {
    Translation {
      translation: nalgebra::Translation3::new(x, y, z),
      primitive: primitive
    }
  }
}

impl<T> GeometricPrimitive for Translation<T> 
where T: GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox {
    self.primitive.bounding_box().transform(&self.translation.to_homogeneous())
  }

  fn sdf(&self, v: &Point) -> f64 {
    let v_temp = self.translation.inverse().transform_point(v);
    self.primitive.sdf(&v_temp)
  }
}

pub fn translate<T>(primitive: T, x: f64, y: f64, z: f64) -> Translation<T>
where T: GeometricPrimitive {
  Translation::<T>::new(primitive, x, y, z)
}
