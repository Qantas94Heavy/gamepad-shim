'use strict';

define(['gamepad'], function (Gamepad) {
  var supportsSetPrototypeOf = typeof Object.setPrototypeOf === 'function';
  var supportsProto = '__proto__' in {};
  var canChangePrototype = supportsSetPrototypeOf || supportsProto;
  
  // is a plain Event better or should we base it on CustomEvent instead?
  function GamepadEvent(type, eventInitDict) {
    if (!arguments.length) throw new TypeError("Failed to construct 'GamepadEvent': An event name must be provided.");
    
    var eventBase = new Event(type, eventInitDict);
    
    if (null != eventInitDict && 'gamepad' in eventInitDict) {
      if (DEBUG || eventInitDict.gamepad instanceof Gamepad) Object.defineProperty(eventBase, 'gamepad',
      { writable: false
      , enumerable: true
      , configurable: true
      , value: eventInitDict.gamepad
      });
      //else throw new TypeError("Failed to construct 'GamepadEvent': The 'gamepad' property does not have a Gamepad type.");
    }
    
    // not very important, all it does is make constructor the correct property and toString generate the correct value
    // jshint -W103
    if (supportsSetPrototypeOf) Object.setPrototypeOf(eventBase, GamepadEvent.prototype);
    else if (supportsProto) eventBase.__proto__ = GamepadEvent.prototype;
    else Object.defineProperties(eventBase,
    { constructor:
      { writable: true
      , enumerable: false
      , configurable: true
      , value: GamepadEvent
      }
    , toString:
      { writable: true
      , enumerable: false
      , configurable: true
      , value: toString
      }
    });
  }
  
  function toString() {
    if (this instanceof GamepadEvent) return '[object GamepadEvent]';
    return {}.toString.call(this);
  }
  
  var GamepadEventPrototype;
  
  // ensure instanceof works correctly in browsers which cannot change the prototype of an object
  if (canChangePrototype) {
    GamepadEventPrototype = Object.create(Event.prototype,
    { constructor:
      { writable: true
      , enumerable: false
      , configurable: true
      , value: GamepadEvent
      }
    });
  
    // check support of ES6 symbol tags
    // jshint notypeof:true
    /* global Symbol */
    if (typeof Symbol === 'function' && typeof Symbol.toStringTag === 'symbol') GamepadEventPrototype[Symbol.toStringTag] = 'GamepadEvent';
    // jshint notypeof:false
    else Object.defineProperty(GamepadEventPrototype, 'toString',
    { writable: true
    , enumerable: false
    , configurable: true
    , value: toString
    });
  }
  else GamepadEventPrototype = Event.prototype;
  
  Object.defineProperty(GamepadEvent, 'prototype',
  { writable: false
  , enumerable: false
  , configurable: false
  , value: GamepadEventPrototype
  });
  
  // let's just add our own dispatch event sugar here
  GamepadEvent._connect = function (gamepad, id, axes, buttons) {
    var newGamepad = Object.create(Gamepad.prototype);
  
    window.dispatchEvent(new GamepadEvent("gamepadconnected", { gamepad: gamepad }));
  }
  
  GamepadEvent._disconnect = function (gamepad, id, axes, buttons) {
    var newGamepad = Object.create(Gamepad.prototype);
  
    window.dispatchEvent(new GamepadEvent("gamepaddisconnected", { gamepad: gamepad }));
  }
});