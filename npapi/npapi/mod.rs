// Copyright 2014 Karl Cheng, other contributors and third parties.
// Licensed under the GNU General Public License, version 3 or later.
// See the COPYRIGHT.md file at the top-level directory of this
// distribution, or go to http://www.gnu.org/licenses/.

/* List of things that contian unions:
 * NPPrint
 * NPAsyncSurface
 * NPCocoaEvent
 * NPVariant
 */

#![allow(non_camel_case_types)]
#![feature(globs)]

extern crate libc;
extern crate core;

use libc::{c_char, c_uchar, c_void};
use core::prelude::*;
  
pub type NPBool = c_uchar;
pub type NPError = i16;
pub type NPReason = i16;
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
  end: u32,
  lastmodified: u32,
  notifyData: *mut c_void,
  
  // Response headers from host. Exists only for >= NPVERS_HAS_RESPONSE_HEADERS.
  // Used for HTTP only; NULL for non-HTTP. Available from NPP_NewStream onwards.
  // Plugin should copy this data before storing it. Includes HTTP status line and all headers,
  // preferably verbatim as received from server, headers formatted as in HTTP ("Header: Value"),
  // and newlines (\n, NOT \r\n) separating lines. Terminated by \n\0 (NOT \n\n\0).
  headers: *const c_char
}

struct NPSavedData {
  len: i32,
  buf: *mut c_void
}

struct NPRect {
  top: u16,
  left: u16,
  bottom: u16,
  right: u16
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
  x: i32, // position of top left corner relative to a Netscape page.
  y: i32,
  width: u32, // maximum window size
  height: u32,
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
  mode: u16, // NP_FULL or NP_EMBED
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

// gcc 3.x generated vtables on UNIX and OSX are incompatible with previous compilers.
// TODO: is this even necessary with Rust?
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


/*
NPError NPP_New(NPMIMEType pluginType, NPP instance,
                          uint16_t mode, int16_t argc, char* argn[],
                          char* argv[], NPSavedData* saved);
NPError NPP_Destroy(NPP instance, NPSavedData** save);
NPError NPP_SetWindow(NPP instance, NPWindow* window);
NPError NPP_NewStream(NPP instance, NPMIMEType type,
                                NPStream* stream, NPBool seekable,
                                uint16_t* stype);
NPError NPP_DestroyStream(NPP instance, NPStream* stream,
                                    NPReason reason);
int32_t NPP_WriteReady(NPP instance, NPStream* stream);
int32_t NPP_Write(NPP instance, NPStream* stream, int32_t offset,
                            int32_t len, void* buffer);
void    NPP_StreamAsFile(NPP instance, NPStream* stream,
                                   const char* fname);
void    NPP_Print(NPP instance, NPPrint* platformPrint);
int16_t NPP_HandleEvent(NPP instance, void* event);
void    NPP_URLNotify(NPP instance, const char* url,
                                NPReason reason, void* notifyData);
NPError NPP_GetValue(NPP instance, NPPVariable variable, void *value);
NPError NPP_SetValue(NPP instance, NPNVariable variable, void *value);
NPBool  NPP_GotFocus(NPP instance, NPFocusDirection direction);
void    NPP_LostFocus(NPP instance);
void    NPP_URLRedirectNotify(NPP instance, const char* url, int32_t status, void* notifyData);
NPError NPP_ClearSiteData(const char* site, uint64_t flags, uint64_t maxAge);
char**  NPP_GetSitesWithData(void);
void    NPP_DidComposite(NPP instance);

/* NPN_* functions are provided by the navigator and called by the plugin. */
void        NPN_Version(int* plugin_major, int* plugin_minor,
                                  int* netscape_major, int* netscape_minor);
NPError     NPN_GetURLNotify(NPP instance, const char* url,
                                       const char* target, void* notifyData);
NPError     NPN_GetURL(NPP instance, const char* url,
                                 const char* target);
NPError     NPN_PostURLNotify(NPP instance, const char* url,
                                        const char* target, uint32_t len,
                                        const char* buf, NPBool file,
                                        void* notifyData);
NPError     NPN_PostURL(NPP instance, const char* url,
                                  const char* target, uint32_t len,
                                  const char* buf, NPBool file);
NPError     NPN_RequestRead(NPStream* stream, NPByteRange* rangeList);
NPError     NPN_NewStream(NPP instance, NPMIMEType type,
                                    const char* target, NPStream** stream);
int32_t     NPN_Write(NPP instance, NPStream* stream, int32_t len,
                                void* buffer);
NPError     NPN_DestroyStream(NPP instance, NPStream* stream,
                                        NPReason reason);
void        NPN_Status(NPP instance, const char* message);
const char* NPN_UserAgent(NPP instance);
void*       NPN_MemAlloc(uint32_t size);
void        NPN_MemFree(void* ptr);
uint32_t    NPN_MemFlush(uint32_t size);
void        NPN_ReloadPlugins(NPBool reloadPages);
NPError     NPN_GetValue(NPP instance, NPNVariable variable,
                                   void *value);
NPError     NPN_SetValue(NPP instance, NPPVariable variable,
                                   void *value);
void        NPN_InvalidateRect(NPP instance, NPRect *invalidRect);
void        NPN_InvalidateRegion(NPP instance,
                                           NPRegion invalidRegion);
void        NPN_ForceRedraw(NPP instance);
void        NPN_PushPopupsEnabledState(NPP instance, NPBool enabled);
void        NPN_PopPopupsEnabledState(NPP instance);
void        NPN_PluginThreadAsyncCall(NPP instance,
                                                void (*func) (void *),
                                                void *userData);
NPError     NPN_GetValueForURL(NPP instance, NPNURLVariable variable,
                                         const char *url, char **value,
                                         uint32_t *len);
NPError     NPN_SetValueForURL(NPP instance, NPNURLVariable variable,
                                         const char *url, const char *value,
                                         uint32_t len);
NPError     NPN_GetAuthenticationInfo(NPP instance,
                                                const char *protocol,
                                                const char *host, int32_t port,
                                                const char *scheme,
                                                const char *realm,
                                                char **username, uint32_t *ulen,
                                                char **password,
                                                uint32_t *plen);
uint32_t    NPN_ScheduleTimer(NPP instance, uint32_t interval, NPBool repeat, void (*timerFunc)(NPP npp, uint32_t timerID));
void        NPN_UnscheduleTimer(NPP instance, uint32_t timerID);
NPError     NPN_PopUpContextMenu(NPP instance, NPMenu* menu);
NPBool      NPN_ConvertPoint(NPP instance, double sourceX, double sourceY, NPCoordinateSpace sourceSpace, double *destX, double *destY, NPCoordinateSpace destSpace);
NPBool      NPN_HandleEvent(NPP instance, void *event, NPBool handled);
NPBool      NPN_UnfocusInstance(NPP instance, NPFocusDirection direction);
void        NPN_URLRedirectResponse(NPP instance, void* notifyData, NPBool allow);
NPError     NPN_InitAsyncSurface(NPP instance, NPSize *size,
                                           NPImageFormat format, void *initData,
                                           NPAsyncSurface *surface);
NPError     NPN_FinalizeAsyncSurface(NPP instance, NPAsyncSurface *surface);
void        NPN_SetCurrentAsyncSurface(NPP instance, NPAsyncSurface *surface, NPRect *changed);
*/