use std::sync::Mutex;

use grapple_frc_msgs::grapple::lasercan::{LaserCanMeasurement, LaserCanRangingMode, LaserCanRoi, LaserCanRoiU4, LaserCanTimingBudget};
use grapplefrcdriver::lasercan::{LaserCanDevice, LASERCAN_STATUS_VALID_MEASUREMENT, LASERCAN_STATUS_NOISE_ISSUE, LASERCAN_STATUS_WEAK_SIGNAL, LASERCAN_STATUS_OUT_OF_BOUNDS, LASERCAN_STATUS_WRAPAROUND};
use pyo3::{prelude::*, exceptions::PyRuntimeError};

#[derive(Debug)]
pub struct ErrorWrapper(anyhow::Error);

impl std::error::Error for ErrorWrapper {}

impl std::fmt::Display for ErrorWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl From<anyhow::Error> for ErrorWrapper {
  fn from(value: anyhow::Error) -> Self {
    Self(value)
  }
}

impl From<ErrorWrapper> for PyErr {
  fn from(value: ErrorWrapper) -> Self {
    PyRuntimeError::new_err(value.0.to_string())
  }
}

pub type Result<T> = std::result::Result<T, ErrorWrapper>;

#[pyclass]
pub struct LaserCAN {
  device: Mutex<LaserCanDevice>
}

#[pymethods]
impl LaserCAN {
  #[new]
  pub fn new(can_id: u8) -> Self {
    Self { device: Mutex::new(LaserCanDevice::new(can_id)) }
  }

  pub fn get_measurement(&mut self) -> Option<LaserCanMeasurement> {
    self.device.lock().unwrap().get_measurement()
  }

  pub fn set_range(&mut self, mode: LaserCanRangingMode) -> Result<()> {
    self.device.lock().unwrap().set_range(mode).map_err(Into::into)
  }

  pub fn set_roi(&mut self, x: u8, y: u8, w: u8, h: u8) -> Result<()> {
    self.device.lock().unwrap().set_roi(LaserCanRoi { x: LaserCanRoiU4(x), y: LaserCanRoiU4(y), w: LaserCanRoiU4(w), h: LaserCanRoiU4(h) }).map_err(Into::into)
  }

  pub fn set_timing_budget(&mut self, budget: LaserCanTimingBudget) -> Result<()> {
    self.device.lock().unwrap().set_timing_budget(budget).map_err(Into::into)
  }
}

/// A Python module implemented in Rust.
#[pymodule]
fn grplpyfrc(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add("LASERCAN_STATUS_VALID_MEASUREMENT", LASERCAN_STATUS_VALID_MEASUREMENT)?;
  m.add("LASERCAN_STATUS_NOISE_ISSUE", LASERCAN_STATUS_NOISE_ISSUE)?;
  m.add("LASERCAN_STATUS_WEAK_SIGNAL", LASERCAN_STATUS_WEAK_SIGNAL)?;
  m.add("LASERCAN_STATUS_OUT_OF_BOUNDS", LASERCAN_STATUS_OUT_OF_BOUNDS)?;
  m.add("LASERCAN_STATUS_WRAPAROUND", LASERCAN_STATUS_WRAPAROUND)?;

  m.add_class::<LaserCanMeasurement>()?;
  m.add_class::<LaserCanRangingMode>()?;
  m.add_class::<LaserCanTimingBudget>()?;

  m.add_class::<LaserCAN>()?;

  Ok(())
}
