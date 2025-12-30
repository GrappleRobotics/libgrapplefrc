use std::{time::{Instant, Duration}, borrow::Cow};

use bounded_static::{IntoBoundedStatic, ToBoundedStatic};
use grapple_frc_msgs::{grapple::{fragments::{FragmentReassembler, FragmentReassemblerRx, FragmentReassemblerTx}, GrappleMessageId, GrappleDeviceMessage, MaybeFragment, errors::{GrappleResult, GrappleError}}, MessageId, binmarshal::{BitView, Demarshal, MarshalUpdate}, Validate};

use crate::{hal_safe_call, HAL_CAN_ReceiveMessage, HAL_CAN_SendMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT};

pub struct GrappleCanDriver {
  can_id: u8,
  device_type: u8,
  reassembler_rx: FragmentReassemblerRx,
  reassembler_tx: FragmentReassemblerTx
}

impl GrappleCanDriver {
  pub fn new(can_id: u8, device_type: u8) -> Self {
    let (rx, tx) = FragmentReassembler::new(1000, 8).split();
    Self {
      can_id,
      device_type,
      reassembler_rx: rx,
      reassembler_tx: tx,
    }
  }

  pub fn spin<F: FnMut(GrappleMessageId, GrappleDeviceMessage) -> bool>(&mut self, consumer: &mut F) {
    let id: MessageId = GrappleMessageId {
      device_type: self.device_type,
      fragment_flag: false,
      ack_flag: false,
      api_class: 0,
      api_index: 0,
      device_id: self.can_id
    }.into();

    let mask: MessageId = GrappleMessageId {
      device_type: 0xFF,
      fragment_flag: false,
      ack_flag: false,
      api_class: 0,
      api_index: 0,
      device_id: 0xFF,
    }.into();

    let mut data = [0u8; 8];
    let mut len = 0u8;
    let mut timestamp = 0u32;

    loop {
      let mut message_id: u32 = id.into();
      let result = hal_safe_call!(HAL_CAN_ReceiveMessage(&mut message_id as *mut u32, mask.into(), data.as_mut_ptr(), &mut len as *mut u8, &mut timestamp as *mut u32));

      match result {
        Ok(_) => {
          let mut view = BitView::new(&mut data);
          let this_message_id: MessageId = message_id.into();
          let msg = MaybeFragment::read(&mut view, this_message_id.into());
          match msg {
            Ok(msg) => {
              let mut storage = Vec::with_capacity(128);
              match self.reassembler_rx.defragment(timestamp as i64, &this_message_id, msg, &mut storage) {
                Ok(Some((mid, m))) => {
                  let cont = consumer(mid, m);
                  if !cont {
                    break;
                  }
                },
                _ => (),
              }
            },
            _ => (),
          }
        },
        Err(_) => break
      }
    }
  }

  pub fn send(&mut self, msg: GrappleDeviceMessage) -> GrappleResult<'static, ()> {
    msg.validate().map_err(|e| e.to_static())?;

    let mut msgs = vec![];
    self.reassembler_tx.maybe_fragment(self.can_id, msg, &mut |id, buf| {
      msgs.push((id, buf.to_vec()));
    }).ok();

    for (id, buf) in msgs {
      hal_safe_call!(HAL_CAN_SendMessage(id.into(), buf.as_ptr(), buf.len() as u8, HAL_CAN_SEND_PERIOD_NO_REPEAT as i32))
        .map_err(|e| GrappleError::Generic(Cow::<str>::Owned(e.to_string()).into()))?;
    }
    Ok(())
  }

  fn request_inner(&mut self, msg: GrappleDeviceMessage, reply_id: GrappleMessageId, timeout_ms: usize) -> GrappleResult<'static, GrappleDeviceMessage> {
    self.send(msg)?;
    let started = Instant::now();

    while Instant::now() - started < Duration::from_millis(timeout_ms as u64) {
      let mut ret = None;

      self.spin(&mut |received_id, received_msg| {
        if received_id == reply_id {
          ret = Some(received_msg.into_static());
          false
        } else {
          true
        }
      });

      if let Some(ret) = ret {
        return Ok(ret);
      }

      // Don't destroy the CPU :)
      // This wouldn't be needed if we had interrupt / event-based reception
      // of CAN messages, but alas we do not.
      std::thread::sleep(Duration::from_millis(5));
    };

    Err(GrappleError::TimedOut(Cow::<str>::Borrowed("CAN Request Timed Out! Is your device plugged in and the firmware up to date?").into()))
  }

  pub fn request(&mut self, mut msg: GrappleDeviceMessage, timeout_ms: usize, retry: usize) -> GrappleResult<'static, GrappleDeviceMessage> {
    let mut id = GrappleMessageId::new(self.can_id);
    msg.update(&mut id);

    let mut complement_id = id.clone();
    complement_id.ack_flag = true;
    
    match self.request_inner(msg.clone(), complement_id, timeout_ms) {
      Ok(x) => Ok(x.to_static()),
      Err(_) if retry >= 1 => self.request(msg, timeout_ms, retry - 1),
      Err(e) => Err(e)
    }
  }
}