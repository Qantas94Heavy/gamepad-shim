'use strict';

// loads and finds the appropriate Flash object
define(function () {
  var flash = document.createElement('object');
  flash.type = 'application/x-shockwave-flash';
  flash.data = 'gamepad-shim.swf';
  flash.id = 'gamepad-shim';
  
  var altText = document.createTextNode('It appears Adobe Flash Player is not enabled. Since your browser does not support the HTML5 Gamepad API, you must enable Adobe Flash Player to use your joystick.');
  flash.appendChild(flash);  
  
  // <object> tags cannot appear in head
  document.body.appendChild(flash);
  return flash;
});