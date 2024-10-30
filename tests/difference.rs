use thanatos::geometry::{Circle, Difference, Point, Rectangle, translate};
use thanatos::geometry::{BoundingBox, Frep, Primitive};

mod test_difference {
  use super::*;

  #[test]
  fn test_bounding_box() {
    let r = Rectangle::new(1., 1.);
    let c = Circle::new(0.25);
    let t = translate(c, Point::new(0.5, 0.5, 0.0));
    let left = r;
    let mut right: Vec<Box<dyn Primitive>> = Vec::new();
    right.push(Box::new(t));
    let d = Difference::new(left, right);
    let bb = d.bounding_box();
    assert_eq!(bb.origin, Point::new(0., 0., 0.));
    assert_eq!(bb.widths, Point::new(1., 1., 0.));
  }

  #[test]
  fn test_inside_geometry() {
    let r = Rectangle::new(1., 1.);
    let c = Circle::new(0.25);
    let t = translate(c, Point::new(0.5, 0.5, 0.0));
    let left = r;
    let mut right: Vec<Box<dyn Primitive>> = Vec::new();
    right.push(Box::new(t));
    let d = Difference::new(left, right);
    let v = Point::new(0.25, 0.25, 0.);
    assert!(d.frep(&v) < 0.);
  }

  #[test]
  fn test_on_boundary() {
    let r = Rectangle::new(1., 1.);
    let c = Circle::new(0.25);
    let t = translate(c, Point::new(0.5, 0.5, 0.0));
    let left = r;
    let mut right: Vec<Box<dyn Primitive>> = Vec::new();
    right.push(Box::new(t));
    let d = Difference::new(left, right);
    let v = Point::new(0.5, 0.25, 0.);
    assert_eq!(d.frep(&v), 0.);
  }

  #[test]
  fn test_outside_geometry() {
    let r = Rectangle::new(1., 1.);
    let c = Circle::new(0.25);
    let t = translate(c, Point::new(0.5, 0.5, 0.0));
    let left = r;
    let mut right: Vec<Box<dyn Primitive>> = Vec::new();
    right.push(Box::new(t));
    let d = Difference::new(left, right);
    let v = Point::new(5.25, 0.25, 0.);
    assert!(d.frep(&v) > 0.);
  }
}