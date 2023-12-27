use std::{net::{TcpListener, TcpStream}, io::{Read, Write, ErrorKind}, time::Duration};

use grapple_frc_msgs::{bridge::BridgedCANMessage, binmarshal::{BitView, BinMarshal, VecBitWriter, LengthTaggedVec, BitWriter}, MessageId};

use crate::{hal_safe_call, HAL_CAN_SendMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT, HAL_CAN_OpenStreamSession, HAL_CANStreamMessage, HAL_CAN_ReadStreamSession, HAL_CAN_CloseStreamSession};

fn handle_client(session_handle: u32, mut stream: TcpStream) -> anyhow::Result<()> {
  let mut read_buf = Vec::with_capacity(1024);

  stream.set_nonblocking(true)?;

  loop {
    // Read from socket first
    match stream.read_to_end(&mut read_buf) {
      Ok(_) => (),
      Err(e) if e.kind() == ErrorKind::WouldBlock => (),
      Err(e) => anyhow::bail!(e)
    };

    if read_buf.len() >= 2 {
      let msg_len: usize = u16::from_le_bytes([ read_buf[0], read_buf[1] ]) as usize;

      if (read_buf.len() - 2) >= msg_len {
        let mut next_buf = read_buf.split_off(msg_len + 2);

        let bridged_msg = BridgedCANMessage::read(&mut BitView::new(&read_buf[2..]), ()).ok_or(anyhow::anyhow!("Invalid Message!"))?;
        hal_safe_call!(HAL_CAN_SendMessage(
          bridged_msg.id.into(),
          bridged_msg.data.0.as_slice().as_ptr(),
          bridged_msg.data.0.len() as u8,
          HAL_CAN_SEND_PERIOD_NO_REPEAT as i32
        ))?;

        next_buf.reserve(1024);
        read_buf = next_buf;
      }
    }

    // See if there's anything to write
    let mut stream_messages: [HAL_CANStreamMessage; 1024] = [HAL_CANStreamMessage { ..Default::default() }; 1024];
    let mut n_read = 0u32;
    let result = hal_safe_call!(HAL_CAN_ReadStreamSession(session_handle, &mut stream_messages as *mut HAL_CANStreamMessage, 1024, &mut n_read as *mut u32));

    match result {
      Ok(_) => {
        for msg in &stream_messages[0..n_read as usize] {
          let message_id: MessageId = msg.messageID.into();
          let bridged_msg = BridgedCANMessage { id: message_id, timestamp: msg.timeStamp, data: LengthTaggedVec::new(msg.data[0..msg.dataSize as usize].to_vec()) };
  
          let mut write_buf = VecBitWriter::new();
          bridged_msg.write(&mut write_buf, ());
          let mut slice = write_buf.slice();
          
          let l = u16::to_le_bytes(slice.len() as u16);
          let mut slice1 = &l[..];
  
          // Block on writes to the socket
  
          while !slice1.is_empty() {
            match stream.write(slice1) {
              Ok(0) => anyhow::bail!("Failed to write"),
              Ok(n) => slice1 = &slice1[n..],
              Err(e) if e.kind() == ErrorKind::Interrupted || e.kind() == ErrorKind::WouldBlock => {},
              Err(e) => anyhow::bail!("Write error: {}", e)
            }
          }
  
          while !slice.is_empty() {
            match stream.write(slice) {
              Ok(0) => anyhow::bail!("Failed to write"),
              Ok(n) => slice = &slice[n..],
              Err(e) if e.kind() == ErrorKind::Interrupted || e.kind() == ErrorKind::WouldBlock => {},
              Err(e) => anyhow::bail!("Write error: {}", e)
            }
          }
        }
      },
      Err(_) => ()
    }

    std::thread::sleep(Duration::from_millis(1));
  }
}

fn start_can_bridge() -> anyhow::Result<()> {
  let server = TcpListener::bind("172.22.11.2:8006")?;

  for stream in server.incoming() {
    // Only handle one client at a time, otherwise the process lives forever when GrappleHook is done.
    let mut session_handle = 0u32;
    hal_safe_call!(HAL_CAN_OpenStreamSession(&mut session_handle as *mut u32, 0u32, 0u32, 1024))?;
    let result = handle_client(session_handle, stream?);
    unsafe { HAL_CAN_CloseStreamSession(session_handle) };
    return result;
  }

  Ok(())
}

#[no_mangle]
pub extern "C" fn start_can_bridge_c() {
  start_can_bridge().unwrap()
}