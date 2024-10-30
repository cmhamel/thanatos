use min_max::*;
use super::base::*;

pub struct LinearExtrusion<T> {
  primitive: T,
  distance: f64
}

impl<T> LinearExtrusion<T> {
  pub fn new(primitive: T, distance: f64) -> Self {
    LinearExtrusion{
      primitive: primitive,
      distance: distance
    }
  }
}

impl<T> BoundingBox for LinearExtrusion<T>
where T: Primitive {
  fn bounding_box(&self) -> HyperRectangle {
    let h = self.primitive.bounding_box();
    HyperRectangle {
      origin: Point::new(h.origin[0], h.origin[1], 0.0),
      widths: Point::new(h.widths[0], h.widths[1], self.distance)
    }
  }
}

impl<T> Frep for LinearExtrusion<T>
where T: Primitive {
  fn frep(&self, v: &Point) -> f64 {
    let r = self.primitive.frep(&v);
    let z = v[2];
    max_partial!(max_partial!(-z, z - self.distance), r)
  }
}
