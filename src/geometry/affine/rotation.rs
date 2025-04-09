use crate::geometry::Primitive;
use crate::geometry::base::*;
use nalgebra::{self, Vector3};
use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rotation {
  rotation: nalgebra::Rotation3<f64>,
  primitive: Box<Primitive>
}

impl Rotation {
  pub fn new(primitive: Primitive, axis: &nalgebra::Unit<Vector3<f64>>, angle: f64) -> Self {
    Rotation {
      rotation: nalgebra::Rotation3::from_axis_angle(axis, angle),
      primitive: Box::new(primitive)
    }
  }
}

impl GeometricPrimitive for Rotation {
  fn bounding_box(&self) -> BoundingBox {
    self.primitive.bounding_box().transform(&self.rotation.to_homogeneous())
  }
  
  fn sdf(&self, v: &Point) -> f64 {
    let v_temp = self.rotation.inverse().transform_point(v);
    self.primitive.sdf(&v_temp)
  }
}

// TODO move to top level
pub fn rotate(primitive: Primitive, axis: char, angle: f64) -> Primitive {
  let axis = match axis {
    'x' => Vector3::x_axis(),
    'y' => Vector3::y_axis(),
    'z' => Vector3::z_axis(),
    _   => panic!("Undefined axis")
  };
  Primitive::Rotation(Rotation::new(primitive, &axis, angle))
}
