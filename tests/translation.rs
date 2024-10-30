use thanatos::geometry::{Circle, Point, translate};
use thanatos::geometry::{BoundingBox, Frep};
use nalgebra::Point3;

mod test_translation {
  use super::*;

  #[test]
  fn test_bounding_box_no_translation() {
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 0., 0.);
    let t = translate(c, v);
    let bb = t.bounding_box();
    assert_eq!(bb.origin, Point::new(-1., -1., 0.));
    assert_eq!(bb.widths, Point::new(2., 2., 0.));
  }

  #[test]
  fn test_bounding_box_x_translation() {
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(1., 0., 0.);
    let t = translate(c, v);
    let bb = t.bounding_box();
    assert_eq!(bb.origin, Point::new(0., -1., 0.));
    assert_eq!(bb.widths, Point::new(2., 2., 0.));
  }

  #[test]
  fn test_bounding_box_y_translation() {
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 1., 0.);
    let t = translate(c, v);
    let bb = t.bounding_box();
    assert_eq!(bb.origin, Point::new(-1., 0., 0.));
    assert_eq!(bb.widths, Point::new(2., 2., 0.));
  }

  #[test]
  fn test_bounding_box_z_translation() {
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 0., 1.);
    let t = translate(c, v);
    let bb = t.bounding_box();
    assert_eq!(bb.origin, Point::new(-1., -1., 1.));
    assert_eq!(bb.widths, Point::new(2., 2., 0.));
  }

  #[test]
  fn test_inside_translated_circle() {
    // x translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(1., 0., 0.);
    let t = translate(c, v);
    let v = Point::new(1., 0., 0.);
    assert!(t.frep(&v) < 0.);

    // y translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 1., 0.);
    let t = translate(c, v);
    let v = Point::new(0., 1., 0.);
    assert!(t.frep(&v) < 0.);

    // z translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 0., 1.);
    let t = translate(c, v);
    let v = Point::new(0., 0., 1.);
    assert!(t.frep(&v) < 0.);
  }

  #[test]
  fn test_on_translated_boundary() {
    // x translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(1., 0., 0.);
    let t = translate(c, v);
    let v = Point::new(2., 0., 0.);
    assert_eq!(t.frep(&v), 0.);

    // y translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 1., 0.);
    let t = translate(c, v);
    let v = Point::new(0., 2., 0.);
    assert_eq!(t.frep(&v), 0.);

    // // z translation
    // let c = Circle::new(1.);
    // let v = Point3::<f64>::new(0., 0., 1.);
    // let t = translate(c, v);
    // let v = Point::new(0., 0., 2.);
    // assert_eq!(t.frep(&v), 0.);
  }

  #[test]
  fn test_outside_translated_circle() {
    // x translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(1., 0., 0.);
    let t = translate(c, v);
    let v = Point::new(2., 1., 0.);
    assert!(t.frep(&v) > 0.);

    // y translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 1., 0.);
    let t = translate(c, v);
    let v = Point::new(1., 2., 0.);
    assert!(t.frep(&v) > 0.);

    // z translation
    let c = Circle::new(1.);
    let v = Point3::<f64>::new(0., 0., 1.);
    let t = translate(c, v);
    let v = Point::new(1., 1., 1.);
    assert!(t.frep(&v) > 0.);
  }

  #[test]
  fn test_other() {
    let c = Circle::new(5.);
    let v = Point3::<f64>::new(1., 0., 0.);
    let t = translate(c, v);
    println!("bb = {:?}", t.bounding_box());
  }
}