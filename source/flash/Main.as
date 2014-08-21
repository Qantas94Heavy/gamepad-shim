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
    
      var axes:Array = [];
      var buttons:Array = [];
      
      for (var i:int = 0; i < device.numControls; ++i) {
        var control:GameInputControl = device.getControlAt(i);
        if (control.id.slice(0, 4) === 'AXIS') axes.push(control);
        else buttons.push(control);
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