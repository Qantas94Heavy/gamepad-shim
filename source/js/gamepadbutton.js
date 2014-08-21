'use strict';

define(function () {
  // we don't actually do anything with the constructor itself
  function GamepadButton() {
    throw new TypeError('Illegal constructor');
  }
  
  // make sure property descriptor is in line with specification
  Object.defineProperty(GamepadButton, 'prototype',
  { writable: false
  , enumerable: false
  , configurable: false
  });
});

/*
interface GamepadButton {
    readonly    attribute boolean pressed;
    readonly    attribute double  value;
};
*/