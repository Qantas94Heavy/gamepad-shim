// Copyright 2014 Karl Cheng, other contributors and third parties.
// Licensed under the GNU General Public License, version 3 or later.
// See the COPYRIGHT.md file at the top-level directory of this
// distribution, or go to http://www.gnu.org/licenses/.

#![allow(non_camel_case_types)]

extern crate libc;
use libc::{int16_t, uint16_t, int32_t, uint32_t, uint64_t, c_char, c_uchar, c_void};
  
pub type NPBool = c_uchar;
pub type NPError = int16_t;
pub type NPReason = int16_t;
pub type NPMIMEType = *mut c_char;

struct NPP_t {
  pdata: *mut c_void,
  ndata: *mut c_void
}
type NPP = *mut NPP_t;

struct NPStream {
  pdata: *mut c_void, // plug-in private data
  ndata: *mut c_void, // Netscape private data
  url: *const c_char,
  end: uint32_t,
  lastmodified: uint32_t,
  notifyData: *mut c_void,
  
  // Response headers from host. Exists only for >= NPVERS_HAS_RESPONSE_HEADERS.
  // Used for HTTP only; NULL for non-HTTP. Available from NPP_NewStream onwards.
  // Plugin should copy this data before storing it. Includes HTTP status line and all headers,
  // preferably verbatim as received from server, headers formatted as in HTTP ("Header: Value"),
  // and newlines (\n, NOT \r\n) separating lines. Terminated by \n\0 (NOT \n\n\0).
  headers: *const c_char 
}

struct NPSavedData {
  len: int32_t,
  buf: *mut c_void
}

struct NPRect {
  top: uint16_t,
  left: uint16_t,
  bottom: uint16_t,
  right: uint16_t
}

enum NPFocusDirection {
  NPFocusNext = 0,
  NPFocusPrevious = 1
}

/*
 * The type of a NPWindow - it specifies the type of the data structure
 * returned in the window field.
 */
enum NPWindowType {
  NPWindowTypeWindow = 1,
  NPWindowTypeDrawable
}

struct NPWindow {
  window: *mut c_void, // platform-specific window handle
  x: int32_t, // position of top left corner relative to a Netscape page.
  y: int32_t,
  width: uint32_t, // maximum window size
  height: uint32_t,
  clipRect: NPRect, // clipping rectangle in port coordinates
  #[cfg(target_os="linux")] #[cfg(target_os="freebsd")]
  ws_info: *mut c_void, // platform-dependent additional data (Unix-only)
  window_type: NPWindowType // is this a window or a drawable?
}

// no union type, but we'll use these two structs for sizeof
struct NPFullPrint {
  pluginPrinted: NPBool, // Set TRUE if plugin handled fullscreen printing
  printOne: NPBool, // TRUE if plugin should print one copy to default printer
  platformPrint: *mut c_void // Platform-specific printing info
}

struct NPEmbedPrint {
  window: NPWindow,
  platformPrint: *mut c_void // Platform-specific printing info
}

// FIXME: In theory, this is the same thing as a union of structs. In practice, they aren't
// represented using the same memory layout as in C, which is a PITA.
/*
enum Print {
  NPFullPrint { // if mode is NP_FULL
    pluginPrinted: NPBool, // Set TRUE if plugin handled fullscreen printing
    printOne: NPBool, // TRUE if plugin should print one copy to default printer
    platformPrint: *mut c_void // Platform-specific printing info
  },
  NPEmbedPrint { // if mode is NP_EMBED
    window: NPWindow,
    platformPrint: *mut c_void // Platform-specific printing info
  }
}
*/

struct NPPrint {
  mode: uint16_t, // NP_FULL or NP_EMBED
  print: Print
}

/* The following masks are applied on certain platforms to NPNV and
 * NPPV selectors that pass around pointers to COM interfaces. Newer
 * compilers on some platforms may generate vtables that are not
 * compatible with older compilers. To prevent older plugins from
 * not understanding a new browser's ABI, these masks change the
 * values of those selectors on those platforms. To remain backwards
 * compatible with different versions of the browser, plugins can
 * use these masks to dynamically determine and use the correct C++
 * ABI that the browser is expecting. This does not apply to Windows
 * as Microsoft's COM ABI will likely not change.
 */

/* gcc 3.x generated vtables on UNIX and OSX are incompatible with
 * previous compilers.
 */
#[cfg(unix)]
static _NP_ABI_MIXIN_FOR_GCC3: int = 0x10000000;
#[cfg(not(unix))]
static _NP_ABI_MIXIN_FOR_GCC3: int = 0;

#[cfg(target_os="macos")]
static _NP_ABI_MIXIN_FOR_MACHO: int = 0x01000000;
#[cfg(not(target_os="macos"))]
static _NP_ABI_MIXIN_FOR_MACHO: int = 0;

static NP_ABI_MASK: int = _NP_ABI_MIXIN_FOR_GCC3 | _NP_ABI_MIXIN_FOR_MACHO;

/*
 * List of variable names for which NPP_GetValue shall be implemented
 */
enum NPPVariable {
  NPPVpluginNameString = 1,
  NPPVpluginDescriptionString,
  NPPVpluginWindowBool,
  NPPVpluginTransparentBool,
  NPPVjavaClass,
  NPPVpluginWindowSize,
  NPPVpluginTimerInterval,
  NPPVpluginScriptableInstance = (10 | NP_ABI_MASK),
  NPPVpluginScriptableIID = 11,
  NPPVjavascriptPushCallerBool = 12,
  NPPVpluginKeepLibraryInMemory = 13,
  NPPVpluginNeedsXEmbed = 14,

  // Get the NPObject for scripting the plugin. Introduced in NPAPI minor version 14.
  NPPVpluginScriptableNPObject = 15,

  // Get the plugin value (as \0-terminated UTF-8 string data) for
  // form submission if the plugin is part of a form. Use
  // NPN_MemAlloc() to allocate memory for the string data. Introduced
  // in NPAPI minor version 15.
  NPPVformValue = 16,

  NPPVpluginUrlRequestsDisplayedBool = 17,

  // Checks if the plugin is interested in receiving the http body of
  // all http requests (including failed ones, http status != 200).
  NPPVpluginWantsAllNetworkStreams = 18,

  // Browsers can retrieve a native ATK accessibility plug ID via this variable.
  NPPVpluginNativeAccessibleAtkPlugId = 19,

  // Checks to see if the plug-in would like the browser to load the "src" attribute.
  NPPVpluginCancelSrcStream = 20,

  NPPVsupportsAdvancedKeyHandling = 21,

  NPPVpluginUsesDOMForCursorBool = 22,

  // Used for negotiating drawing models
  NPPVpluginDrawingModel = 1000,

  // Used for negotiating event models
  #[cfg(target_os="macos")]
  NPPVpluginEventModel = 1001,
  
  // In the NPDrawingModelCoreAnimation drawing model, the browser asks the plug-in for a Core Animation layer.
  #[cfg(target_os="macos")]
  NPPVpluginCoreAnimationLayer = 1003
}

/*
 * List of variable names for which NPN_GetValue should be implemented.
 */
enum NPNVariable {
  NPNVxDisplay = 1,
  NPNVxtAppContext,
  NPNVnetscapeWindow,
  NPNVjavascriptEnabledBool,
  NPNVasdEnabledBool,
  NPNVisOfflineBool,

  NPNVserviceManager = (10 | NP_ABI_MASK),
  NPNVDOMElement = (11 | NP_ABI_MASK),
  NPNVDOMWindow = (12 | NP_ABI_MASK),
  NPNVToolkit = (13 | NP_ABI_MASK),
  NPNVSupportsXEmbedBool = 14,

  // Get the NPObject wrapper for the browser window.
  NPNVWindowNPObject = 15,

  // Get the NPObject wrapper for the plugins DOM element.
  NPNVPluginElementNPObject = 16,

  NPNVSupportsWindowless = 17,

  NPNVprivateModeBool = 18,

  NPNVsupportsAdvancedKeyHandling = 21,

  NPNVdocumentOrigin = 22,

  // Get the current drawing model (NPDrawingModel)
  NPNVpluginDrawingModel = 1000,
  
  #[cfg(target_os="macos")]
  NPNVcontentsScaleFactor = 1001,
  
  #[cfg(target_os="macos", target_pointer_size="32")]
  NPNVsupportsQuickDrawBool = 2000,
  
  #[cfg(target_os="macos")]
  NPNVsupportsCoreGraphicsBool = 2001,
  
  #[cfg(target_os="macos")]
  NPNVsupportsOpenGLBool = 2002,
  
  #[cfg(target_os="macos")]
  NPNVsupportsCoreAnimationBool = 2003,
  
  #[cfg(target_os="macos")]
  NPNVsupportsInvalidatingCoreAnimationBool = 2004,
  
  NPNVsupportsAsyncBitmapSurfaceBool = 2007,
  
  #[cfg(win32)]
  NPNVsupportsAsyncWindowsDXGISurfaceBool = 2008,

  // TRUE if the browser supports the Carbon event model
  #[cfg(target_os="macos", target_pointer_size="32")]
  NPNVsupportsCarbonBool = 3000,
  
  // TRUE if the browser supports the Cocoa event model
  #[cfg(target_os="macos")]
  NPNVsupportsCocoaBool = 3001,
  
  // TRUE if the browser supports the updated Cocoa text input specification.
  #[cfg(target_os="macos")]
  NPNVsupportsUpdatedCocoaTextInputBool = 3002,
  
  // TRUE if the browser supports CA model compositing
  #[cfg(target_os="macos")]
  NPNVsupportsCompositingCoreAnimationPluginsBool = 74656
}