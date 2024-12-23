use thanatos::geometry::{Circle, Point, Widths};
use thanatos::geometry::GeometricPrimitive;

mod test_circle {
  use super::*;

  #[test]
  fn test_bounding_box() {
    let c = Circle::new(1.);
    let bb = c.bounding_box();
    assert_eq!(bb.min, Point::new(-1., -1., 0.));
    assert_eq!(bb.max, Widths::new(1., 1., 0.));

    let c = Circle::new(5.);
    let bb = c.bounding_box();
    assert_eq!(bb.min, Point::new(-5., -5., 0.));
    assert_eq!(bb.max, Widths::new(5., 5., 0.));
  }
  
  #[test]
  fn test_inside_circle() {
    let c = Circle::new(1.);
    let v = Point::new(0., 0., 0.);
    assert!(c.sdf(&v) < 0.);

    let c = Circle::new(5.);
    let v = Point::new(0., 0., 0.);
    assert!(c.sdf(&v) < 0.);
  }

  #[test]
  fn test_on_boundary() {
    let c = Circle::new(1.);
    let v = Point::new(1., 0., 0.);
    assert_eq!(c.sdf(&v), 0.);

    let c = Circle::new(5.);
    println!("c = {:?}", c);
    let v = Point::new(5., 0., 0.);
    assert_eq!(c.sdf(&v), 0.);
  }

  #[test]
  fn test_outside_circle() {
    let c = Circle::new(1.);
    let v = Point::new(1., 1., 0.);
    assert!(c.sdf(&v) > 0.);

    let c = Circle::new(5.);
    let v = Point::new(5., 5., 0.);
    assert!(c.sdf(&v) > 0.);
  }
}
