// use std::num::sqrt;
use approx::assert_relative_eq;
use std::f64::consts::PI;
use thanatos::geometry::{Point, Rectangle, rotate};
use thanatos::geometry::GeometricPrimitive;

mod test_rotation {
  use super::*;

  #[test]
  fn test_bounding_box_no_rotation_2d() {
    let r = Rectangle::new(1., 1.);
    let r = rotate(r, 'z', 0.0);
    let bb = r.bounding_box();
    assert_eq!(bb.min, Point::new(0., 0., 0.));
    assert_eq!(bb.max, Point::new(1., 1., 0.));
  }

  #[test]
  fn test_bounding_box_rotation_z_axis_2d() {
    let r = Rectangle::new(1., 1.);
    let r = rotate(r, 'z', PI / 4.);
    let bb = r.bounding_box();
    let temp: f64 = 2.;
    assert_relative_eq!(bb.min, Point::new(-temp.sqrt() / 2., 0., 0.));
    assert_relative_eq!(bb.max, Point::new(temp.sqrt() / 2., temp.sqrt(), 0.));
  }

  #[test]
  fn test_inside_box_rotation_z_axis_2d() {
    let r = Rectangle::new(1., 1.);
    let r = rotate(r, 'z', PI / 4.);
    let temp: f64 = 2.;
    let v = Point::new(0., temp.sqrt() / 2., 0.);
    assert!(r.sdf(&v) < 0.);
  }

  #[test]
  fn test_outside_box_rotation_z_axis_2d() {
    let r = Rectangle::new(1., 1.);
    let r = rotate(r, 'z', PI / 4.);
    let v = Point::new(0., 4., 0.);
    assert!(r.sdf(&v) > 0.);
  }
}
