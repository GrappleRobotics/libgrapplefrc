use grapplefrcdriver::lasercan::{LaserCanMeasurement, LaserCanRangingMode, LaserCanRoi, LaserCanTimingBudget};

#[allow(dead_code)]
pub use pyo3::prelude::*;

#[allow(dead_code)]
pub use grapplefrcdriver::lasercan::LaserCAN;

#[allow(dead_code)]
pub use grapplefrcdriver::mitocandria::MitoCANdria;

#[pyfunction]
pub fn can_bridge_tcp() {
  grapplefrcdriver::can_bridge::start_can_bridge_c_background();
}

#[pymodule]
pub fn libgrapplefrc(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(can_bridge_tcp, m)?)?;
  m.add_class::<LaserCAN>()?;
  m.add_class::<LaserCanMeasurement>()?;
  m.add_class::<LaserCanRoi>()?;
  m.add_class::<LaserCanTimingBudget>()?;
  m.add_class::<LaserCanRangingMode>()?;

  m.add_class::<MitoCANdria>()?;

  Ok(())
}

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// /// A Python module implemented in Rust.
// #[pymodule]
// fn libgrapplefrc_py(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }
