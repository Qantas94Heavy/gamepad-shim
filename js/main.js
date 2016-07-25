'use strict';

if (typeof DEBUG === 'undefined') window.DEBUG = true;

require.config({
  urlArgs: DEBUG ? '_=' + Date.now() : ''
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
  
  function createButton(value) {
    var button = Object.create(GamepadButton.prototype);

    // same threshold value as Microsoft Xinput (XINPUT_GAMEPAD_TRIGGER_THRESHOLD = 30)
    button.pressed = value > 30 / 255;
    button.value = value;
    return button;
  }
  
  Navigator.prototype.getGamepads = function () {
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
      newGamepad.buttons = oldGamepad.buttons.map(createButton);
      newGamepad.connected = oldGamepad.connected;
      newGamepad.index = oldGamepad.index;
      newGamepad.mapping = oldGamepad.mapping;
      
      gamepads[i] = newGamepad;
    }
    
    return gamepads;
  };
});