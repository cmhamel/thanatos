use super::base::*;

pub struct Union<T> {
  left: T,
  right: Vec<Box<dyn Primitive>>
}

impl<T> Union<T> {
  pub fn new(left: T, right: Vec<Box<dyn Primitive>>) -> Self {
    Union{left: left, right: right}
  }
}

impl<T> BoundingBox for Union<T> 
where T: Primitive {
  fn bounding_box(&self) -> HyperRectangle {
    let bb_left = self.left.bounding_box();
    let bb_rights: Vec<_> = self.right
      .iter()
      .map(|x| x.bounding_box())
      .collect();

    let mut origin_x = bb_left.origin.x;
    let mut origin_y = bb_left.origin.y;
    let mut origin_z = bb_left.origin.z;

    let mut width_x = bb_left.widths.x;
    let mut width_y = bb_left.widths.y;
    let mut width_z = bb_left.widths.z;

    for bb in bb_rights {
      origin_x = origin_x.min(bb.origin.x);
      origin_y = origin_y.min(bb.origin.y);
      origin_z = origin_z.min(bb.origin.z);

      width_x = width_x.max(bb.widths.x);
      width_y = width_y.max(bb.widths.y);
      width_z = width_z.max(bb.widths.z);
    }

    let origin = Point::new(origin_x, origin_y, origin_z);
    let widths = Point::new(width_x, width_y, width_z);

    HyperRectangle{origin: origin, widths: widths}
  }
}

impl<T> Frep for Union<T>
where T: Primitive {
  fn frep(&self, v: &Point) -> f64 {
    let mut ret = self.left.frep(&v);
    for p in &self.right {
      ret = ret.min(p.frep(&v));
    }
    ret
  }
}
