// &[*mut c_char]???
type NPP_NewProcPtr = fn (pluginType: NPMIMEType, instance: NPP, mode: uint16_t,
                          argc: int16_t, argn: *mut *mut c_char, argv: *mut *mut c_char,
                          saved: *mut NPSavedData) -> NPError;
type NPP_DestroyProcPtr = fn (instance: NPP, save: *mut *mut NPSavedData) -> NPError;
type NPP_SetWindowProcPtr = fn (instance: NPP, window: *mut NPWindow) -> NPError;
type NPP_NewStreamProcPtr = fn (instance: NPP, mime_type: NPMIMEType, stream: *mut NPStream,
                                seekable: NPBool, stype: *mut uint16_t) -> NPError;
type NPP_DestroyStreamProcPtr = fn (instance: NPP, stream: *mut NPStream, reason: NPReason) -> NPError;
type NPP_WriteReadyProcPtr = fn (instance: NPP, stream: *mut NPStream) -> int32_t;
type NPP_WriteProcPtr = fn (instance: NPP, stream: *mut NPStream, offset: int32_t, len: int32_t, buffer: *mut c_void) -> int32_t;
type NPP_StreamAsFileProcPtr = fn (instance: NPP, stream: *mut NPStream, fname: *const c_char);
type NPP_PrintProcPtr = fn (instance: NPP, platformPrint: *mut NPPrint);
type NPP_HandleEventProcPtr = fn (instance: NPP, event: *mut c_void) -> int16_t;
type NPP_URLNotifyProcPtr = fn (instance: NPP, url: *const c_char, reason: NPReason, notifyData: *mut c_void);
type NPP_GetValueProcPtr = fn (instance: NPP, variable: NPPVariable, ret_value: *mut c_void) -> NPError;
type NPP_SetValueProcPtr = fn (instance: NPP, variable: NPNVariable, value: *mut c_void) -> NPError;
type NPP_GotFocusPtr = fn (instance: NPP, direction: NPFocusDirection) -> NPBool;
type NPP_LostFocusPtr = fn (instance: NPP);
type NPP_URLRedirectNotifyPtr = fn (instance: NPP, url: *const c_char, status: int32_t, notifyData: *mut c_void);
type NPP_ClearSiteDataPtr = fn (site: *const c_char, flags: uint64_t, maxAge: uint64_t) -> NPError;
type NPP_GetSitesWithDataPtr = fn () -> *mut *mut c_char;
type NPP_DidCompositePtr = fn (instance: NPP);

pub struct NPPluginFuncs {
  size: uint16_t,
  version: uint16_t,
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

/*
typedef NPError      (* NPN_GetValueProcPtr)(NPP instance, NPNVariable variable, void *ret_value);
typedef NPError      (* NPN_SetValueProcPtr)(NPP instance, NPPVariable variable, void *value);
typedef NPError      (* NPN_GetURLNotifyProcPtr)(NPP instance, const char* url, const char* window, void* notifyData);
typedef NPError      (* NPN_PostURLNotifyProcPtr)(NPP instance, const char* url, const char* window, uint32_t len, const char* buf, NPBool file, void* notifyData);
typedef NPError      (* NPN_GetURLProcPtr)(NPP instance, const char* url, const char* window);
typedef NPError      (* NPN_PostURLProcPtr)(NPP instance, const char* url, const char* window, uint32_t len, const char* buf, NPBool file);
typedef NPError      (* NPN_RequestReadProcPtr)(NPStream* stream, NPByteRange* rangeList);
typedef NPError      (* NPN_NewStreamProcPtr)(NPP instance, NPMIMEType type, const char* window, NPStream** stream);
typedef int32_t      (* NPN_WriteProcPtr)(NPP instance, NPStream* stream, int32_t len, void* buffer);
typedef NPError      (* NPN_DestroyStreamProcPtr)(NPP instance, NPStream* stream, NPReason reason);
typedef void         (* NPN_StatusProcPtr)(NPP instance, const char* message);

// Browser manages the lifetime of the buffer returned by NPN_UserAgent,
// don't depend on it sticking around and don't free it.
typedef const char*  (* NPN_UserAgentProcPtr)(NPP instance);
typedef void*        (* NPN_MemAllocProcPtr)(uint32_t size);
typedef void         (* NPN_MemFreeProcPtr)(void* ptr);
typedef uint32_t     (* NPN_MemFlushProcPtr)(uint32_t size);
typedef void         (* NPN_ReloadPluginsProcPtr)(NPBool reloadPages);
typedef void*        (* NPN_GetJavaEnvProcPtr)(void);
typedef void*        (* NPN_GetJavaPeerProcPtr)(NPP instance);
typedef void         (* NPN_InvalidateRectProcPtr)(NPP instance, NPRect *rect);
typedef void         (* NPN_InvalidateRegionProcPtr)(NPP instance, NPRegion region);
typedef void         (* NPN_ForceRedrawProcPtr)(NPP instance);
typedef NPIdentifier (* NPN_GetStringIdentifierProcPtr)(const NPUTF8* name);
typedef void         (* NPN_GetStringIdentifiersProcPtr)(const NPUTF8** names, int32_t nameCount, NPIdentifier* identifiers);
typedef NPIdentifier (* NPN_GetIntIdentifierProcPtr)(int32_t intid);
typedef bool         (* NPN_IdentifierIsStringProcPtr)(NPIdentifier identifier);
typedef NPUTF8*      (* NPN_UTF8FromIdentifierProcPtr)(NPIdentifier identifier);
typedef int32_t      (* NPN_IntFromIdentifierProcPtr)(NPIdentifier identifier);
typedef NPObject*    (* NPN_CreateObjectProcPtr)(NPP npp, NPClass *aClass);
typedef NPObject*    (* NPN_RetainObjectProcPtr)(NPObject *obj);
typedef void         (* NPN_ReleaseObjectProcPtr)(NPObject *obj);
typedef bool         (* NPN_InvokeProcPtr)(NPP npp, NPObject* obj, NPIdentifier methodName, const NPVariant *args, uint32_t argCount, NPVariant *result);
typedef bool         (* NPN_InvokeDefaultProcPtr)(NPP npp, NPObject* obj, const NPVariant *args, uint32_t argCount, NPVariant *result);
typedef bool         (* NPN_EvaluateProcPtr)(NPP npp, NPObject *obj, NPString *script, NPVariant *result);
typedef bool         (* NPN_GetPropertyProcPtr)(NPP npp, NPObject *obj, NPIdentifier propertyName, NPVariant *result);
typedef bool         (* NPN_SetPropertyProcPtr)(NPP npp, NPObject *obj, NPIdentifier propertyName, const NPVariant *value);
typedef bool         (* NPN_RemovePropertyProcPtr)(NPP npp, NPObject *obj, NPIdentifier propertyName);
typedef bool         (* NPN_HasPropertyProcPtr)(NPP npp, NPObject *obj, NPIdentifier propertyName);
typedef bool         (* NPN_HasMethodProcPtr)(NPP npp, NPObject *obj, NPIdentifier propertyName);
typedef void         (* NPN_ReleaseVariantValueProcPtr)(NPVariant *variant);
typedef void         (* NPN_SetExceptionProcPtr)(NPObject *obj, const NPUTF8 *message);
typedef void         (* NPN_PushPopupsEnabledStateProcPtr)(NPP npp, NPBool enabled);
typedef void         (* NPN_PopPopupsEnabledStateProcPtr)(NPP npp);
typedef bool         (* NPN_EnumerateProcPtr)(NPP npp, NPObject *obj, NPIdentifier **identifier, uint32_t *count);
typedef void         (* NPN_PluginThreadAsyncCallProcPtr)(NPP instance, void (*func)(void *), void *userData);
typedef bool         (* NPN_ConstructProcPtr)(NPP npp, NPObject* obj, const NPVariant *args, uint32_t argCount, NPVariant *result);
typedef NPError      (* NPN_GetValueForURLPtr)(NPP npp, NPNURLVariable variable, const char *url, char **value, uint32_t *len);
typedef NPError      (* NPN_SetValueForURLPtr)(NPP npp, NPNURLVariable variable, const char *url, const char *value, uint32_t len);
typedef NPError      (* NPN_GetAuthenticationInfoPtr)(NPP npp, const char *protocol, const char *host, int32_t port, const char *scheme, const char *realm, char **username, uint32_t *ulen, char **password, uint32_t *plen);
typedef uint32_t     (* NPN_ScheduleTimerPtr)(NPP instance, uint32_t interval, NPBool repeat, void (*timerFunc)(NPP npp, uint32_t timerID));
typedef void         (* NPN_UnscheduleTimerPtr)(NPP instance, uint32_t timerID);
typedef NPError      (* NPN_PopUpContextMenuPtr)(NPP instance, NPMenu* menu);
typedef NPBool       (* NPN_ConvertPointPtr)(NPP instance, double sourceX, double sourceY, NPCoordinateSpace sourceSpace, double *destX, double *destY, NPCoordinateSpace destSpace);
typedef NPBool       (* NPN_HandleEventPtr)(NPP instance, void *event, NPBool handled);
typedef NPBool       (* NPN_UnfocusInstancePtr)(NPP instance, NPFocusDirection direction);
typedef void         (* NPN_URLRedirectResponsePtr)(NPP instance, void* notifyData, NPBool allow);
typedef NPError      (* NPN_InitAsyncSurfacePtr)(NPP instance, NPSize *size, NPImageFormat format, void *initData, NPAsyncSurface *surface);
typedef NPError      (* NPN_FinalizeAsyncSurfacePtr)(NPP instance, NPAsyncSurface *surface);
typedef void         (* NPN_SetCurrentAsyncSurfacePtr)(NPP instance, NPAsyncSurface *surface, NPRect *changed);
*/

pub struct NPNetscapeFuncs {
  size: uint16_t,
  version: uint16_t,
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