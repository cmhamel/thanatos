use serde::{Deserialize, Serialize};
use super::base::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Circle {
  radius: f64
}

impl Circle {
  pub fn new(radius: f64) -> Self {
    Circle{radius: radius}
  }
}

impl BoundingBox for Circle {
  fn bounding_box(&self) -> HyperRectangle {
    HyperRectangle::new(
      Point::new(-self.radius, -self.radius, 0.),
      Point::new(2. * self.radius, 2. * self.radius, 0.)
    )
  }
}

impl Frep for Circle {
  fn frep(&self, v: &Point) -> f64 {
    // let temp = Point2::<f64>::new(v[0], v[1]);
    // v.coords.norm() - self.radius 
    let temp = v[0] * v[0] + v[1] * v[1];
    temp.sqrt() - self.radius
  }
}
