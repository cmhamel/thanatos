use crate::geometry::base::*;
use nalgebra::{self, Vector3};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub struct Rotation<T: GeometricPrimitive> {
  rotation: nalgebra::Rotation3<f64>,
  primitive: T
}

impl<T> Rotation<T>
where T: GeometricPrimitive {
  pub fn new(primitive: T, axis: &nalgebra::Unit<Vector3<f64>>, angle: f64) -> Self {
    Rotation {
      rotation: nalgebra::Rotation3::from_axis_angle(axis, angle),
      primitive: primitive
    }
  }
}

impl<T> GeometricPrimitive for Rotation<T>
where T: GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox {
    self.primitive.bounding_box().transform(&self.rotation.to_homogeneous())
  }
  
  fn sdf(&self, v: &Point) -> f64 {
    let v_temp = self.rotation.inverse().transform_point(v);
    self.primitive.sdf(&v_temp)
  }
}

// pub fn rotate<T>(primitive: T, axis: &nalgebra::Unit<Vector3<f64>>, angle: f64) -> Rotation<T> 
// where T: GeometricPrimitive {
//   Rotation::<T>::new(primitive, axis, angle)
// }
pub fn rotate<T>(primitive: T, axis: char, angle: f64) -> Rotation<T> 
where T: GeometricPrimitive {
  let axis = match axis {
    'x' => Vector3::x_axis(),
    'y' => Vector3::y_axis(),
    'z' => Vector3::z_axis(),
    _   => panic!("Undefined axis")
  };
  Rotation::new(primitive, &axis, angle)
}
