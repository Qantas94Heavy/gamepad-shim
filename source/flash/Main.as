package {
  import flash.display.Sprite;
  import flash.external.ExternalInterface;
  
  import flash.ui.GameInput;
  import flash.ui.GameInputDevice;
  import flash.ui.GameInputControl;
  import flash.events.GameInputEvent;
  
  public class Main extends Sprite {
    private var gameInput:GameInput = new GameInput();
    
    private var gamepads:Array = [];
    private var emptyElements:Array = [];
    
    private function getGamepads():Array {
      return gamepads;
    }
    
    private function addDevice(device:GameInputDevice):void {
      device.enabled = true;
    
      // object reference only changes when one of the values are supposed to change
      var axes:Array = [];
      var buttons:Array = [];
      
      for (var i:int = 0; i < device.numControls; ++i) {
        var control:GameInputControl = device.getControlAt(i);
        
        var a:GameInputControlName = new GameInputControlName();
        a.
        
        var value:Number = control.value;
        var minValue:Number = control.minValue;
        var maxValue:Number = control.maxValue;
        
        if (/^AXIS/.test(control.id)) {          
          // normalise axis value to range [-1, 1] (is this necessary?)
          if (minValue === -1 && maxValue === 1) axes.push(value);
          else axes.push((control.value - control.minValue) / (control.maxValue - control.minValue) * 2 - 1);
        } else if (/^BUTTON/.test(control.id)) {
          var button:Object;
          
          // AFAIK, controls listed as button are not pressure sensitive (0/1)
          if (minValue !== 0 || maxValue !== 1) throw new RangeError('GameInput button range not set to [0, 1]');
          if (value === 1) button = { pressed: true, value: 1 };
          else if (value === 0) button = { pressed: false, value: 0 };
          else throw new RangeError('GameInput button value not 0 or 1');
          
          buttons.push(button);
        } else {
          throw new TypeError('Unexpected GameInput control type: ' + control.id);
        }
      }
      
      var gamepad:Object = { id: device.id, axes: axes, buttons: buttons, connected: device.enabled, index: gamepads.length, mapping: '' };
      
      ExternalInterface.call('GamepadEvent._connect', gamepad);
      gamepads.push(gamepad);
    }
    
    private function onGamepadConnected(event:GameInputEvent):void {
      addDevice(event.device);
    }
    
    private function onGamepadDisconnected(event:GameInputEvent):void {
      ExternalInterface.call('GamepadEvent._disconnect', event.device);
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