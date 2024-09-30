#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]

extern crate alloc;

use std::ffi::{c_char, CString};

use grapple_frc_msgs::grapple::errors::GrappleResult;
use jni::{JNIEnv, objects::{JThrowable, JValue}};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod calling;
pub mod can;
pub mod can_bridge;
pub mod lasercan;
pub mod mitocandria;
pub mod ws_can_bridge;

#[repr(C)]
pub enum CGrappleResult<T> {
  Ok(T),
  Err(CGrappleError),
}

#[repr(C)]
pub struct CGrappleError {
  pub message: *mut c_char,
  pub code: u8,
}

impl<'a, T> From<GrappleResult<'a, T>> for CGrappleResult<T> {
  fn from(value: GrappleResult<'a, T>) -> Self {
    match value {
      Err(e) => {
        let str = CString::new(format!("{}", e)).unwrap();
        CGrappleResult::Err(CGrappleError {
          message: str.into_raw(),
          code: e.to_error_code()
        })
      },
      Ok(v) => CGrappleResult::Ok(v)
    }
  }
}

// Needed because bindgen doesn't have a () type.
#[repr(C)]
pub struct Empty { _sentinel: u8 }

impl From<()> for Empty {
  fn from(_: ()) -> Self {
    Self { _sentinel: 0x00 }
  }
}

// Needed because otherwise the generated headers have templates :(
#[repr(C)]
pub struct UnitCGrappleResult(CGrappleResult<Empty>);

#[no_mangle]
pub extern "C" fn free_error(err: CGrappleError) {
  if err.message.is_null() {
    return;
  }
  unsafe { drop(CString::from_raw(err.message)); }
}

#[repr(C)]
pub struct MaybeDoubleResult(COptional<CGrappleResult<f64>>);
#[repr(C)]
pub struct MaybeBoolResult(COptional<CGrappleResult<bool>>);

#[repr(C)]
pub enum COptional<T> {
  None,
  Some(T)
}

impl<T> From<Option<T>> for COptional<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(v) => COptional::Some(v),
      None => COptional::None,
    }
  }
}

pub trait JNIResultExtension<T> {
  fn with_jni_throw<'local, V, F: FnOnce(T) -> V>(self, env: &mut JNIEnv<'local>, exc: &str, f: F) -> Option<V>;
}

impl<'a, T> JNIResultExtension<T> for GrappleResult<'a, T> {
  fn with_jni_throw<'local, V, F: FnOnce(T) -> V>(self, env: &mut JNIEnv<'local>, exc: &str, f: F) -> Option<V> {
    match self {
      Ok(v) => Some(f(v)),
      Err(e) => {
        // let cls = env.find_class(&format!("au/grapplerobotics/{}", exc)).unwrap();
        let msg = env.new_string(e.to_string()).unwrap();
        let ex_obj: JThrowable = env
          .new_object(&format!("au/grapplerobotics/{}", exc), "(Ljava/lang/String;I)V", &[JValue::Object(&msg), JValue::Int(e.to_error_code() as i32)])
          .unwrap()
          .into();
        env.throw(ex_obj).unwrap();
        None
      },
    }
  }
}
