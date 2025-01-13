package au.grapplerobotics;

public class CanBridge {
  public static native void runTCP();
  public static native void runWebsocket(int port);
  public static native void runWebsocketInBackground(int port);
}