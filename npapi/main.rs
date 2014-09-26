// Copyright 2014 Karl Cheng, other contributors and third parties.
// Licensed under the GNU General Public License, version 3 or later.
// See the COPYRIGHT.md file at the top-level directory of this
// distribution, or go to http://www.gnu.org/licenses/.

#![crate_type="dylib"]
#![no_std]
#![feature(globs)]
#![allow(non_snake_case_functions)]

extern crate libc;
extern crate core;

use libc::c_char;
use core::intrinsics::transmute;
use core::prelude::*;

// necessary?
// pub use nostd::{stack_exhausted, eh_personality, begin_unwind};
use npapi::{NPError, NPPluginFuncs};

mod nostd;
mod npapi;

// do we need a 'static lifetime on this?
#[no_mangle]
#[cfg(unix)]
pub extern fn NP_GetMimeDescription() -> *const c_char {
  // returns string with following syntax: "<mime type>:<extension1>,<...extN>:<description>"
  // C strings must be null-terminated
  let mime = b"application/x.qantas94heavy-gamepad::Gamepad API Shim\0";
  
  unsafe {
    // is it &[c_char] or &'static [c_char]?
    let mime_as_i8: &[c_char] = transmute(mime);
    &mime_as_i8[0] as *const c_char
  }
}

// undocumented function, appears to run on Unix-like systems only
// bug in the NPAPI C headers: return type is supposed to be const char*, not char*
// see https://code.google.com/p/npapi-sdk/issues/detail?id=13
#[no_mangle]
#[cfg(unix)]
pub extern fn NP_GetPluginVersion() -> *mut c_char {
  // no documentation, but from the few examples I've seen "x.x.x" should be fine
  // C strings must be null-terminated
  let version = b"0.1.0\0";
  
  unsafe {
    let version_as_i8: &[i8] = transmute(version);
    &version_as_i8[0] as *const c_char
  }
}

// hard to find documentation about this function
// according to headers, runs on OSX and Windows
// apparently it's equivalent to the second argument to NP_Initialize
#[no_mangle]
#[cfg(win32)]
#[cfg(target_os="macos")]
pub extern "system" fn NP_GetEntryPoints(pFuncs: *mut NPPluginFuncs) -> NPError {}

fn init(bFuncs: *mut NPNetscapeFuncs) -> ??? {
  
}

#[no_mangle]
#[cfg(win32)]
#[cfg(target_os="macos")]
pub extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs) -> NPError {
  return init(bFuncs);
}

#[no_mangle]
#[cfg(not(win32,target_os="macos")]
pub extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs, pFuncs: *mut NPPluginFuncs) -> NPError {
  NP_GetEntryPoints(pFuncs);
  init(bFuncs);
}




  
// the second argument is not passed on OSX and Windows (that's for NP_GetEntryPoints instead)
#[no_mangle]
pub extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs, pFuncs_opt: Option<*mut NPPluginFuncs>) -> NPError {
  init(bFuncs);
  
}


#[no_mangle]
pub extern "system" fn NP_Shutdown() -> NPError {}

/*
The browser calls these Plug-in methods:

    NPP_GetValue: Query the plug-in for information.
    NPP_Print: Request a platform-specific print operation for the instance.
    NPP_SetValue: Set the browser information.
    NPP_SetWindow: Set the window in which a plug-in draws.
    NPP_HandleEvent: Deliver a platform-specific event to the instance.

The plug-in can call these Netscape methods to query and set information:

    NPN_GetValue: Get the browser information.
    NPN_SetValue: Set plug-in the browser information.
    
Plug-in developers can take advantage of the memory features provided in the Plug-in API to allocate and free memory.

    Use the NPN_MemAlloc method to allocate memory from the browser.
    Use the NPN_MemFree method to free memory allocated with NPN_MemAlloc.
    Use the NPN_MemFlush method to free memory (Mac OS only) before calling memory-intensive Mac Toolbox calls.

NPERR_NO_ERROR 	0 	No errors occurred
NPERR_GENERIC_ERROR 	1 	Error with no specific error code occurred
NPERR_INVALID_INSTANCE_ERROR 	2 	Invalid instance passed to the plug-in
NPERR_INVALID_FUNCTABLE_ERROR 	3 	Function table invalid
NPERR_MODULE_LOAD_FAILED_ERROR 	4 	Loading of plug-in failed
NPERR_OUT_OF_MEMORY_ERROR 	5 	Memory allocation failed
NPERR_INVALID_PLUGIN_ERROR 	6 	Plug-in missing or invalid
NPERR_INVALID_PLUGIN_DIR_ERROR 	7 	Plug-in directory missing or invalid
NPERR_INCOMPATIBLE_VERSION_ERROR 	8 	Versions of plug-in and Communicator do not match
NPERR_INVALID_PARAM 	9 	Parameter missing or invalid
NPERR_INVALID_URL 	10 	URL missing or invalid
NPERR_FILE_NOT_FOUND 	11 	File missing or invalid
NPERR_NO_DATA 	12 	Stream contains no data
NPERR_STREAM_NOT_SEEKABLE 	13 	Seekable stream expected
*/
