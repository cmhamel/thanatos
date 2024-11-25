use serde_json::{self, Result, Value};
use std::fs;
use thanatos::geometry::*;
use thanatos::mesh::*;

fn main() -> Result<()> {
  // setup a geometry
  let e1 = Torus::new(0.25, 2.5);
  let r1 = rotate(e1.clone(), 'x', 45.);
  let r2 = rotate(e1.clone(), 'y', 45.);
  let g2 = Union::new(e1, r1);
  let g3 = Union::new(g2, r2);

  // spherical annulus
  // let s1 = Sphere::new(2.);
  // let s2 = Sphere::new(1.);
  // let g3 = Difference::new(s2, s1);

  // serialization
  println!("Geometry = {:?}", g3);

  let s = serde_json::to_string(&g3)?;
  let _ = fs::write("example.json", &s);

  deserialize_geometry("example.json");
  println!("S = {:?}", s);
  let s: Value = serde_json::from_str(&s)?;
  // let e1 = &s["left"]["left"];
  let temp = serde_json::to_string(&s["left"]["left"])?;
  let e1: Primitives = serde_json::from_str(&temp)?;

  // println!("S = {:?}", s);
  // println!("S = {:?}", s["left"]["left"]);

  // meshing
  let n_samples = 64;
  let m = mesh(g3, n_samples, n_samples, n_samples, 1., 0.25);
  m.export_stl("example.stl");

  Ok(())
}
