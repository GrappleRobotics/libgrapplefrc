use std::{borrow::Cow, time::{Duration, Instant}};

use bounded_static::ToBoundedStatic;
use grapple_frc_msgs::{binmarshal::AsymmetricCow, grapple::{errors::{GrappleError, GrappleResult}, mitocandria::{self, MitocandriaAdjustableChannelRequest, MitocandriaChannelStatus, MitocandriaSwitchableChannelRequest}, GrappleDeviceMessage, Request, DEVICE_TYPE_POWER_DISTRIBUTION_MODULE}, request_factory};

use crate::can::GrappleCanDriver;

pub struct MitoCANdria {
  driver: GrappleCanDriver,
  last_status_frame: Option<(Instant, mitocandria::MitocandriaStatusFrame)>
}

impl MitoCANdria {
  pub fn new(can_id: u8) -> Self {
    Self {
      driver: GrappleCanDriver::new(can_id, DEVICE_TYPE_POWER_DISTRIBUTION_MODULE),
      last_status_frame: None,
    }
  }

  fn get_status(&mut self) -> Option<mitocandria::MitocandriaStatusFrame> {
    self.driver.spin(&mut |_id, msg| {
      match msg {
        GrappleDeviceMessage::PowerDistributionModule(mitocandria::MitocandriaMessage::StatusFrame(frame)) => {
          self.last_status_frame = Some((Instant::now(), frame));
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

  fn set_switchable(&mut self, req: MitocandriaSwitchableChannelRequest) -> GrappleResult<'static, ()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::PowerDistributionModule(
      mitocandria::MitocandriaMessage::ChannelRequest(
        mitocandria::MitocandriaChannelRequest::SetSwitchableChannel(data)
      )
    ));
    decode(self.driver.request(encode(req), 500)?)
      .map_err(|e| e.to_static())?.map_err(|e| e.to_static())?;
    Ok(())
  }

  fn set_adjustable(&mut self, req: MitocandriaAdjustableChannelRequest) -> GrappleResult<'static, ()> {
    let (encode, decode) = request_factory!(data, GrappleDeviceMessage::PowerDistributionModule(
      mitocandria::MitocandriaMessage::ChannelRequest(
        mitocandria::MitocandriaChannelRequest::SetAdjustableChannel(data)
      )
    ));
    decode(self.driver.request(encode(req), 500)?)
      .map_err(|e| e.to_static())?.map_err(|e| e.to_static())?;
    Ok(())
  }

  pub fn get_current(&mut self, channel: u8) -> Option<GrappleResult<'static, f64>> {
    let status = self.get_status()?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { current } => Some(Ok(*current as f64 / 1000.0)),
        MitocandriaChannelStatus::Switchable { current, .. } => Some(Ok(*current as f64 / 1000.0)),
        MitocandriaChannelStatus::Adjustable { current, .. } => Some(Ok(*current as f64 / 1000.0))
      },
      None => Some(Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!"))))),
    }
  }

  pub fn get_voltage(&mut self, channel: u8) -> Option<GrappleResult<'static, f64>> {
    let status = self.get_status()?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { .. } => Some(Ok(5.0)),
        MitocandriaChannelStatus::Switchable { .. } => Some(Ok(5.0)),
        MitocandriaChannelStatus::Adjustable { voltage, .. } => Some(Ok(*voltage as f64 / 1000.0))
      },
      None => Some(Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!"))))),
    }
  }

  pub fn get_voltage_setpoint(&mut self, channel: u8) -> Option<GrappleResult<'static, f64>> {
    let status = self.get_status()?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { .. } => Some(Ok(5.0)),
        MitocandriaChannelStatus::Switchable { .. } => Some(Ok(5.0)),
        MitocandriaChannelStatus::Adjustable { voltage_setpoint, .. } => Some(Ok(*voltage_setpoint as f64 / 1000.0))
      },
      None => Some(Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!"))))),
    }
  }

  pub fn get_enabled(&mut self, channel: u8) -> Option<GrappleResult<'static, bool>> {
    let status = self.get_status()?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { .. } => Some(Ok(true)),
        MitocandriaChannelStatus::Switchable { enabled, .. } => Some(Ok(*enabled)),
        MitocandriaChannelStatus::Adjustable { enabled, .. } => Some(Ok(*enabled).into())
      },
      None => Some(Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!"))))),
    }
  }

  pub fn set_enabled(&mut self, channel: u8, enabled: bool) -> GrappleResult<'static, ()> {
    let status = self.get_status().ok_or(GrappleError::FailedAssertion(AsymmetricCow(Cow::Borrowed("MitoCANdria Offline"))))?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { .. } => Err(GrappleError::FailedAssertion(AsymmetricCow(Cow::Borrowed("Cannot switch a non-switchable channel"))))?,
        MitocandriaChannelStatus::Switchable { .. } | MitocandriaChannelStatus::Adjustable { .. } => {
          self.set_switchable(MitocandriaSwitchableChannelRequest { channel, enabled })
        },
      },
      None => Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!")))),
    }
  }

  pub fn set_voltage(&mut self, channel: u8, voltage: f64) -> GrappleResult<'static, ()> {
    let status = self.get_status().ok_or(GrappleError::FailedAssertion(AsymmetricCow(Cow::Borrowed("MitoCANdria Offline"))))?;
    match status.channels.get(channel as usize) {
      Some(chan) => match chan {
        MitocandriaChannelStatus::NonSwitchable { .. } | MitocandriaChannelStatus::Switchable { .. } => {
          Err(GrappleError::FailedAssertion(AsymmetricCow(Cow::Borrowed("Cannot adjust voltage on a non-adjustable channel"))))?
        },
        MitocandriaChannelStatus::Adjustable { .. } => {
          self.set_adjustable(MitocandriaAdjustableChannelRequest { channel, voltage: (voltage * 1000.0) as u16 })
        }
      },
      None => Err(GrappleError::ParameterOutOfBounds(AsymmetricCow(Cow::Borrowed("Invalid channel!")))),
    }
  }
}

#[cfg(feature = "c")]
mod c {
  use crate::{MaybeBoolResult, MaybeDoubleResult, UnitCGrappleResult};

  use super::MitoCANdria;

  // C
  #[no_mangle]
  pub extern "C" fn mitocandria_new(can_id: u8) -> *mut MitoCANdria {
    Box::into_raw(Box::new(MitoCANdria::new(can_id)))
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_free(lc: *mut MitoCANdria) {
    if lc.is_null() { return; }
    unsafe { drop(Box::from_raw(lc)) }
  }  

  #[no_mangle]
  pub extern "C" fn mitocandria_get_channel_current(inst: *mut MitoCANdria, channel: u8) -> MaybeDoubleResult {
    unsafe { MaybeDoubleResult((*inst).get_current(channel).map(Into::into).into()) }
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_get_channel_enabled(inst: *mut MitoCANdria, channel: u8) -> MaybeBoolResult {
    unsafe { MaybeBoolResult((*inst).get_enabled(channel).map(Into::into).into()) }
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_get_channel_voltage(inst: *mut MitoCANdria, channel: u8) -> MaybeDoubleResult {
    unsafe { MaybeDoubleResult((*inst).get_voltage(channel).map(Into::into).into()) }
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_get_channel_voltage_setpoint(inst: *mut MitoCANdria, channel: u8) -> MaybeDoubleResult {
    unsafe { MaybeDoubleResult((*inst).get_voltage_setpoint(channel).map(Into::into).into()) }
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_set_channel_enabled(inst: *mut MitoCANdria, channel: u8, enabled: bool) -> UnitCGrappleResult {
    unsafe { UnitCGrappleResult((*inst).set_enabled(channel, enabled).map(Into::into).into()) }
  }

  #[no_mangle]
  pub extern "C" fn mitocandria_set_channel_voltage(inst: *mut MitoCANdria, channel: u8, voltage: f64) -> UnitCGrappleResult {
    unsafe { UnitCGrappleResult((*inst).set_voltage(channel, voltage).map(Into::into).into()) }
  }
}

#[cfg(feature = "jni")]
mod jni {
  use jni::{objects::{JClass, JObject, JValueGen}, sys::{jdouble, jint, jlong, jobject}, JNIEnv};

  use crate::JNIResultExtension;

use super::MitoCANdria;

  // JNI
  fn get_handle<'local>(env: &mut JNIEnv<'local>, inst: JObject<'local>) -> *mut MitoCANdria {
    let handle = env.get_field(inst, "handle", "Lau/grapplerobotics/MitoCANdria$Handle;").unwrap().l().unwrap();
    env.get_field(handle, "handle", "J").unwrap().j().unwrap() as *mut MitoCANdria
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_init<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    can_id: jint,
  ) -> jlong {
    let ptr = Box::into_raw(Box::new(MitoCANdria::new(can_id as u8)));
    return ptr as jlong;
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_free<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    handle: jlong,
  ) {
    unsafe { drop(Box::from_raw(handle as *mut MitoCANdria)); }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_getChannelCurrent<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
  ) -> jobject {
    let mc = get_handle(&mut env, inst);
    let optcls = env.find_class("java/util/OptionalDouble").unwrap();

    match unsafe { (*mc).get_current(channel as u8) } {
      Some(v) => {
        let v = v.with_jni_throw(&mut env, "CouldNotGetException", |v| v);
        match v {
          Some(v) => {
            env.call_static_method(optcls, "of", "(D)Ljava/util/OptionalDouble;", &[JValueGen::Double(v)]).unwrap().l().unwrap().into_raw()
          },
          None => JObject::null().into_raw(),   // Doesn't matter, it'll raise an exception
        }
      },
      None => {
        env.call_static_method(optcls, "empty", "()Ljava/util/OptionalDouble;", &[]).unwrap().l().unwrap().into_raw()
      },
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_getChannelEnabled<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
  ) -> jobject {
    let mc = get_handle(&mut env, inst);
    let optcls = env.find_class("java/util/OptionalInt").unwrap();

    match unsafe { (*mc).get_enabled(channel as u8) } {
      Some(v) => {
        let v = v.with_jni_throw(&mut env, "CouldNotGetException", |v| v);
        match v {
          Some(v) => {
            env.call_static_method(optcls, "of", "(I)Ljava/util/OptionalInt;", &[JValueGen::Int(v as i32)]).unwrap().l().unwrap().into_raw()
          },
          None => JObject::null().into_raw(),   // Doesn't matter, it'll raise an exception
        }
      },
      None => {
        env.call_static_method(optcls, "empty", "()Ljava/util/OptionalInt;", &[]).unwrap().l().unwrap().into_raw()
      },
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_getChannelVoltage<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
  ) -> jobject {
    let mc = get_handle(&mut env, inst);
    let optcls = env.find_class("java/util/OptionalDouble").unwrap();

    match unsafe { (*mc).get_voltage(channel as u8) } {
      Some(v) => {
        let v = v.with_jni_throw(&mut env, "CouldNotGetException", |v| v);
        match v {
          Some(v) => {
            env.call_static_method(optcls, "of", "(D)Ljava/util/OptionalDouble;", &[JValueGen::Double(v)]).unwrap().l().unwrap().into_raw()
          },
          None => JObject::null().into_raw(),   // Doesn't matter, it'll raise an exception
        }
      },
      None => {
        env.call_static_method(optcls, "empty", "()Ljava/util/OptionalDouble;", &[]).unwrap().l().unwrap().into_raw()
      },
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_getChannelVoltageSetpoint<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
  ) -> jobject {
    let mc = get_handle(&mut env, inst);
    let optcls = env.find_class("java/util/OptionalDouble").unwrap();

    match unsafe { (*mc).get_voltage_setpoint(channel as u8) } {
      Some(v) => {
        let v = v.with_jni_throw(&mut env, "CouldNotGetException", |v| v);
        match v {
          Some(v) => {
            env.call_static_method(optcls, "of", "(D)Ljava/util/OptionalDouble;", &[JValueGen::Double(v)]).unwrap().l().unwrap().into_raw()
          },
          None => JObject::null().into_raw(),   // Doesn't matter, it'll raise an exception
        }
      },
      None => {
        env.call_static_method(optcls, "empty", "()Ljava/util/OptionalDouble;", &[]).unwrap().l().unwrap().into_raw()
      },
    }
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_setChannelEnabled<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
    enabled: bool,
  ) {
    let mc = get_handle(&mut env, inst);
    unsafe { (*mc).set_enabled(channel as u8, enabled) }.with_jni_throw(&mut env, "ConfigurationFailedException", |_| {});
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_MitoCANdria_setChannelVoltage<'local>(
    mut env: JNIEnv<'local>,
    inst: JObject<'local>,
    channel: jint,
    voltage: jdouble,
  ) {
    let mc = get_handle(&mut env, inst);
    unsafe { (*mc).set_voltage(channel as u8, voltage) }.with_jni_throw(&mut env, "ConfigurationFailedException", |_| {});
  }
}