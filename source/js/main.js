'use strict';

if (typeof DEBUG === 'undefined') window.DEBUG = true;

require.config({
  urlArgs: DEBUG ? '_=' + Date.now() : '',
  baseUrl: '/gamepad-shim/source/js'
});

define(['flash', 'gamepad', 'gamepadbutton', 'gamepadevent'], function (flash, Gamepad, GamepadButton, GamepadEvent) {
  // we don't actually do anything with these
  if (DEBUG || typeof window.Gamepad !== 'function') Object.defineProperty(window, 'Gamepad',
  { configurable: true
  , enumerable: false
  , writable: true
  , value: Gamepad
  });
   
  if (DEBUG || typeof window.GamepadButton !== 'function') Object.defineProperty(window, 'GamepadButton',
  { configurable: true
  , enumerable: false
  , writable: true
  , value: GamepadButton
  });
  
  if (DEBUG || typeof window.GamepadEvent !== 'function') Object.defineProperty(window, 'GamepadEvent',
  { configurable: true
  , enumerable: false
  , writable: true
  , value: GamepadEvent
  });
  
  var objToString = {}.toString;
  
  Navigator.prototype.getGamepads = function getGamepads() {
    //function getGamepads() { [native code] }
    if (objToString.call(this) !== '[object Navigator]') throw new TypeError('Illegal invocation');
    
    var gamepads = flash.getGamepads();
    
    for (var i = 0; i < gamepads.length; ++i) {
      var oldGamepad = gamepads[i];
      if (oldGamepad === null) {
        delete gamepads[i];
        continue;
      }
      
      var newGamepad = Object.create(Gamepad.prototype);
      newGamepad.id = oldGamepad.id;
      newGamepad.axes = oldGamepad.axes;
      newGamepad.buttons = oldGamepad.buttons;
      newGamepad.connected = oldGamepad.connected;
      newGamepad.index = oldGamepad.index;
      newGamepad.mapping = oldGamepad.mapping;
      
      gamepads[i] = newGamepad;
    }
    
    return gamepads;
  };
});