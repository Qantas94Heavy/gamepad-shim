package {
  import flash.display.Sprite;
  import flash.external.ExternalInterface;
  import flash.ui.GameInput;
  import flash.ui.GameInputDevice;
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
      var controls:Array = [];
      for (var i:int = 0; i < device.numControls; ++i) {
        controls.push(device.getControlAt(i));
      }
      ExternalInterface.call('GamepadEvent._connect', device, controls);
      gamepads.push([device, controls]);
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