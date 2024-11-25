use thanatos::geometry::{Circle, Point, translate};
use thanatos::geometry::GeometricPrimitive;

mod test_translation {
  use super::*;

  #[test]
  fn test_bounding_box_no_translation_2d() {
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 0.);
    let bb = t.bounding_box();
    assert_eq!(bb.min, Point::new(-1., -1., 0.));
    assert_eq!(bb.max, Point::new(1., 1., 0.));
  }

  #[test]
  fn test_bounding_box_x_translation_2d() {
    let c = Circle::new(1.);
    let t = translate(c, 1., 0., 0.);
    let bb = t.bounding_box();
    assert_eq!(bb.min, Point::new(0., -1., 0.));
    assert_eq!(bb.max, Point::new(2., 1., 0.));
  }

  #[test]
  fn test_bounding_box_y_translation_2d() {
    let c = Circle::new(1.);
    let t = translate(c, 0., 1., 0.);
    let bb = t.bounding_box();
    assert_eq!(bb.min, Point::new(-1., 0., 0.));
    assert_eq!(bb.max, Point::new(1., 2., 0.));
  }

  #[test]
  fn test_bounding_box_z_translation_2d() {
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 1.);
    let bb = t.bounding_box();
    assert_eq!(bb.min, Point::new(-1., -1., 1.));
    assert_eq!(bb.max, Point::new(1., 1., 1.));
  }

  #[test]
  fn test_inside_translated_circle() {
    // no translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 0.);
    let v = Point::new(0., 0., 0.);
    assert!(t.sdf(&v) < 0.);

    // x translation
    let c = Circle::new(1.);
    let t = translate(c, 1., 0., 0.);
    let v = Point::new(1., 0., 0.);
    assert!(t.sdf(&v) < 0.);

    // y translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 1., 0.);
    let v = Point::new(0., 1., 0.);
    assert!(t.sdf(&v) < 0.);

    // z translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 1.);
    let v = Point::new(0., 0., 1.);
    assert!(t.sdf(&v) < 0.);
  }

  #[test]
  fn test_on_translated_boundary() {
    // no translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 0.);
    let v = Point::new(1., 0., 0.);
    assert_eq!(t.sdf(&v), 0.);

    // x translation
    let c = Circle::new(1.);
    let t = translate(c, 1., 0., 0.);
    let v = Point::new(2., 0., 0.);
    assert_eq!(t.sdf(&v), 0.);

    // y translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 1., 0.);
    let v = Point::new(0., 2., 0.);
    assert_eq!(t.sdf(&v), 0.);

    // z translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 1.);
    let v = Point::new(1., 0., 1.);
    assert_eq!(t.sdf(&v), 0.);
  }

  #[test]
  fn test_outside_translated_circle() {
    // no translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 0.);
    let v = Point::new(2., 0., 0.);
    assert!(t.sdf(&v) > 0.);

    // x translation
    let c = Circle::new(1.);
    let t = translate(c, 1., 0., 0.);
    let v = Point::new(2., 1., 0.);
    assert!(t.sdf(&v) > 0.);

    // y translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 1., 0.);
    let v = Point::new(1., 2., 0.);
    assert!(t.sdf(&v) > 0.);

    // z translation
    let c = Circle::new(1.);
    let t = translate(c, 0., 0., 1.);
    let v = Point::new(1., 1., 1.);
    assert!(t.sdf(&v) > 0.);
  }

  #[test]
  fn test_other() {
    let c = Circle::new(5.);
    let t = translate(c, 1., 0., 0.);
    println!("bb = {:?}", t.bounding_box());
  }
}