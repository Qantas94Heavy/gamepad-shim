package {
  import flash.display.Sprite;
  import flash.external.ExternalInterface;
  import flash.utils.setInterval;
  
  import flash.ui.GameInput;
  
  public class Main extends Sprite {    
    public function Main():void {
      flash.utils.setInterval(function ():void {
        ExternalInterface.call('console.log', GameInput.numDevices);
      }, 1000);
    }
  }
}