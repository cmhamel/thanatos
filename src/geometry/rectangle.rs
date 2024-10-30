use super::base::*;
use min_max::*;

#[derive(Clone, Debug)]
pub struct Rectangle {
  lower_corner: Point,
  length: f64,
  height: f64
}

// TODO also add a center option
impl Rectangle {
  pub fn new(length: f64, height: f64) -> Self {
    let lower_corner = Point::new(0., 0., 0.);
    Rectangle{
      lower_corner: lower_corner, 
      length: length, 
      height: height
    }
  }
}

impl BoundingBox for Rectangle {
  fn bounding_box(&self) -> HyperRectangle {
    HyperRectangle::new(
      self.lower_corner,
      Point::new(self.length, self.height, 0.)
    )
  }
}

impl Frep for Rectangle {
  fn frep(&self, v: &Point) -> f64 {
    let x = v[0];
    let y = v[1];
    let dx = self.length;
    let dy = self.height;
    let lbx = self.lower_corner[0];
    let lby = self.lower_corner[1];
    max_partial!(-x + lbx, x - dx - lbx, -y + lby, y - dy - lby)
  }
}
