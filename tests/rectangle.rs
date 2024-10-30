use thanatos::geometry::{Point, Rectangle};
use thanatos::geometry::{BoundingBox, Frep};
// use thanatos::geometry::Geometries;

mod test_rectangle {
  use super::*;

  #[test]
  fn test_bounding_box() {
    let r = Rectangle::new(1., 1.);
    let bb = r.bounding_box();
    assert_eq!(bb.origin, Point::new(0., 0., 0.));
    assert_eq!(bb.widths, Point::new(1., 1., 0.));

    let r = Rectangle::new(3., 2.);
    let bb = r.bounding_box();
    assert_eq!(bb.origin, Point::new(0., 0., 0.));
    assert_eq!(bb.widths, Point::new(3., 2., 0.));
  }

  #[test]
  fn test_inside_rectangle() {
    let r = Rectangle::new(1., 1.);
    let v = Point::new(0.5, 0.5, 0.);
    assert!(r.frep(&v) < 0.);

    let r = Rectangle::new(2., 3.);
    let v = Point::new(0.5, 0.5, 0.);
    assert!(r.frep(&v) < 0.);
  }

  #[test]
  fn test_on_boundary() {
    let r = Rectangle::new(1., 1.);
    let v = Point::new(1., 0.5, 0.);
    assert_eq!(r.frep(&v), 0.);

    let r = Rectangle::new(2., 3.);
    let v = Point::new(2., 0.5, 0.);
    assert_eq!(r.frep(&v), 0.);
  }

  #[test]
  fn test_outside_rectangle() {
    let r = Rectangle::new(1., 1.);
    let v = Point::new(1.5, 0.5, 0.);
    assert!(r.frep(&v) > 0.);

    let r = Rectangle::new(2., 3.);
    let v = Point::new(2.5, 0.5, 0.);
    assert!(r.frep(&v) > 0.);
  }
}
