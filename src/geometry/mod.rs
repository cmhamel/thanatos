mod affine;
mod base;
mod booleans;
mod primitives;

use base::BoundingBox;
use enum_dispatch::enum_dispatch;
use pyo3::prelude::*;
use serde::{self, Deserialize, Serialize};
use std::fs;

pub use affine::{
  Rotation,
  Translation,
  rotate,
  translate
};
pub use base::GeometricPrimitive;
pub use base::{Point, Widths};
pub use booleans::{
  Difference,
  Intersection,
  Union
};
pub use primitives::three_dimensions;
pub use primitives::three_dimensions::{
  Ellipsoid,
  Gyroid,
  Sphere,
  Torus
};
pub use primitives::two_dimensions;
pub use primitives::two_dimensions::{
  Circle,
  Rectangle  
};

#[enum_dispatch(GeometricPrimitive)]
#[pyclass]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Primitive {
  // affine
  // Rotation(Box<Rotation>),
  Rotation(Rotation),
  Translation(Translation),
  // booleans
  Difference(Difference),
  Intersection(Intersection),
  Union(Union),
  // 2D Primitives
  Circle(Circle),
  Rectangle(Rectangle),
  // 3D Primitives
  Ellipsoid(Ellipsoid),
  Sphere(Sphere),
  Torus(Torus)
}

// stuff only useful on the python side
#[pymethods]
impl Primitive {
  #[staticmethod]
  pub fn from_json_file(file: &str) -> Self {
    let json_str: String = fs::read_to_string(file).expect("failed reading json file");
    let primitive: Primitive = serde_json::from_str(&json_str).expect("failed parsing json str");
    primitive
  }

  pub fn to_json(&self) -> String {
    let json_str = serde_json::to_string(self).expect("failed to convert to json");
    json_str
  }

  pub fn get_bbox(&self) -> ([f64; 3], [f64; 3]) {
    let bb = self.bounding_box();
    let min: [f64; 3] = bb.min.into();
    let max: [f64; 3] = bb.max.into();
    (min, max)
  }

  pub fn get_sdf(&self, x: f64, y: f64, z: f64) -> f64 {
    let v = Point::new(x, y, z);
    self.sdf(&v)
  }

  fn difference(&self, other: Primitive) -> Self {
    Primitive::Difference(Difference::new(self.clone(), other))
  }

  fn intersection(&self, other: Primitive) -> Self {
    Primitive::Intersection(Intersection::new(self.clone(), other))
  }

  fn rotate(&self, axis: char, angle: f64) -> Self {
    rotate(self.clone(), axis, angle)
  }

  fn translate(&self, x: f64, y: f64, z: f64) -> Self {
    translate(self.clone(), x, y, z)
  }

  fn union(&self, other: Primitive) -> Self {
    Primitive::Union(Union::new(self.clone(), other))
  }
}

// pub fn primitive_from_json(value: &Value) -> () {
  
// }

// impl GeometricPrimitive for Primitive {
//   fn bounding_box(&self) -> BoundingBox {
//     match self {
//       // affine
//       Primitive::Rotation(g)     => g.bounding_box(),
//       Primitive::Translation(g)  => g.bounding_box(),
//       // booleans
//       Primitive::Difference(g)   => g.bounding_box(),
//       Primitive::Intersection(g) => g.bounding_box(),
//       Primitive::Union(g)        => g.bounding_box(),
//       // 3D geometric Primitive
//       Primitive::Ellipsoid(g)    => g.bounding_box(),
//       Primitive::Sphere(g)       => g.bounding_box(),
//       Primitive::Torus(g)        => g.bounding_box()
//     }
//   } 

//   fn sdf(&self, v: &Point) -> f64 {
//     match self {
//       // affine
//       Primitive::Rotation(g)     => g.sdf(v),
//       Primitive::Translation(g)  => g.sdf(v),
//       // booleans
//       Primitive::Difference(g)   => g.sdf(v),
//       Primitive::Intersection(g) => g.sdf(v),
//       Primitive::Union(g)        => g.sdf(v),
//       // 3D geometric Primitive
//       Primitive::Ellipsoid(g)    => g.sdf(v),
//       Primitive::Sphere(g)       => g.sdf(v),
//       Primitive::Torus(g)        => g.sdf(v)
//     }
//   }
// }

//
// pub use deserialize::deserialize_geometry;
