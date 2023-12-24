use std::{time::{Duration, Instant}, ffi::c_int, ops::{DerefMut, Deref}};

use grapple_frc_msgs::{grapple::{errors::{CowStr, GrappleError}, lasercan::{LaserCanStatusFrame, LaserCanMessage, LaserCanRoi, LaserCanRoiU4}, GrappleDeviceMessage, Request}, binmarshal::HasTags, request_factory};
use jni::objects::{JClass, JObject, JValueGen};
use jni::sys::{jint, jlong, jobject, jboolean};
use jni::JNIEnv;

use crate::{with_err, COptional, JNIResultExtension, can::GrappleCanDriver};

pub trait LaserCanImpl {
  fn status(&mut self) -> Option<LaserCanStatusFrame>;
  fn set_timing_budget(&mut self, budget: u8) -> anyhow::Result<()>;
  fn set_roi(&mut self, roi: LaserCanRoi) -> anyhow::Result<()>;
  fn set_range(&mut self, long: bool) -> anyhow::Result<()>;
}

pub struct NativeLaserCan {
  driver: GrappleCanDriver,
  last_status_frame: Option<(Instant, LaserCanStatusFrame)>,
}

impl NativeLaserCan {
  pub fn new(can_id: u8) -> Self {
    Self {
      driver: GrappleCanDriver::new(can_id, <GrappleDeviceMessage as HasTags>::Tags::DistanceSensor.to_tag()),
      last_status_frame: None
    }
  }
}

impl LaserCanImpl for NativeLaserCan {
  fn status(&mut self) -> Option<LaserCanStatusFrame> {
    self.driver.spin(&mut |_id, msg| {
      match msg {
        GrappleDeviceMessage::DistanceSensor(LaserCanMessage::Status(status)) => {
          self.last_status_frame = Some((Instant::now(), status));
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

  fn set_timing_budget(&mut self, budget: u8) -> anyhow::Result<()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetTimingBudget(data)));
    decode(self.driver.request(encode(budget), 500)?)??;
    Ok(())
  }

  fn set_roi(&mut self, roi: LaserCanRoi) -> anyhow::Result<()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetRoi(data)));
    decode(self.driver.request(encode(roi), 500)?)??;
    Ok(())
  }

  fn set_range(&mut self, long: bool) -> anyhow::Result<()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::DistanceSensor(LaserCanMessage::SetRange(data)));
    decode(self.driver.request(encode(long), 500)?)??;
    Ok(())
  }
}

pub struct LaserCanDevice {
  backend: Box<dyn LaserCanImpl>
}

impl LaserCanDevice {
  pub fn new(can_id: u8) -> Self {
    Self { backend: Box::new(NativeLaserCan::new(can_id)) }
  }
}

impl Deref for LaserCanDevice {
  type Target = Box<dyn LaserCanImpl>;

  fn deref(&self) -> &Self::Target {
    &self.backend
  }
}

impl DerefMut for LaserCanDevice {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.backend
  }
}

// C

#[no_mangle]
pub extern "C" fn lasercan_new(can_id: u8) -> *mut LaserCanDevice {
  Box::into_raw(Box::new(LaserCanDevice::new(can_id)))
}

#[no_mangle]
pub extern "C" fn lasercan_free(lc: *mut LaserCanDevice) {
  if lc.is_null() { return; }
  unsafe { drop(Box::from_raw(lc)) }
}  

// Need to wrap this so MSVC doesn't complain about using C++ generics in extern "C"
#[repr(C)]
pub struct MaybeStatusFrame(COptional<LaserCanStatusFrame>);

#[no_mangle]
pub extern "C" fn lasercan_get_status(inst: *mut LaserCanDevice) -> MaybeStatusFrame {
  MaybeStatusFrame(unsafe { (*inst).status().into() })
}

#[no_mangle]
pub extern "C" fn lasercan_set_timing_budget(inst: *mut LaserCanDevice, budget: u8) -> c_int {
  unsafe {
    with_err((*inst).set_timing_budget(budget))
  }
}

#[no_mangle]
pub extern "C" fn lasercan_set_roi(inst: *mut LaserCanDevice, roi: LaserCanRoi) -> c_int {
  unsafe {
    with_err((*inst).set_roi(roi))
  }
}

#[no_mangle]
pub extern "C" fn lasercan_set_range(inst: *mut LaserCanDevice, long: bool) -> c_int {
  unsafe {
    with_err((*inst).set_range(long))
  }
}

// JNI

fn get_handle<'local>(env: &mut JNIEnv<'local>, inst: JObject<'local>) -> *mut LaserCanDevice {
  let handle = env.get_field(inst, "handle", "Lau/grapplerobotics/LaserCan$Handle;").unwrap().l().unwrap();
  env.get_field(handle, "handle", "J").unwrap().j().unwrap() as *mut LaserCanDevice
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_init<'local>(
  mut _env: JNIEnv<'local>,
  _class: JClass<'local>,
  can_id: jint,
) -> jlong {
  let ptr = Box::into_raw(Box::new(LaserCanDevice::new(can_id as u8)));
  return ptr as jlong;
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_free<'local>(
  mut _env: JNIEnv<'local>,
  _class: JClass<'local>,
  handle: jlong,
) {
  unsafe { drop(Box::from_raw(handle as *mut LaserCanDevice)); }
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_status<'local>(
  mut env: JNIEnv<'local>,
  inst: JObject<'local>,
) -> jobject {
  let lc = get_handle(&mut env, inst);
  let status = unsafe { (*lc).status() };

  match status {
    None => JObject::null().into_raw(),
    Some(status) => {
      let cls = env.find_class("au/grapplerobotics/LaserCan$RegionOfInterest").unwrap();
      let roi = env.new_object(cls, "(IIII)V", &[
        JValueGen::Int(status.roi.x.0 as jint),
        JValueGen::Int(status.roi.y.0 as jint),
        JValueGen::Int(status.roi.w.0 as jint),
        JValueGen::Int(status.roi.h.0 as jint),
      ]).unwrap();

      let cls = env.find_class("au/grapplerobotics/LaserCan$Measurement").unwrap();
      env.new_object(cls, "(IIIZILau/grapplerobotics/LaserCan$RegionOfInterest;)V", &[
        JValueGen::Int(status.status as jint),
        JValueGen::Int(status.distance_mm as jint),
        JValueGen::Int(status.ambient as jint),
        JValueGen::Bool(status.long as jboolean),
        JValueGen::Int(status.budget_ms as jint),
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
  unsafe { (*lc).set_range(is_long).with_jni_throw(&mut env, |_| {}) }
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_setTimingBudget<'local>(
  mut env: JNIEnv<'local>,
  inst: JObject<'local>,
  budget: jint,
) {
  let lc = get_handle(&mut env, inst);
  unsafe { (*lc).set_timing_budget(budget as u8).with_jni_throw(&mut env, |_| {}) }
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
    }).with_jni_throw(&mut env, |_| {})
  }
}
