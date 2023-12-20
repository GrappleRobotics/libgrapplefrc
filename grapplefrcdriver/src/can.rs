use std::time::{Duration, Instant};

use grapple_frc_msgs::{
    binmarshal::{BinMarshal, BitView},
    grapple::{
        fragments::FragmentReassembler, GrappleDeviceMessage, GrappleMessageId, MaybeFragment,
    },
    MessageId, Validate,
};

use crate::{
    hal_safe_call, HAL_CAN_ReceiveMessage, HAL_CAN_SendMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT,
};

pub struct GrappleCanDriver {
    can_id: u8,
    device_type: u8,
    fragment_id: u8,
    reassembler: FragmentReassembler,
}

impl GrappleCanDriver {
    pub fn new(can_id: u8, device_type: u8) -> Self {
        Self {
            can_id,
            device_type,
            fragment_id: 0,
            reassembler: FragmentReassembler::new(1000),
        }
    }

    pub fn spin<F: FnMut(GrappleMessageId, GrappleDeviceMessage) -> bool>(
        &mut self,
        consumer: &mut F,
    ) {
        let id: MessageId = GrappleMessageId {
            device_type: self.device_type,
            fragment_flag: false,
            ack_flag: false,
            api_class: 0,
            api_index: 0,
            device_id: self.can_id,
        }
        .into();

        let mask: MessageId = GrappleMessageId {
            device_type: 0xFF,
            fragment_flag: false,
            ack_flag: false,
            api_class: 0,
            api_index: 0,
            device_id: 0xFF,
        }
        .into();

        let mut data = [0u8; 8];
        let mut len = 0u8;
        let mut timestamp = 0u32;

        loop {
            let mut message_id: u32 = id.into();
            let result = hal_safe_call!(HAL_CAN_ReceiveMessage(
                &mut message_id as *mut u32,
                mask.into(),
                data.as_mut_ptr(),
                &mut len as *mut u8,
                &mut timestamp as *mut u32
            ));

            match result {
                Ok(_) => {
                    let mut view = BitView::new(&mut data);
                    let this_message_id: MessageId = message_id.into();
                    let msg = MaybeFragment::read(&mut view, this_message_id.into());
                    match msg {
                        Some(msg) => {
                            match self.reassembler.defragment(
                                timestamp as i64,
                                &this_message_id,
                                msg,
                            ) {
                                Some(m) => {
                                    let cont = consumer(this_message_id.into(), m);
                                    if !cont {
                                        break;
                                    }
                                }
                                None => (),
                            }
                        }
                        None => (),
                    }
                }
                Err(_) => break,
            }
        }
    }

    pub fn send(&mut self, msg: GrappleDeviceMessage) -> anyhow::Result<()> {
        msg.validate().map_err(|e| anyhow::anyhow!("{}", e))?;

        let mut msgs = vec![];
        FragmentReassembler::maybe_fragment(
            self.can_id,
            msg,
            self.fragment_id.wrapping_add(1),
            &mut |id, buf| {
                msgs.push((id, buf.to_vec()));
            },
        );

        for (id, buf) in msgs {
            hal_safe_call!(HAL_CAN_SendMessage(
                id.into(),
                buf.as_ptr(),
                buf.len() as u8,
                HAL_CAN_SEND_PERIOD_NO_REPEAT as i32
            ))?;
        }
        Ok(())
    }

    pub fn request(
        &mut self,
        mut msg: GrappleDeviceMessage,
        timeout_ms: usize,
    ) -> anyhow::Result<GrappleDeviceMessage> {
        let mut id = GrappleMessageId::new(self.can_id);
        msg.update(&mut id);

        let mut complement_id = id.clone();
        complement_id.ack_flag = true;

        self.send(msg)?;
        let started = Instant::now();

        while Instant::now() - started < Duration::from_millis(timeout_ms as u64) {
            let mut ret = None;

            self.spin(&mut |received_id, received_msg| {
                if received_id == complement_id {
                    ret = Some(received_msg);
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
        }

        anyhow::bail!(
            "CAN Request Timed Out! Is your device plugged in and the firmware up to date?"
        );
    }
}
