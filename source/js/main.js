define(function () {
  // if you're dumb enough to overwrite this, please don't bother being a programmer
  try {
    null.e;
  } catch (e) {
    var TypeError = e.constructor;
  }
  
  // we only use these to 
  function Gamepad() {
    throw new TypeError('Illegal constructor.');
  }
  
  function GamepadButton() {
    throw new TypeError('Illegal constructor.');
  }
  
  function GamepadEvent() {
    throw new TypeError('Illegal constructor.');
  }
  
  if (typeof navigator.getGamepads !== 'function') {
    window.Gamepad = Gamepad;
    window.GamepadButton = GamepadButton;
    window.GamepadEvent = GamepadEvent;
  }
  
  var IEFlash = document.createElement('object');
  IEFlash.classid = 'clsid:d27cdb6e-ae6d-11cf-96b8-444553540000';
  IEFlash.id = 'gamepad-shim';
  
  var src = document.createElement('param');
  src.name = 'src';
  src.value = 'gamepad-shim.swf';
  IEFlash.appendChild(src);
  
  var altText = document.createTextNode("It appears Adobe Flash Player is not enabled. If your browser does not support the HTML5 Gamepad API, you'll need to enable Adobe Flash Player to use your joystick.");
  
  var otherFlash = document.createElement('object');
  otherFlash.type = 'application/x-shockwave-flash';
  otherFlash.data = 'gamepad-shim.swf';
  otherFlash.appendChild();
  IEFlash.appendChild(otherFlash);
  
  document.getElementsByTagName('head')[0].appendChild(IEFlash);
})();


/*
<object classid="clsid:d27cdb6e-ae6d-11cf-96b8-444553540000" id="gamepad-shim">
  <param name="src" value="untitled.swf">
  <!--[if !IE]>-->
  <object type="application/x-shockwave-flash" data="untitled.swf">
  <!--<![endif]-->
    
  <!--[if !IE]>-->
  </object>
  <!--<![endif]-->
</object>
*/