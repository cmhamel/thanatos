pub mod geometry;
pub mod mesh;

pub use geometry::*;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn thanatos(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_class::<Circle>()?;
  m.add_class::<Rectangle>()?;
  Ok(())
}

