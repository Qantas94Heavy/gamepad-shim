gamepad-shim
============

This standardises the use of the Gamepad API across browsers where possible:

 - Vendor-specific prefixes
 - Flash fallback
 - ActiveX/NPAPI instllation by user

<object classid="clsid:D27CDB6E-AE6D-11cf-96B8-444553540000" id="gamepad-shim">
  <param name="src" value="untitled.swf">
  <!--[if !IE]>-->
  <object type="application/x-shockwave-flash" data="untitled.swf">
  <!--<![endif]-->
    It appears Adobe Flash Player is not enabled. If your browser does not support the HTML5 Gamepad API, you'll need to enable Adobe Flash Player to use your joystick.
  <!--[if !IE]>-->
  </object>
  <!--<![endif]-->
</object>