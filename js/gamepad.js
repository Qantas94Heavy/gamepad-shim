'use strict';

define(function () {
  // we don't actually do anything with the constructor itself
  function Gamepad() {
    throw new TypeError('Illegal constructor');
  }
  
  // make sure property descriptor is in line with specification
  Object.defineProperty(Gamepad, 'prototype',
  { writable: false
  , enumerable: false
  , configurable: false
  });
  
  return Gamepad;
});

/*
interface Gamepad {
    readonly    attribute double[]            axes;
    readonly    attribute GamepadButton[]     buttons;
    readonly    attribute boolean             connected;
    readonly    attribute DOMString           id;
    readonly    attribute long                index;
    readonly    attribute GamepadMappingType  mapping;
    readonly    attribute DOMHighResTimeStamp timestamp;
};

enum GamepadMappingType {
  "": no mapping
  "standard": mapped to standard gamepad (basically PS/2 compatible controller)
}
 */