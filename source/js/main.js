'use strict';

if (typeof DEBUG === 'undefined') window.DEBUG = true;

require.config({ urlArgs: "_=" + Date.now() });

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
    if (objToString.call(this) !== '[object Navigator]') throw new TypeError('Illegal invocation');
    //function getGamepads() { [native code] }
    
    return flash.getGamepads(this);
  };
});