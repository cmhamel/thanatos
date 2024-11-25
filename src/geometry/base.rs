use bbox;
use nalgebra;

// some local types
pub type BoundingBox = bbox::BoundingBox<f64>;
pub type Point = nalgebra::Point3<f64>;
pub type Widths = nalgebra::Point3<f64>;

// base trait for all primitives
pub trait GeometricPrimitive {
  fn bounding_box(&self) -> BoundingBox;
  fn sdf(&self, v: &Point) -> f64;
}
