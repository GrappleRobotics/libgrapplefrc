#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]

use std::{cell::RefCell, ffi::{c_char, CString, c_int}, ptr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod calling;
pub mod lasercan;

// From https://michael-f-bryan.github.io/rust-ffi-guide/errors/return_types.html

thread_local!{
  static LAST_ERROR: RefCell<Option<anyhow::Error>> = RefCell::new(None);
}

pub fn update_last_error(err: anyhow::Error) {
  LAST_ERROR.with(|prev| {
      *prev.borrow_mut() = Some(err);
  });
}

/// Retrieve the most recent error, clearing it in the process.
pub fn take_last_error() -> Option<anyhow::Error> {
  LAST_ERROR.with(|prev| prev.borrow_mut().take())
}

pub fn with_err(result: anyhow::Result<()>) -> c_int {
  match result {
    Ok(()) => 0,
    Err(e) => {
      update_last_error(e);
      -1
    },
  }
}

#[no_mangle]
pub extern "C" fn last_error() -> *mut c_char {
  match take_last_error() {
    Some(err) => {
      let str = CString::new(format!("{}", err)).unwrap();
      str.into_raw()
    },
    None => ptr::null_mut()
  }
}

#[no_mangle]
pub extern "C" fn free_error(s: *mut c_char) {
  if s.is_null() {
    return;
  }
  unsafe { drop(CString::from_raw(s)); }
}