use std::time::{Duration, Instant};

use bounded_static::ToBoundedStatic as _;
pub use grapple_frc_msgs::{grapple::{Request, errors::{GrappleResult, GrappleError}, lasercan::{LaserCanMessage, LaserCanRoi, LaserCanMeasurement, LaserCanTimingBudget, LaserCanRangingMode}, GrappleDeviceMessage, DEVICE_TYPE_DISTANCE_SENSOR}, request_factory};

use crate::can::GrappleCanDriver;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use grapple_frc_msgs::grapple::errors::{convert_grpl_result_to_py, GrappleResultPy};

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct LaserCAN {
  driver: GrappleCanDriver,
  last_status_frame: Option<(Instant, LaserCanMeasurement)>,
}

impl LaserCAN {
  pub fn new(can_id: u8) -> Self {
    Self {
      driver: GrappleCanDriver::new(can_id, DEVICE_TYPE_DISTANCE_SENSOR),
      last_status_frame: None
    }
  }

  fn get_measurement(&mut self) -> Option<LaserCanMeasurement> {
    self.driver.spin(&mut |_id, msg| {
      match msg {
        GrappleDeviceMessage::DistanceSensor(LaserCanMessage::Measurement(measurement)) => {
          self.last_status_frame = Some((Instant::now(), measurement));
          false
        },
        _ => true
      }
    });

    match self.last_status_frame.clone() {
      Some((time, frame)) => {
        if (Instant::now() - time) > Duration::from_millis(500) {
          self.last_status_frame = None;
          None
        } else {
          Some(frame.clone())
        }
      },
      None => None
    }
  }

  fn set_timing_budget(&mut self, budget: LaserCanTimingBudget) -> GrappleResult<'static, ()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetTimingBudget(data)));
    decode(self.driver.request(encode(budget), 200, 3)?)
      .map_err(|e| e.to_static())?.map_err(|e| e.to_static())?;
    Ok(())
  }

  fn set_roi(&mut self, roi: LaserCanRoi) -> GrappleResult<'static, ()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetRoi(data)));
    decode(self.driver.request(encode(roi), 200, 3)?)
      .map_err(|e| e.to_static())?.map_err(|e| e.to_static())?;
    Ok(())
  }

  fn set_range(&mut self, mode: LaserCanRangingMode) -> GrappleResult<'static, ()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetRange(data)));
    decode(self.driver.request(encode(mode), 200, 3)?)
      .map_err(|e| e.to_static())?.map_err(|e| e.to_static())?;
    Ok(())
  }
}

#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "pyo3", pymethods)]
impl LaserCAN {
  #[new]
  pub fn new_py(can_id: u8) -> Self {
    return Self::new(can_id);
  }
  
  #[pyo3(name = "get_measurement")]
  fn get_measurement_py(&mut self) -> Option<LaserCanMeasurement> {
    return self.get_measurement()
  }

  #[pyo3(name = "set_timing_budget")]
  fn set_timing_budget_py(&mut self, budget: LaserCanTimingBudget, py: Python<'_>) -> PyResult<GrappleResultPy> {
    convert_grpl_result_to_py(py, self.set_timing_budget(budget))
  }

  #[pyo3(name = "set_roi")]
  fn set_roi_py(&mut self, roi: LaserCanRoi, py: Python<'_>) -> PyResult<GrappleResultPy> {
    convert_grpl_result_to_py(py, self.set_roi(roi))
  }

  #[pyo3(name = "set_range")]
  fn set_range_py(&mut self, mode: LaserCanRangingMode, py: Python<'_>) -> PyResult<GrappleResultPy> {
    convert_grpl_result_to_py(py, self.set_range(mode))
  }
}

#[cfg(feature = "c")]
mod c {
  use grapple_frc_msgs::grapple::lasercan::{LaserCanMeasurement, LaserCanTimingBudget, LaserCanRoi, LaserCanRangingMode};

  use crate::{COptional, UnitCGrappleResult};

  use super::LaserCAN;

  // C
  #[no_mangle]
  pub extern "C" fn lasercan_new(can_id: u8) -> *mut LaserCAN {
    Box::into_raw(Box::new(LaserCAN::new(can_id)))
  }

  #[no_mangle]
  pub extern "C" fn lasercan_free(lc: *mut LaserCAN) {
    if lc.is_null() { return; }
    unsafe { drop(Box::from_raw(lc)) }
  }  

  // Need to wrap this so MSVC doesn't complain about using C++ generics in extern "C"
  #[repr(C)]
  pub struct MaybeMeasurement(COptional<LaserCanMeasurement>);

  #[no_mangle]
  pub extern "C" fn lasercan_get_measurement(inst: *mut LaserCAN) -> MaybeMeasurement {
    MaybeMeasurement(unsafe { (*inst).get_measurement().into() })
  }

  #[no_mangle]
  pub extern "C" fn lasercan_set_timing_budget(inst: *mut LaserCAN, budget: LaserCanTimingBudget) -> UnitCGrappleResult {
    unsafe {
      UnitCGrappleResult((*inst).set_timing_budget(budget).map(Into::into).into())
    }
  }

  #[no_mangle]
  pub extern "C" fn lasercan_set_roi(inst: *mut LaserCAN, roi: LaserCanRoi) -> UnitCGrappleResult {
    unsafe {
      UnitCGrappleResult((*inst).set_roi(roi).map(Into::into).into())
    }
  }

  #[no_mangle]
  pub extern "C" fn lasercan_set_range(inst: *mut LaserCAN, mode: LaserCanRangingMode) -> UnitCGrappleResult {
    unsafe {
      UnitCGrappleResult((*inst).set_range(mode).map(Into::into).into())
    }
  }
}

#[cfg(feature = "jni")]
mod jni {
  use grapple_frc_msgs::grapple::lasercan::{LaserCanRangingMode, LaserCanTimingBudget, LaserCanRoi, LaserCanRoiU4};
use jni::{objects::{JObject, JClass, JValueGen}, JNIEnv, sys::{jint, jlong, jobject, jboolean}};

  use crate::JNIResultExtension;

use super::LaserCAN;

  // JNI
  fn get_handle<'local>(env: &mut JNIEnv<'local>, inst: JObject<'local>) -> *mut LaserCAN {
    let handle = env.get_field(inst, "handle", "Lau/grapplerobotics/LaserCan$Handle;").unwrap().l().unwrap();
    env.get_field(handle, "handle", "J").unwrap().j().unwrap() as *mut LaserCAN
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_init<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    can_id: jint,
  ) -> jlong {
    let ptr = Box::into_raw(Box::new(LaserCAN::new(can_id as u8)));
    return ptr as jlong;
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_free<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
  ) {
    unsafe { drop(Box::from_raw(handle as *mut LaserCAN)); }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_getMeasurementInternal<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
  ) -> jobject {
    let lc = get_handle(&mut env, inst);
    let status = unsafe { (*lc).get_measurement() };

    match status {
      None => JObject::null().into_raw(),
      Some(status) => {
        let cls = env.find_class("au/grapplerobotics/interfaces/LaserCanInterface$RegionOfInterest").unwrap();
        let roi = env.new_object(cls, "(IIII)V", &[
          JValueGen::Int(status.roi.x.0 as jint),
          JValueGen::Int(status.roi.y.0 as jint),
          JValueGen::Int(status.roi.w.0 as jint),
          JValueGen::Int(status.roi.h.0 as jint),
        ]).unwrap();

        let cls = env.find_class("au/grapplerobotics/interfaces/LaserCanInterface$Measurement").unwrap();
        env.new_object(cls, "(IIIZILau/grapplerobotics/interfaces/LaserCanInterface$RegionOfInterest;)V", &[
          JValueGen::Int(status.status as jint),
          JValueGen::Int(status.distance_mm as jint),
          JValueGen::Int(status.ambient as jint),
          JValueGen::Bool((status.mode == LaserCanRangingMode::Long) as jboolean),
          JValueGen::Int(status.budget as u8 as jint),
          JValueGen::Object(&roi)
        ]).unwrap().into_raw()
      }
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_setRangingMode<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    is_long: bool,
  ) {
    let lc = get_handle(&mut env, inst);
    unsafe {
      (*lc).set_range(if is_long { LaserCanRangingMode::Long } else { LaserCanRangingMode::Short })
        .with_jni_throw(&mut env, "ConfigurationFailedException", |_| {});
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_setTimingBudget<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    budget: jint,
  ) {
    let lc = get_handle(&mut env, inst);
    unsafe { (*lc).set_timing_budget(match budget as u8 {
      20 => LaserCanTimingBudget::TB20ms,
      33 => LaserCanTimingBudget::TB33ms,
      50 => LaserCanTimingBudget::TB50ms,
      100 => LaserCanTimingBudget::TB100ms,
      _ => panic!("Invalid Timing Budget")
    }).with_jni_throw(&mut env, "ConfigurationFailedException", |_| {}) };
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_LaserCan_setRoi<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    x: jint,
    y: jint,
    w: jint,
    h: jint,
  ) {
    let lc = get_handle(&mut env, inst);
    unsafe {
      (*lc).set_roi(LaserCanRoi {
        x: LaserCanRoiU4(x as u8),
        y: LaserCanRoiU4(y as u8),
        w: LaserCanRoiU4(w as u8),
        h: LaserCanRoiU4(h as u8),
      }).with_jni_throw(&mut env, "ConfigurationFailedException", |_| {});
    }
  }
}
