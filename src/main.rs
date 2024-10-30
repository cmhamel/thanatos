mod geometry;
mod mesh;

pub use geometry::*;
pub use mesh::*;

fn main() {
  println!("Euclid!");

  // let c1 = Circle::new(1.);
  let c1 = Rectangle::new(1., 2.);
  let e1 = LinearExtrusion::new(c1, 100.);
  let mesh = mesh(e1, 128, 128, 128, 1., 0.1);

  // let s1 = Sphere::new(1.5);
  // let t1 = translate(s1, Point::new(1., 0., 0.));
  // let s2 = Sphere::new(1.);
  // let t2 = translate(s2, Point::new(0., 1., 0.));
  // // let s3 = Sphere::new(1.);
  // let s3 = Brick::new(1., 2., 3.);
  // let u = Difference::new(s3, vec!(Box::new(t1), Box::new(t2)));
  // let me = mesh(u, 100, 100, 100, 1., 0.2);

  // let me = mesh(s3, 100, 100, 100, 1., 0.1);

  // let mut binary_stl = Vec::<u8>::new();
  mesh.export_stl("mesh.stl");

  // geometry::save_csg_model(&c1);
}
