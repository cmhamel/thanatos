mod affine;
mod base;
mod booleans;
mod deserialize;
mod primitives;

use serde::{self, Deserialize, Serialize};

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

#[derive(Deserialize, Serialize)]
pub enum Primitives {
  // Circle(Circle),
  // Rectangle(Rectangle),
  // #[serde(rename = "Sphere")]
  Ellipsoid(Ellipsoid),
  Sphere(Sphere),
  Torus(Torus)
}

// impl GeometricPrimitive for Primitives {
//   fn bounding_box(&self) -> BoundingBox {

//   }
// }

pub enum AffineTransformations<T>
where T: GeometricPrimitive {
  Rotation(Rotation<T>),
  Translation(Translation<T>)
}

pub enum BooleanOperations<A, B>
where A: GeometricPrimitive,
      B: GeometricPrimitive {
  Difference(Difference<A, B>),
  Intersection(Intersection<A, B>),
  Union(Union<A, B>)
}

pub enum AllPrimitives<A, B> 
where A: GeometricPrimitive ,
      B: GeometricPrimitive {
  // affine
  Rotation(Rotation<A>),
  // boolean
  Union(Union<A, B>),
  // geometric primitives
  Torus(Torus)
}

//
pub use deserialize::deserialize_geometry;
