'use strict';

define(['gamepad'], function (Gamepad) {
  var supportsSetPrototypeOf = typeof Object.setPrototypeOf === 'function';
  var supportsProto = '__proto__' in {};
  var canChangePrototype = supportsSetPrototypeOf || supportsProto;
  
  // is a plain Event better, or should we base it on CustomEvent instead?
  function GamepadEvent(type, eventInitDict) {
    // TODO: add more robust check that it is indeed called using the new operator
    if (!(this instanceof GamepadEvent)) throw new TypeError("Failed to construct 'GamepadEvent': Please use the 'new' operator, this DOM object constructor cannot be called as a function.");
    
    if (!arguments.length) throw new TypeError("Failed to construct 'GamepadEvent': An event name must be provided.");
    
    var eventBase = new Event(type, eventInitDict);
    if (null != eventInitDict && 'gamepad' in eventInitDict) {
      if (eventInitDict.gamepad instanceof Gamepad) Object.defineProperty(eventBase, 'gamepad',
      { writable: false
      , enumerable: true
      , configurable: true
      , value: eventInitDict.gamepad
      });
      else throw new TypeError("Failed to construct 'GamepadEvent': The 'gamepad' property does not have a Gamepad type.");
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
    
    return eventBase;
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
  
  // Internal stuff, please don't use this in application code
  GamepadEvent._connect = function (options) {
    console.log(1);
    
    var gamepad = Object.create(Gamepad.prototype);
    gamepad.id = options.id;
    gamepad.axes = options.axes;
    gamepad.buttons = options.buttons;
    gamepad.connected = options.connected;
    gamepad.index = options.index;
    gamepad.mapping = options.mapping;
  
    window.dispatchEvent(new GamepadEvent('gamepadconnected', { gamepad: gamepad }));
  };
  
  GamepadEvent._disconnect = function (options) {
    console.log(2);
    
    var gamepad = Object.create(Gamepad.prototype);
    gamepad.id = options.id;
    gamepad.axes = options.axes;
    gamepad.buttons = options.buttons;
    gamepad.connected = options.connected;
    gamepad.index = options.index;
    gamepad.mapping = options.mapping;
  
    window.dispatchEvent(new GamepadEvent('gamepaddisconnected', { gamepad: gamepad }));
  };
  
  return GamepadEvent;
});