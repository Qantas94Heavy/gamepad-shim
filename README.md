Gamepad API Shim
================

This standardises the use of the Gamepad API across browsers where possible using a Flash fallback.

Browser Support
---------------

 - Internet Explorer: 9 or higher
 - Chrome/Opera: latest and previous stable
 - Konqueror: 4.11 or higher
 - Firefox: latest and previous stable, ESR

Other browsers may work, but they will not be supported.

Known Issues
------------

### Specification Conformance

 - Calling `Object.prototype.toString` on a `Gamepad`, `GamepadButton` or `GamepadEvent` button will return an incorrect value
 - `new GamepadEvent().gamepad` is implemented as a data property on the instance itself, not a getter on `GamepadEvent.prototype.gamepad`.
 - `navigator.getGamepads` returns a different object every time
 - If `__proto__` and `Object.setPrototypeOf` are not supported:
 
    - `constructor` and `toString` will be own properties on the instance instead of the prototype
    - The prototype of `GamepadEvent` instances will be `Event.prototype` rather than a custom prototype object
    
### Other Issues

 - Y axes are flipped (i.e. -1 is up, 1 is down), not sure if there is a way to fix this
 - Flash IDs do not match those given by native Gamepad API implementations