use nalgebra::{Point3, Vector3};
use serde::{Deserialize, Serialize};

// pub type Point = Vector3<f64>;
pub type Point = Point3<f64>;
pub type TriangleFace = Vector3<usize>;

#[derive(Clone, Debug)]
pub struct HyperRectangle {
  pub origin: Point,
  pub widths: Point
}

// TODO specialize to 2/3D
impl HyperRectangle {
  pub fn new(origin: Point, widths: Point) -> Self {
    HyperRectangle{origin: origin, widths: widths}
  }
}

// pub struct Circle;
pub trait BoundingBox {
  // returns a tuple
  // the first element is the minimum corner
  // the second element is the lenghts of the box
  // can be 1D, 2D, etc.
  fn bounding_box(&self) -> HyperRectangle;
}

pub trait Frep {
  fn frep(&self, v: &Point) -> f64;
}

pub trait Transform {
  fn forward(&self, v: &Point) -> Point;
  fn inverse(&self, v: &Point) -> Point;
  fn transform(&self) -> HyperRectangle;
}

// stuff for affine operations
#[derive(Clone, Debug)]
pub struct MapContainer<T, U> {
  map: T,
  primitive: U
}

impl<T, U> MapContainer<T, U> {
  pub fn new(map: T, primitive: U) -> Self {
    MapContainer{map: map, primitive: primitive}
  }
}

impl<T: Transform, U: BoundingBox> BoundingBox for MapContainer<T, U> {
  fn bounding_box(&self) -> HyperRectangle {
    self.transform()
  }
}

impl<T: Transform, U: Frep> Frep for MapContainer<T, U> {
  fn frep(&self, v: &Point) -> f64 {
    self.primitive.frep(&self.map.inverse(v))
  }
}

impl<T: Transform, U: BoundingBox> Transform for MapContainer<T, U> {
  fn forward(&self, v: &Point) -> Point {
    self.map.forward(v)
  }

  fn inverse(&self, v: &Point) -> Point {
    self.map.inverse(v)
  }

  fn transform(&self) -> HyperRectangle {
    let bb = self.primitive.bounding_box();
    let origin = self.forward(&bb.origin);
    HyperRectangle{origin: origin, widths: bb.widths}
  }
}

// trait that trys to capture all the behavior
pub trait Primitive: BoundingBox + Frep {}

impl<T> Primitive for T
where T: BoundingBox + Frep {
  // empty
}

pub fn save_csg_model<T>(primitive: &T) -> () 
where T: Serialize {
  let json = serde_json::to_string(primitive).unwrap();
  println!("Serialized to {:?}", json);
}

// helpers
// pub fn smooth_min(a: f64, b: f64, mut k: f64) -> f64 {
//   // polynomial smooth min

//   if k < 0.00001 {
//     k = 0.00001
//   }

//   let h = (k - (a - b).abs()).max(0.0) / k;

//   a.min(b) - h * h * k * (1.0 / 4.0)
// }
