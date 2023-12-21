use std::{sync::{RwLock, mpsc, Arc}, time::Duration, ffi::c_int};

use grapple_frc_msgs::{grapple::{lasercan::{LaserCanStatusFrame, LaserCanMessage, LaserCanRoi, LaserCanRoiU4}, MANUFACTURER_GRAPPLE, DEVICE_TYPE_DISTANCE_SENSOR, GrappleDeviceMessage}, can::{CANId, CANMessage, FragmentReassembler}, ManufacturerMessage, Message, Validate};
use jni::objects::{JClass, JObject, JValueGen};
use jni::sys::{jint, jlong, jobject, jboolean};
use jni::JNIEnv;

use crate::{hal_safe_call, HAL_CAN_ReceiveMessage, HAL_CAN_SendMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT, with_err};

pub struct LaserCanDevice {
  can_id: u8,
  fragment_id: u8,
  last_status_frame: Option<LaserCanStatusFrame>,
  reassembler: FragmentReassembler,
}

impl LaserCanDevice {
  pub fn new(can_id: u8) -> LaserCanDevice {
    LaserCanDevice { can_id, fragment_id: 0, last_status_frame: None, reassembler: FragmentReassembler::new(1000) }
  }

  pub fn send_ll(&mut self, msg: Message) -> anyhow::Result<()> {
    msg.validate().map_err(|e| anyhow::anyhow!("{}", e))?;

    let fragments = FragmentReassembler::maybe_split(msg, self.fragment_id.wrapping_add(1));
    if let Some(fragments) = fragments {
      for frag in fragments {
        hal_safe_call!(HAL_CAN_SendMessage(Into::<u32>::into(frag.id), frag.payload.as_ptr(), frag.len, HAL_CAN_SEND_PERIOD_NO_REPEAT as i32))?;
      }
      Ok(())
    } else {
      Ok(())
    }
  }

  pub fn spin_once(&mut self) {
    let mut message_id: u32 = CANId { manufacturer: MANUFACTURER_GRAPPLE, device_type: DEVICE_TYPE_DISTANCE_SENSOR, device_id: self.can_id, api_class: 0x00, api_index: 0x00 }.into();
    let mask: u32 = CANId { manufacturer: 0xFF, device_type: 0xFF, device_id: 0xFF, api_class: 0x00, api_index: 0x00 }.into();
    let mut data = [0u8; 8];
    let mut len = 0u8;
    let mut timestamp = 0u32;

    let result = hal_safe_call!(HAL_CAN_ReceiveMessage(&mut message_id as *mut u32, mask, data.as_mut_ptr(), &mut len as *mut u8, &mut timestamp as *mut u32));

    match result {
      Ok(_) => {
        // Try decode
        let msg = CANMessage::decode(message_id.into(), &data[..]);
        match self.reassembler.process(timestamp as i64, len, msg) {
          Some((_, msg)) => match msg {
            CANMessage::Message(msg) => match msg.msg {
              ManufacturerMessage::Grapple(GrappleDeviceMessage::DistanceSensor(grapple_frc_msgs::grapple::lasercan::LaserCanMessage::Status(status))) => {
                self.last_status_frame = Some(status);
              },
              _ => ()
            }
            _ => ()
          },
          None => { },
        }
      },
      Err(_) => { },
    }
  }

  pub fn send(&mut self, msg: LaserCanMessage) -> anyhow::Result<()> {
    let msg = Message::new(self.can_id, ManufacturerMessage::Grapple(GrappleDeviceMessage::DistanceSensor(msg)));
    self.send_ll(msg)
  }

  pub fn status(&mut self) -> LaserCanStatusFrame {
    self.spin_once();
    match self.last_status_frame.clone() {
      Some(v) => v,
      None => LaserCanStatusFrame {
        status: 0xFF,
        distance_mm: 0,
        ambient: 0,
        long: false,
        budget_ms: 0,
        roi: LaserCanRoi { x: LaserCanRoiU4(0), y: LaserCanRoiU4(0), w: LaserCanRoiU4(0), h: LaserCanRoiU4(0) },
      }
    }
  }
}

#[no_mangle]
pub extern "C" fn lasercan_new(can_id: u8) -> *mut LaserCanDevice {
  Box::into_raw(Box::new(LaserCanDevice::new(can_id)))
}

#[no_mangle]
pub extern "C" fn lasercan_free(lc: *mut LaserCanDevice) {
  if lc.is_null() { return; }
  unsafe { drop(Box::from_raw(lc)) }
}

#[no_mangle]
pub extern "C" fn lasercan_get_status(inst: *mut LaserCanDevice) -> LaserCanStatusFrame {
  unsafe { (*inst).status() }
}

#[no_mangle]
pub extern "C" fn lasercan_set_timing_budget(inst: *mut LaserCanDevice, budget: u8) -> c_int {
  unsafe {
    with_err((*inst).send(LaserCanMessage::SetTimingBudget { budget }))
  }
}

#[no_mangle]
pub extern "C" fn lasercan_set_roi(inst: *mut LaserCanDevice, roi: LaserCanRoi) -> c_int {
  unsafe {
    with_err((*inst).send(LaserCanMessage::SetRoi { roi }))
  }
}

#[no_mangle]
pub extern "C" fn lasercan_set_range(inst: *mut LaserCanDevice, long: bool) -> c_int {
  unsafe {
    with_err((*inst).send(LaserCanMessage::SetRange { long }))
  }
}

// JNI

fn get_handle<'local>(env: &mut JNIEnv<'local>, inst: JObject<'local>) -> *mut LaserCanDevice {
  let handle = env.get_field(inst, "handle", "Lau/grapplerobotics/LaserCan$Handle;").unwrap().l().unwrap();
  env.get_field(handle, "handle", "J").unwrap().j().unwrap() as *mut LaserCanDevice
}

fn try_send<'local>(env: &mut JNIEnv<'local>, lc: *mut LaserCanDevice, msg: LaserCanMessage) {
  let cls = env.find_class("au/grapplerobotics/NativeException").unwrap();
  let ret = unsafe { (*lc).send(msg) };
  match ret {
    Ok(()) => (),
    Err(err) => env.throw_new(cls, format!("{}", err)).unwrap(),
  }
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_init<'local>(
  mut env: JNIEnv<'local>,
  class: JClass<'local>,
  can_id: jint,
) -> jlong {
  let ptr = Box::into_raw(Box::new(LaserCanDevice::new(can_id as u8)));
  return ptr as jlong;
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_free<'local>(
  mut env: JNIEnv<'local>,
  class: JClass<'local>,
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

  if status.status == 0xFF {
    JObject::null().into_raw()
  } else {
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

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_setRangingMode<'local>(
  mut env: JNIEnv<'local>,
  inst: JObject<'local>,
  is_long: bool,
) {
  let lc = get_handle(&mut env, inst);
  try_send(&mut env, lc, LaserCanMessage::SetRange { long: is_long })
}

#[no_mangle]
pub extern "system" fn Java_au_grapplerobotics_LaserCan_setTimingBudget<'local>(
  mut env: JNIEnv<'local>,
  inst: JObject<'local>,
  budget: jint,
) {
  let lc = get_handle(&mut env, inst);
  try_send(&mut env, lc, LaserCanMessage::SetTimingBudget { budget: budget as u8 })
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
  try_send(&mut env, lc, LaserCanMessage::SetRoi {
    roi: LaserCanRoi {
      x: LaserCanRoiU4(x as u8),
      y: LaserCanRoiU4(y as u8),
      w: LaserCanRoiU4(w as u8),
      h: LaserCanRoiU4(h as u8),
    },
  })
}
