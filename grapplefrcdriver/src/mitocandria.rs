use std::{borrow::Cow, time::{Duration, Instant}};

use bounded_static::ToBoundedStatic;
use grapple_frc_msgs::{binmarshal::AsymmetricCow, grapple::{errors::{GrappleError, GrappleResult}, mitocandria::{self, MitocandriaAdjustableChannelRequest, MitocandriaChannelStatus, MitocandriaSwitchableChannelRequest}, GrappleDeviceMessage, Request, DEVICE_TYPE_POWER_DISTRIBUTION_MODULE}, request_factory};

use crate::can::GrappleCanDriver;

pub struct MitoCANdria {
  driver: GrappleCanDriver,
  last_status_frame: Option<(Instant, mitocandria::MitocandriaStatusFrame)>,
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
        MitocandriaChannelStatus::Switchable { .. } => {
          self.set_switchable(MitocandriaSwitchableChannelRequest { channel, enabled })
        },
        MitocandriaChannelStatus::Adjustable { voltage_setpoint, .. } => {
          self.set_adjustable(MitocandriaAdjustableChannelRequest { channel, enabled, voltage: *voltage_setpoint })
        }
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
          self.set_adjustable(MitocandriaAdjustableChannelRequest { channel, enabled: false, voltage: (voltage * 1000.0) as u16 })
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