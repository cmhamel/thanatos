use bbox;
use enum_dispatch::enum_dispatch;
use nalgebra;

// some local types
pub type BoundingBox = bbox::BoundingBox<f64>;
pub type Point = nalgebra::Point3<f64>;
pub type Widths = nalgebra::Point3<f64>;

// base trait for all Primitive
#[enum_dispatch]
pub trait GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox;
  fn sdf(&self, v: &Point) -> f64;
}
