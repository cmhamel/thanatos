use super::base::*;

#[derive(Clone, Debug)]
pub struct Translation {
  vector: Point
}

impl Translation {
  pub fn new(vector: Point) -> Self {
    Translation{vector: vector}
  }
}

// move these maybe?
pub fn add_points(p1: &Point, p2: &Point) -> Point {
  let temp = p1.coords + p2.coords;
  Point::new(temp[0], temp[1], temp[2])
}

pub fn subtract_points(p1: &Point, p2: Point) -> Point {
  (p1 - p2).into()
}

impl Transform for Translation {
  fn forward(&self, v: &Point) -> Point {
    add_points(v, &self.vector)
  }

  fn inverse(&self, v: &Point) -> Point {
    subtract_points(v, self.vector)
  }

  fn transform(&self) -> HyperRectangle {
    // dummy method
    HyperRectangle{
      origin: Point::new(0., 0., 0.),
      widths: Point::new(0., 0., 0.)
    }
  }
}

pub fn translate<T>(primitive: T, v: Point) -> MapContainer<Translation, T> 
where T: BoundingBox {
  let v = Translation::new(v);
  MapContainer::<Translation, T>::new(v, primitive)
}
