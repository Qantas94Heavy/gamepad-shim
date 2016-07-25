// Copyright 2014 Karl Cheng, other contributors and third parties.
// Licensed under the GNU General Public License, version 3 or later.
// See the COPYRIGHT.md file at the top-level directory of this
// distribution, or go to http://www.gnu.org/licenses/.

#![crate_type="dylib"]
#![feature(no_std, globs, lang_items)]
#![no_std]
#![allow(non_snake_case_functions)]

extern crate libc;
extern crate core;

use libc::c_char;
use core::mem::transmute;
use core::prelude::*;
use npapi::{NPError, NPPluginFuncs};

// `use` statments has to go before `mod` statements
mod nostd;
mod externs;

static NPERR_NO_ERROR: NPError = 0; // No errors occurred
static NPERR_GENERIC_ERROR: NPError = 1; // Error with no specific error code occurred
static NPERR_INVALID_INSTANCE_ERROR: NPError = 2; // Invalid instance passed to the plug-in
static NPERR_INVALID_FUNCTABLE_ERROR: NPError = 3; // Function table invalid
static NPERR_MODULE_LOAD_FAILED_ERROR: NPError = 4; // Loading of plug-in failed
static NPERR_OUT_OF_MEMORY_ERROR: NPError = 5; // Memory allocation failed
static NPERR_INVALID_PLUGIN_ERROR: NPError = 6; // Plug-in missing or invalid
static NPERR_INVALID_PLUGIN_DIR_ERROR: NPError = 7; // Plug-in directory missing or invalid
static NPERR_INCOMPATIBLE_VERSION_ERROR: NPError = 8; // Versions of plug-in and Communicator do not match
static NPERR_INVALID_PARAM: NPError = 9; // Parameter missing or invalid
static NPERR_INVALID_URL: NPError = 10; // URL missing or invalid
static NPERR_FILE_NOT_FOUND: NPError = 11; // File missing or invalid
static NPERR_NO_DATA: NPError = 12; // Stream contains no data
static NPERR_STREAM_NOT_SEEKABLE: NPError = 13; // Seekable stream expected

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
*/
