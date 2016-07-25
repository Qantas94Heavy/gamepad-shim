extern crate libc;
use libc::{c_char, c_void};
use npapi::{NPError, NPNetscapeFuncs};
mod npapi;

pub fn init(mut bFuncs: *mut NPNetscapeFuncs) -> NPError {
  unsafe {
    pFuncs.version       = (0 << 8) | 1;
    pFuncs.newp          = NPP_New;
    pFuncs.destroy       = NPP_Destroy;
    pFuncs.setwindow     = NPP_SetWindow;
    pFuncs.newstream     = NPP_NewStream;
    pFuncs.destroystream = NPP_DestroyStream;
    pFuncs.asfile        = NPP_StreamAsFile;
    pFuncs.writeready    = NPP_WriteReady;
    pFuncs.write         = NPP_Write;
    pFuncs.print         = NPP_Print;
    pFuncs.event         = NPP_HandleEvent;
    pFuncs.urlnotify     = NPP_URLNotify;
    pFuncs.getvalue      = NPP_GetValue;
    pFuncs.setvalue      = NPP_SetValue;
  }
  
  // TODO: implement this
  NPERR_GENERIC_ERROR
}

// TODO: is that a correct translation for char* argv[]?
fn NPP_New(pluginType: NPMIMEType, instance: NPP, _mode: u16, _argc: i16,
           _argn: *mut *mut char, _argv: *mut *mut c_char, saved: *mut NPSavedData) -> NPError {
  NPERR_GENERIC_ERROR
}

// we don't really care about the window
fn NPP_SetWindow() -> NPError {
  NPERR_NO_ERROR
}

fn NPP_Destroy(instance: NPP, _save: *mut *mut NPSavedData) -> NPError {
  NPERR_GENERIC_ERROR
}

fn NPP_GetValue(instance: *mut c_void, variable: NPPVariable, value: *mut c_void) -> NPError {
  match variable {
    NPPVariable:: => ,
    NPPVariable:: => ,
    
  NPERR_GENERIC_ERROR
}