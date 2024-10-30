use serde::{Deserialize, Serialize};
use super::base::*;

// #[derive(Clone, Debug)]
// #[derive(Deserialize, Serialize)]
pub struct Difference<T> {
  left: T,
  right: Vec<Box<dyn Primitive>>
}

impl<T> Difference<T> {
  pub fn new(left: T, right: Vec<Box<dyn Primitive>>) -> Self {
    Difference{left: left, right: right}
  }
}

impl<T> BoundingBox for Difference<T> 
where 
  T: BoundingBox {
  fn bounding_box(&self) -> HyperRectangle {
    let left_bb = self.left.bounding_box();
    left_bb
  }
}

impl<T> Frep for Difference<T>
where 
  T: Frep {
  fn frep(&self, v: &Point) -> f64 {
    let left = self.left.frep(&v);
    let rights = self.right.iter().map(|x| x.frep(&v));
    let mut ret = left;
    for right in rights {
      ret = ret.max(-right);
    }
    ret
  }
}
