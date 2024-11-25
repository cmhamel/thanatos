use thanatos::geometry::{Point, Sphere, Widths};
use thanatos::geometry::GeometricPrimitive;

mod test_sphere {
  use super::*;

  #[test]
  fn test_bounding_box() {
    let c = Sphere::new(1.);
    let bb = c.bounding_box();
    assert_eq!(bb.min, Point::new(-1., -1., -1.));
    assert_eq!(bb.max, Widths::new(1., 1., 1.));

    let c = Sphere::new(5.);
    let bb = c.bounding_box();
    assert_eq!(bb.min, Point::new(-5., -5., -5.));
    assert_eq!(bb.max, Widths::new(5., 5., 5.));
  }
  
  #[test]
  fn test_inside_sphere() {
    let c = Sphere::new(1.);
    let v = Point::new(0., 0., 0.);
    assert!(c.sdf(&v) < 0.);

    let c = Sphere::new(5.);
    let v = Point::new(0., 0., 0.);
    assert!(c.sdf(&v) < 0.);
  }

  #[test]
  fn test_on_boundary() {
    let c = Sphere::new(1.);
    let v = Point::new(1., 0., 0.);
    assert_eq!(c.sdf(&v), 0.);

    let c = Sphere::new(5.);
    println!("c = {:?}", c);
    let v = Point::new(5., 0., 0.);
    assert_eq!(c.sdf(&v), 0.);
  }

  #[test]
  fn test_outside_sphere() {
    let c = Sphere::new(1.);
    let v = Point::new(1., 1., 0.);
    assert!(c.sdf(&v) > 0.);

    let c = Sphere::new(5.);
    let v = Point::new(5., 5., 0.);
    assert!(c.sdf(&v) > 0.);
  }
}
