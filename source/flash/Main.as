package {
  import flash.display.Sprite;
  import flash.external.ExternalInterface;
  import flash.events.Event;
  import flash.utils.Dictionary;
  
  import flash.ui.GameInput;
  import flash.ui.GameInputDevice;
  import flash.ui.GameInputControl;
  import flash.events.GameInputEvent;
  
  public class Main extends Sprite {
    private var gameInput:GameInput = new GameInput();
    
    private var gamepads:Array = [];
    private var emptyElements:Array = [];
    
    private function getGamepads():Object {
      return gamepads.map(function (gamepad:Object, ..._):Object {
        if (gamepad === null) return null;
        return gamepad.w3c;
      });
    }
    
    private function onControlChange(event:Event):void {
      var control:GameInputControl = event.target as GameInputControl;
      
      var i:int = 0;
      for (; i < gamepads.length; ++i) {
        if (gamepads[i].device === control.device) break;
      }
      
      var gamepadControl:Array = gamepads[i].flashToGamepadControl[control];
      ExternalInterface.call("console.log", gamepadControl);
      gamepadControl[0][gamepadControl[1]] = control.value;
    }
    
    private function addDevice(device:GameInputDevice):void {
      device.enabled = true;
    
      // object reference should only change when one of the values change
      var axes:Array = [];
      var buttons:Array = [];
      
      var flashToGamepadControl:Dictionary = new Dictionary(true);
      
      for (var i:int = 0; i < device.numControls; ++i) {
        var control:GameInputControl = device.getControlAt(i);
        control.addEventListener(Event.CHANGE, onControlChange);
        
        var value:Number = control.value;
        var minValue:Number = control.minValue;
        var maxValue:Number = control.maxValue;
        
        if (/^AXIS/.test(control.id)) {          
          // normalise axis value to range [-1, 1] (is this necessary?)
          if (minValue === -1 && maxValue === 1) axes.push(value);
          else axes.push((control.value - control.minValue) / (control.maxValue - control.minValue) * 2 - 1);
          
          flashToGamepadControl[control] = [axes, axes.length - 1];
        } else if (/^BUTTON/.test(control.id)) {
          var button:Object;
          
          // AFAIK, controls listed as button are not pressure sensitive (0/1)
          if (minValue !== 0 || maxValue !== 1) throw new RangeError('GameInput button range not set to [0, 1]');
          if (value === 1) button = { pressed: true, value: 1 };
          else if (value === 0) button = { pressed: false, value: 0 };
          else throw new RangeError('GameInput button value not 0 or 1');
          
          buttons.push(button);
          flashToGamepadControl[control] = [buttons, buttons.length - 1];
        } else {
          throw new TypeError('Unexpected GameInput control type: ' + control.id);
        }
      }
      
      var w3c:Object = { id: device.id, axes: axes, buttons: buttons, connected: device.enabled, index: gamepads.length, mapping: '' };
      
      ExternalInterface.call('GamepadEvent._connect', w3c);
      gamepads.push({ device: device, w3c: w3c, flashToGamepadControl: flashToGamepadControl });
    }
    
    private function onGamepadConnected(event:GameInputEvent):void {
      addDevice(event.device);
    }
    
    private function onGamepadDisconnected(event:GameInputEvent):void {
      ExternalInterface.call('GamepadEvent._disconnect', event.device);
      var i:int = 0;
      for (; i < gamepads.length; ++i) {
        if (gamepads[i].device === event.device) break;
      }
      
      gamepads[i] = null;
    }
    
    public function Main():void {
      if (!GameInput.isSupported || !ExternalInterface.available) throw new TypeError('Flash GameInput is not supported on this platform');
      ExternalInterface.addCallback('getGamepads', getGamepads);
      
      gameInput.addEventListener(GameInputEvent.DEVICE_ADDED, onGamepadConnected);
      gameInput.addEventListener(GameInputEvent.DEVICE_REMOVED, onGamepadDisconnected);
      
      for (var i:int = 0; i < GameInput.numDevices; ++i) {
        addDevice(GameInput.getDeviceAt(i));
      }
    }
  }
}