#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(rustdoc::broken_intra_doc_links)]

use std::{
    cell::RefCell,
    ffi::{c_char, c_int, CString},
    ptr,
};

use std::ffi::{c_char, CString};

use grapple_frc_msgs::grapple::errors::GrappleResult;
use jni::{JNIEnv, objects::{JThrowable, JValue}};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod calling;
pub mod can;
pub mod can_bridge;
pub mod lasercan;

// From https://michael-f-bryan.github.io/rust-ffi-guide/errors/return_types.html

thread_local! {
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
        }
    }
}

#[no_mangle]
pub extern "C" fn last_error() -> *mut c_char {
    match take_last_error() {
        Some(err) => {
            let str = CString::new(format!("{}", err)).unwrap();
            str.into_raw()
        }
        None => ptr::null_mut(),
    }
}

// Needed because otherwise the generated headers have templates :(
#[repr(C)]
pub struct UnitCGrappleResult(CGrappleResult<Empty>);

#[no_mangle]
pub extern "C" fn free_error(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(s));
    }
}

#[repr(C)]
pub enum COptional<T> {
    None,
    Some(T),
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
    fn with_jni_throw<'local, F: FnOnce(T)>(self, env: &mut JNIEnv<'local>, f: F);
}

impl<T> JNIResultExtension<T> for anyhow::Result<T> {
    fn with_jni_throw<'local, F: FnOnce(T)>(self, env: &mut JNIEnv<'local>, f: F) {
        match self {
            Ok(v) => f(v),
            Err(e) => {
                let cls = env
                    .find_class("au/grapplerobotics/NativeException")
                    .unwrap();
                env.throw_new(cls, format!("{}", e)).unwrap();
            }
        }
    }
}
