use std::{borrow::Cow, time::Duration};

use futures::{SinkExt, StreamExt};
use grapple_frc_msgs::{binmarshal::{BitView, BitWriter, Demarshal, LengthTaggedPayload, Marshal, VecBitWriter}, bridge::BridgedCANMessage, MessageId};
use warp::{filters::ws::{Message, WebSocket}, Filter};

use crate::{hal_safe_call, HAL_CANStreamMessage, HAL_CAN_CloseStreamSession, HAL_CAN_OpenStreamSession, HAL_CAN_ReadStreamSession, HAL_CAN_SendMessage, HAL_CAN_SEND_PERIOD_NO_REPEAT};

pub struct CanDropGuard {
  session_handle: u32
}

impl Drop for CanDropGuard {
  fn drop(&mut self) {
    unsafe { HAL_CAN_CloseStreamSession(self.session_handle) };
  }
}

async fn client_connected(ws: WebSocket) -> anyhow::Result<()> {
  let (mut tx, mut rx) = ws.split();
  println!("CAN Bridge - WebSocket Client Connected!");

  let mut recv_interval = tokio::time::interval(Duration::from_millis(1));

  let mut session_handle = 0u32;
  hal_safe_call!(HAL_CAN_OpenStreamSession(&mut session_handle as *mut u32, 0u32, 0u32, 1024))?;
  let guard = CanDropGuard { session_handle };

  loop {
    tokio::select! {
      _ = recv_interval.tick() => {
        // See if there's anything to write
        let mut stream_messages: [HAL_CANStreamMessage; 1024] = [HAL_CANStreamMessage { ..Default::default() }; 1024];
        let mut n_read = 0u32;
        let result = hal_safe_call!(HAL_CAN_ReadStreamSession(guard.session_handle, &mut stream_messages as *mut HAL_CANStreamMessage, 1024, &mut n_read as *mut u32));

        match result {
          Ok(_) => {
            for msg in &stream_messages[0..n_read as usize] {
              let message_id: MessageId = msg.messageID.into();
              let bridged_msg = BridgedCANMessage { id: message_id, timestamp: msg.timeStamp, data: Cow::Borrowed(Into::<&LengthTaggedPayload<_>>::into(&msg.data[0..msg.dataSize as usize])).into() };
      
              let mut write_buf = VecBitWriter::new();
              bridged_msg.write(&mut write_buf, ()).ok();
              let slice = write_buf.slice();

              tx.send(Message::binary(slice)).await?;
            }
          },
          Err(_) => ()
        }
      },
      msg = rx.next() => match msg {
        Some(Ok(msg)) => {
          let bytes = msg.as_bytes();
          if msg.is_ping() {
            tx.send(Message::pong(bytes)).await?;
          } else if msg.is_binary() {
            let bridged_msg = BridgedCANMessage::read(&mut BitView::new(bytes), ()).map_err(|e| anyhow::anyhow!("Invalid Message! {:?}", e))?;
            let r = bridged_msg.data.as_ref();
            let msg_data = r.as_ref();
            hal_safe_call!(HAL_CAN_SendMessage(
              bridged_msg.id.into(),
              msg_data.as_ptr(),
              msg_data.len() as u8,
              HAL_CAN_SEND_PERIOD_NO_REPEAT as i32
            ))?;
          } else if msg.is_close() {
            break;
          } else {
            println!("Unknown WebSocket Message: {:?}", msg);
          }
        },
        None => (),
        Some(Err(e)) => {
          println!("Websocket Error : {}", e);
          break;
        }
      }
    }
  }

  Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn run_ws_can_bridge(mut port: i32) -> anyhow::Result<()> {
  let routes = warp::path::end()
    .and(warp::ws())
    .map(|ws: warp::ws::Ws| {
      ws.on_upgrade(move |websocket| async {
        match client_connected(websocket).await {
          Ok(()) => (),
          Err(e) => println!("Error in WebSocket handler: {}", e)
        }
      })
    });
  
  if port <= 0 {
    port = 7171;
  }

  warp::serve(routes).run(([0, 0, 0, 0], port as u16)).await;
  Ok(())
}

pub fn run_ws_can_bridge_in_background(port: i32) {
  std::thread::spawn(move || {
    run_ws_can_bridge(port).unwrap()
  });
}

#[no_mangle]
pub extern "C" fn run_ws_can_bridge_c(port: i32) {
  run_ws_can_bridge(port).unwrap()
}

#[no_mangle]
pub extern "C" fn run_ws_can_bridge_in_background_c(port: i32) {
  run_ws_can_bridge_in_background(port)
}

#[cfg(feature = "jni")]
mod jni {
  use jni::{objects::JClass, sys::jint, JNIEnv};

  use super::{run_ws_can_bridge, run_ws_can_bridge_in_background};

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_CanBridge_runWebsocket<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    port: jint
  ) {
    run_ws_can_bridge(port).unwrap()
  }

  #[no_mangle]
  pub extern "system" fn Java_au_grapplerobotics_CanBridge_runWebsocketInBackground<'local>(
    mut _env: JNIEnv<'local>,
    _class: JClass<'local>,
    port: jint
  ) {
    run_ws_can_bridge_in_background(port)
  }
}