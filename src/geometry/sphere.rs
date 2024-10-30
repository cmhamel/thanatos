use super::base::*;

#[derive(Clone, Debug)]
pub struct Sphere {
  radius: f64
}

impl Sphere {
  pub fn new(radius: f64) -> Self {
    Sphere{radius: radius}
  }
}

impl BoundingBox for Sphere {
  fn bounding_box(&self) -> HyperRectangle {
    HyperRectangle::new(
      Point::new(-self.radius, -self.radius, -self.radius),
      Point::new(2. * self.radius, 2. * self.radius, 2. * self.radius)
    )
  }
}

impl Frep for Sphere {
  fn frep(&self, v: &Point) -> f64 {
    v.coords.norm() - self.radius 
  }
}
