pub mod geometry;
pub mod mesh;

pub use geometry::Primitive;
use pyo3::wrap_pyfunction;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pyfunction]
fn ellipsoid(a: f64, b: f64, c: f64) -> Primitive {
  Primitive::Ellipsoid(geometry::Ellipsoid::new(a, b, c))
}

#[pyfunction]
fn sphere(radius: f64) -> Primitive {
  Primitive::Sphere(geometry::Sphere::new(radius))
}

#[pyfunction]
fn torus(a: f64, c: f64) -> Primitive {
  Primitive::Torus(geometry::Torus::new(a, c))
}

#[pyfunction]
fn stl_mesh(primitive: Primitive, x: usize, y: usize, z: usize, scale: f64, threshold: f64, path: &str) -> () {
  // mesh::mesh(primitive, x, y, z, scale, threshold);
  let m = mesh::mesh(primitive, x, y, z, scale, threshold);
  m.export_stl(path);
}

// #[pyfunction]
// fn voxel_mesh(primitive: Primitive, nelx: usize, nely: usize, nelz: usize, output: &str) -> () {
//   mesh::voxel_mesh(primitive, nelx, nely, nelz, output);
// }

#[pymodule]
fn thanatos(m: &Bound<'_, PyModule>) -> PyResult<()> {

  // Three dimensional Primitive
  m.add_function(wrap_pyfunction!(ellipsoid, m)?)?;
  m.add_function(wrap_pyfunction!(sphere, m)?)?;
  m.add_function(wrap_pyfunction!(torus, m)?)?;

  // enum type
  m.add_class::<Primitive>()?;

  // meshing
  m.add_function(wrap_pyfunction!(stl_mesh, m)?)?;
  // m.add_function(wrap_pyfunction!(voxel_mesh, m)?)?;

  Ok(())
}

