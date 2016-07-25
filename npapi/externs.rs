use start::*;
mod start;

macro_rules! to_c_str {
  ($str:expr) => ( $str.as_ptr() as *const c_char );
}

#[no_mangle]
#[cfg(unix)]
pub extern fn NP_GetMimeDescription() -> *const c_char {
  // returns string with following syntax: "<mime type>:<extension1>,<...extN>:<description>"
  // C strings must be null-terminated
  to_c_str!(b"application/x.qantas94heavy-gamepad::HTML5 Gamepad API Shim\0")
}

// undocumented function, appears to run on Unix-like systems only
// bug in the NPAPI C headers: return type is supposed to be const char*, not char*
// see https://code.google.com/p/npapi-sdk/issues/detail?id=13
#[no_mangle]
#[cfg(unix)]
pub extern fn NP_GetPluginVersion() -> *const c_char {
  // no documentation, but from the few examples I've seen "x.x.x" should be fine
  // C strings must be null-terminated
   to_c_str!(b"0.1.0\0")
}

// hard to find documentation about this function
// according to headers, runs on OSX and Windows
// apparently it's equivalent to the second argument to NP_Initialize
#[no_mangle]
#[cfg(any(win32, target_os="macos"))]
extern "system" fn NP_GetEntryPoints(pFuncs: *mut NPPluginFuncs) -> NPError {
  // TODO: implement this
  NPERR_GENERIC_ERROR
}

// TODO: can these functions be merged in any way?
#[no_mangle]
#[cfg(any(win32, target_os="macos"))]
extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs) -> NPError {
  init(bFuncs)
}

#[no_mangle]
#[cfg(not(win32,target_os="macos")]
extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs, pFuncs: *mut NPPluginFuncs) -> NPError {
  NP_GetEntryPoints(pFuncs);
  init(bFuncs)
}
  
// the second argument is not passed on OSX and Windows (that's for NP_GetEntryPoints instead)
#[no_mangle]
extern "system" fn NP_Initialize(bFuncs: *mut NPNetscapeFuncs, pFuncs_opt: Option<*mut NPPluginFuncs>) -> NPError {
  match pFuncs_opt {
    Some(pFuncs) => NP_GetEntryPoints(pFuncs),
    None => {}
  }
  
  init(bFuncs)
}

#[no_mangle]
extern "system" fn NP_Shutdown() -> NPError {}