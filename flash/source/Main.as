package {
  import flash.display.Sprite;
  import flash.external.ExternalInterface;
  import flash.events.Event;
  import flash.utils.Dictionary;
  import flash.utils.setInterval;
  
  import flash.ui.GameInput;
  import flash.ui.GameInputDevice;
  import flash.ui.GameInputControl;
  import flash.events.GameInputEvent;
  
  public class Main extends Sprite {
    private var gameInput:GameInput = new GameInput();
    
    private var gamepads:Array = [];
    
    private function getGamepads():Object {
      return gamepads.map(function (gamepad:Object, ..._):Object {
        if (gamepad === null) return null;
        return gamepad.w3c;
      });
    }
    
    private function onControlChange(event:Event):void {
      var control:GameInputControl = event.target as GameInputControl;
      
      for (var i:int = 0; i < gamepads.length; ++i) {
        if (gamepads[i].device === control.device) {
          var gamepadControl:Array = gamepads[i].flashToGamepadControl[control];
          gamepadControl[0][gamepadControl[1]] = control.value;
          break;
        }
      }
    }
    
    private function addDevice(device:GameInputDevice):void {
      device.enabled = true;
    
      // object reference should only change when one of the values change
      var axes:Array = [];
      var buttons:Array = [];
      
      // weak map
      var flashToGamepadControl:Dictionary = new Dictionary(true);
      
      for (var i:int = 0; i < device.numControls; ++i) {
        var control:GameInputControl = device.getControlAt(i);
        control.addEventListener(Event.CHANGE, onControlChange);
        
        var value:Number = control.value;
        var minValue:Number = control.minValue;
        var maxValue:Number = control.maxValue;
        
        if (/^AXIS/.test(control.id)) {          
          // normalise axis value to range [-1, 1] (is this necessary?)
          // TODO: find way of doing this which has better floating point accuracy
          if (minValue === -1 && maxValue === 1) axes.push(value);
          else axes.push((value - minValue) / (maxValue - minValue) * 2 - 1);
          
          flashToGamepadControl[control] = [axes, axes.length - 1];
        } else if (/^BUTTON/.test(control.id)) {
          // AFAIK all buttons are in range [0, 1]
          if (minValue !== 0 || maxValue !== 1) {
            throw new RangeError('GameInput button range not set to [0, 1]');
          }
          
          buttons.push(value);
          flashToGamepadControl[control] = [buttons, buttons.length - 1];
        }
        else throw new TypeError('Unexpected GameInput control type: ' + control.id);
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
      
      for (var i:int = 0; i < gamepads.length; ++i) {
        if (gamepads[i].device === event.device) {
          gamepads[i] = null;
          break;
        }
      }
    }
    
    public function Main():void {
      if (!GameInput.isSupported) {
        var error:String = 'Flash GameInput is not supported on this platform';
        if (ExternalInterface.available) ExternalInterface.call('eval', 'throw new TypeError("' + error + '")');
        throw new TypeError(error);
      }
      
      ExternalInterface.addCallback('getGamepads', getGamepads);
      
      flash.utils.setInterval(function ():void {
        ExternalInterface.call('console.log', GameInput.numDevices);
      }, 1000);
      
      gameInput.addEventListener(GameInputEvent.DEVICE_ADDED, onGamepadConnected);
      gameInput.addEventListener(GameInputEvent.DEVICE_REMOVED, onGamepadDisconnected);
      gameInput.addEventListener(GameInputEvent.DEVICE_UNUSABLE, function (event:GameInputEvent):void {
        ExternalInterface.call('console.warn', event);
      });
      
      for (var i:int = 0; i < GameInput.numDevices; ++i) {
        addDevice(GameInput.getDeviceAt(i));
      }
    }
  }
}