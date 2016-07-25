// &[*mut c_char]???
type NPP_NewProcPtr = fn (pluginType: NPMIMEType, instance: NPP, mode: u16,
                          argc: i16, argn: *mut *mut c_char, argv: *mut *mut c_char,
                          saved: *mut NPSavedData) -> NPError;
type NPP_DestroyProcPtr = fn (instance: NPP, save: *mut *mut NPSavedData) -> NPError;
type NPP_SetWindowProcPtr = fn (instance: NPP, window: *mut NPWindow) -> NPError;
type NPP_NewStreamProcPtr = fn (instance: NPP, mime_type: NPMIMEType, stream: *mut NPStream,
                                seekable: NPBool, stype: *mut u16) -> NPError;
type NPP_DestroyStreamProcPtr = fn (instance: NPP, stream: *mut NPStream, reason: NPReason) -> NPError;
type NPP_WriteReadyProcPtr = fn (instance: NPP, stream: *mut NPStream) -> i32;
type NPP_WriteProcPtr = fn (instance: NPP, stream: *mut NPStream, offset: i32, len: i32, buffer: *mut c_void) -> i32;
type NPP_StreamAsFileProcPtr = fn (instance: NPP, stream: *mut NPStream, fname: *const c_char);
type NPP_PrintProcPtr = fn (instance: NPP, platformPrint: *mut NPPrint);
type NPP_HandleEventProcPtr = fn (instance: NPP, event: *mut c_void) -> i16;
type NPP_URLNotifyProcPtr = fn (instance: NPP, url: *const c_char, reason: NPReason, notifyData: *mut c_void);

// Any NPObjects returned to the browser via NPP_GetValue should be retained
// by the plugin on the way out. The browser is responsible for releasing.
type NPP_GetValueProcPtr = fn (instance: NPP, variable: NPPVariable, ret_value: *mut c_void) -> NPError;
type NPP_SetValueProcPtr = fn (instance: NPP, variable: NPNVariable, value: *mut c_void) -> NPError;
type NPP_GotFocusPtr = fn (instance: NPP, direction: NPFocusDirection) -> NPBool;
type NPP_LostFocusPtr = fn (instance: NPP);
type NPP_URLRedirectNotifyPtr = fn (instance: NPP, url: *const c_char, status: i32, notifyData: *mut c_void);
type NPP_ClearSiteDataPtr = fn (site: *const c_char, flags: u64, maxAge: u64) -> NPError;
type NPP_GetSitesWithDataPtr = fn () -> *mut *mut c_char;
type NPP_DidCompositePtr = fn (instance: NPP);

pub struct NPPluginFuncs {
  size: u16,
  version: u16,
  newp: NPP_NewProcPtr,
  destroy: NPP_DestroyProcPtr,
  setwindow: NPP_SetWindowProcPtr,
  newstream: NPP_NewStreamProcPtr,
  destroystream: NPP_DestroyStreamProcPtr,
  asfile: NPP_StreamAsFileProcPtr,
  writeready: NPP_WriteReadyProcPtr,
  write: NPP_WriteProcPtr,
  print: NPP_PrintProcPtr,
  event: NPP_HandleEventProcPtr,
  urlnotify: NPP_URLNotifyProcPtr,
  javaClass: *mut c_void,
  getvalue: NPP_GetValueProcPtr,
  setvalue: NPP_SetValueProcPtr,
  gotfocus: NPP_GotFocusPtr,
  lostfocus: NPP_LostFocusPtr,
  urlredirectnotify: NPP_URLRedirectNotifyPtr,
  clearsitedata: NPP_ClearSiteDataPtr,
  getsiteswithdata: NPP_GetSitesWithDataPtr,
  didComposite: NPP_DidCompositePtr
}

// _NPNetscapeFuncs

type NPN_GetValueProcPtr = fn (instance: NPP, variable: NPNVariable, ret_value: *mut c_void) -> NPError;
type NPN_SetValueProcPtr = fn (instance: NPP, variable: NPPVariable, value: *mut c_void) -> NPError;
type NPN_GetURLNotifyProcPtr = fn (instance: NPP, url: *const c_char, window: *const c_char, notifyData: *mut c_void) -> NPError;
type NPN_PostURLNotifyProcPtr = fn (instance: NPP, url: *const c_char, window: *const c_char, len: u32, buf: *const c_char, file: NPBool, notifyData: *mut c_void) -> NPError;
type NPN_GetURLProcPtr = fn (instance: NPP, url: *const c_char, window: *const c_char) -> NPError;
type NPN_PostURLProcPtr = fn (instance: NPP, url: *const c_char, window: *const c_char, len: u32, buf: *const c_char, file: NPBool) -> NPError;
type NPN_RequestReadProcPtr = fn (stream: *mut NPStream, rangeList: *mut NPByteRange) -> NPError;
type NPN_NewStreamProcPtr = fn (instance: NPP, NPMIMEType type, window: *const c_char, stream: *mut *mut NPStream) -> NPError;
type NPN_WriteProcPtr = fn (instance: NPP, NPStream* stream, len: i32, buffer: *mut c_void) -> i32;
type NPN_DestroyStreamProcPtr = fn (instance: NPP, NPStream* stream, reason: NPReason) -> NPError;
type NPN_StatusProcPtr = fn (instance: NPP, message: *const c_char);

// Browser manages the lifetime of the buffer returned by NPN_UserAgent,
// don't depend on it sticking around and don't free it.
type NPN_UserAgentProcPtr = fn (instance: NPP) -> *const c_char;
type NPN_MemAllocProcPtr = fn (size: u32) -> *mut c_void;
type NPN_MemFreeProcPtr = fn (ptr: *mut c_void);
type NPN_MemFlushProcPtr = fn (size: u32) -> u32;
type NPN_ReloadPluginsProcPtr = fn (reloadPages: NPBool);
type NPN_GetJavaEnvProcPtr = fn () -> *mut c_void;
type NPN_GetJavaPeerProcPtr = fn (instance: NPP) -> *mut c_void;
type NPN_InvalidateRectProcPtr = fn (instance: NPP, rect: *mut NPRect);
type NPN_InvalidateRegionProcPtr = fn (instance: NPP, region: NPRegion);
type NPN_ForceRedrawProcPtr = fn (instance: NPP);
type NPN_GetStringIdentifierProcPtr = fn (name: *const NPUTF8) -> NPIdentifier;
type NPN_GetStringIdentifiersProcPtr = fn (names: *const *const NPUTF8, nameCount: i32, identifiers: *mut NPIdentifier);
type NPN_GetIntIdentifierProcPtr = fn (intid: i32) -> NPIdentifier;
type NPN_IdentifierIsStringProcPtr = fn (identifier: NPIdentifier) -> c_bool;
type NPN_UTF8FromIdentifierProcPtr = fn (identifier: NPIdentifier) -> *mut NPUTF8;
type NPN_IntFromIdentifierProcPtr = fn (identifier: NPIdentifier) -> i32;
type NPN_CreateObjectProcPtr = fn (npp: NPP, aClass: *mut NPClass) -> *mut NPObject;
type NPN_RetainObjectProcPtr = fn (obj: *mut NPObject) -> *mut NPObject;
type NPN_ReleaseObjectProcPtr = fn (obj: *mut NPObject);

type NPN_InvokeProcPtr = fn (npp: NPP, obj: *mut NPObject, methodName: NPIdentifier, args: *const NPVariant, argCount: u32, result: *mut NPVariant) -> c_bool;
type NPN_InvokeDefaultProcPtr = fn (npp: NPP, obj: *mut NPObject, args: *const NPVariant, argCount: u32, result: *mut NPVariant) -> c_bool;
type NPN_EvaluateProcPtr = fn (npp: NPP, obj: *mut NPObject, NPString *script, result: *mut NPVariant) -> c_bool;
type NPN_GetPropertyProcPtr = fn (npp: NPP, obj: *mut NPObject, propertyName: NPIdentifier, result: *mut NPVariant) -> c_bool;
type NPN_SetPropertyProcPtr = fn (npp: NPP, obj: *mut NPObject, propertyName: NPIdentifier, value: *const NPVariant) -> c_bool;
type NPN_RemovePropertyProcPtr = fn (npp: NPP, obj: *mut NPObject, propertyName: NPIdentifier) -> c_bool;
type NPN_HasPropertyProcPtr = fn (npp: NPP, obj: *mut NPObject, propertyName: NPIdentifier) -> c_bool;
type NPN_HasMethodProcPtr = fn (npp: NPP, obj: *mut NPObject, propertyName: NPIdentifier) -> c_bool;
type NPN_ReleaseVariantValueProcPtr = fn (variant: *mut NPVariant);
type NPN_SetExceptionProcPtr = fn (obj: *mut NPObject, message: *const NPUTF8);
type NPN_PushPopupsEnabledStateProcPtr = fn (npp: NPP, enabled: NPBool);
type NPN_PopPopupsEnabledStateProcPtr = fn (npp: NPP);
type NPN_EnumerateProcPtr = fn (npp: NPP, obj: *mut NPObject, identifier: *mut *mut NPIdentifier, count: *mut u32) -> c_bool;
type NPN_PluginThreadAsyncCallProcPtr = fn (instance: NPP, func: fn (*mut c_void), userData: *mut c_void);
type NPN_ConstructProcPtr = fn (npp: NPP, obj: *mut NPObject, args: *const NPVariant, argCount: u32, result: *mut NPVariant) -> c_bool;
type NPN_GetValueForURLPtr = fn (npp: NPP, variable: NPNURLVariable, url: *const c_char, value: *mut *mut c_char, u32 *len) -> NPError;
type NPN_SetValueForURLPtr = fn (npp: NPP, variable: NPNURLVariable, url: *const c_char, value: *const c_char, len: u32) -> NPError;
type NPN_GetAuthenticationInfoPtr = fn (npp: NPP, protocol: *const c_char, host: *const c_char, port: i32, scheme: *const c_char, realm: *const c_char, username: *mut *mut c_char, ulen: *mut u32, password: *mut *mut c_char, plen: *mut u32) -> NPError;
type NPN_ScheduleTimerPtr = fn (instance: NPP, interval: u32, repeat: NPBool, timerFunc: fn (npp: NPP, timerID: u32)) -> u32;
type NPN_UnscheduleTimerPtr = fn (instance: NPP, timerID: u32);
type NPN_PopUpContextMenuPtr = fn (instance: NPP, NPMenu* menu) -> NPError;
type NPN_ConvertPointPtr = fn (instance: NPP, sourceX: c_double, sourceY: c_double, sourceSpace: NPCoordinateSpace, destX: *mut c_double, destY: *mut c_double, destSpace: NPCoordinateSpace) -> NPBool;
type NPN_HandleEventPtr = fn (instance: NPP, event: *mut c_void, handled: NPBool) -> NPBool;
type NPN_UnfocusInstancePtr = fn (instance: NPP, direction: NPFocusDirection) -> NPBool;
type NPN_URLRedirectResponsePtr = fn (instance: NPP, notifyData: *mut c_void, allow: NPBool);
type NPN_InitAsyncSurfacePtr = fn (instance: NPP, NPSize *size, format: NPImageFormat, initData: *mut c_void, surface: *mut NPAsyncSurface) -> NPError;
type NPN_FinalizeAsyncSurfacePtr = fn (instance: NPP, surface: *mut NPAsyncSurface) -> NPError;
type NPN_SetCurrentAsyncSurfacePtr = fn (instance: NPP, surface: *mut NPAsyncSurface, changed: *mut NPRect);

pub struct NPNetscapeFuncs {
  size: u16,
  version: u16,
  geturl: NPN_GetURLProcPtr,
  posturl: NPN_PostURLProcPtr,
  requestread: NPN_RequestReadProcPtr,
  newstream: NPN_NewStreamProcPtr,
  write: NPN_WriteProcPtr,
  destroystream: NPN_DestroyStreamProcPtr,
  status: NPN_StatusProcPtr,
  uagent: NPN_UserAgentProcPtr,
  memalloc: NPN_MemAllocProcPtr,
  memfree: NPN_MemFreeProcPtr,
  memflush: NPN_MemFlushProcPtr,
  reloadplugins: NPN_ReloadPluginsProcPtr,
  getJavaEnv: NPN_GetJavaEnvProcPtr,
  getJavaPeer: NPN_GetJavaPeerProcPtr,
  geturlnotify: NPN_GetURLNotifyProcPtr,
  posturlnotify: NPN_PostURLNotifyProcPtr,
  getvalue: NPN_GetValueProcPtr,
  setvalue: NPN_SetValueProcPtr,
  invalidaterect: NPN_InvalidateRectProcPtr,
  invalidateregion: NPN_InvalidateRegionProcPtr,
  forceredraw: NPN_ForceRedrawProcPtr,
  getstringidentifier: NPN_GetStringIdentifierProcPtr,
  getstringidentifiers: NPN_GetStringIdentifiersProcPtr,
  getintidentifier: NPN_GetIntIdentifierProcPtr,
  identifierisstring: NPN_IdentifierIsStringProcPtr,
  utf8fromidentifier: NPN_UTF8FromIdentifierProcPtr,
  intfromidentifier: NPN_IntFromIdentifierProcPtr,
  createobject: NPN_CreateObjectProcPtr,
  retainobject: NPN_RetainObjectProcPtr,
  releaseobject: NPN_ReleaseObjectProcPtr,
  invoke: NPN_InvokeProcPtr,
  invokeDefault: NPN_InvokeDefaultProcPtr,
  evaluate: NPN_EvaluateProcPtr,
  getproperty: NPN_GetPropertyProcPtr,
  setproperty: NPN_SetPropertyProcPtr,
  removeproperty: NPN_RemovePropertyProcPtr,
  hasproperty: NPN_HasPropertyProcPtr,
  hasmethod: NPN_HasMethodProcPtr,
  releasevariantvalue: NPN_ReleaseVariantValueProcPtr,
  setexception: NPN_SetExceptionProcPtr,
  pushpopupsenabledstate: NPN_PushPopupsEnabledStateProcPtr,
  poppopupsenabledstate: NPN_PopPopupsEnabledStateProcPtr,
  enumerate: NPN_EnumerateProcPtr,
  pluginthreadasynccall: NPN_PluginThreadAsyncCallProcPtr,
  construct: NPN_ConstructProcPtr,
  getvalueforurl: NPN_GetValueForURLPtr,
  setvalueforurl: NPN_SetValueForURLPtr,
  getauthenticationinfo: NPN_GetAuthenticationInfoPtr,
  scheduletimer: NPN_ScheduleTimerPtr,
  unscheduletimer: NPN_UnscheduleTimerPtr,
  popupcontextmenu: NPN_PopUpContextMenuPtr,
  convertpoint: NPN_ConvertPointPtr,
  handleevent: NPN_HandleEventPtr,
  unfocusinstance: NPN_UnfocusInstancePtr,
  urlredirectresponse: NPN_URLRedirectResponsePtr,
  initasyncsurface: NPN_InitAsyncSurfacePtr,
  finalizeasyncsurface: NPN_FinalizeAsyncSurfacePtr,
  setcurrentasyncsurface: NPN_SetCurrentAsyncSurfacePtr
}
