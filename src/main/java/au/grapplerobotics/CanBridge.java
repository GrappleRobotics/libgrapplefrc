package au.grapplerobotics;

public class CanBridge {
  public static void runTCP() {
    try {
      GrappleJNI.forceLoad();
      CanBridge.runTCPNow();
    } catch (UnsatisfiedLinkError e) {
      e.printStackTrace();
      System.exit(1);
    }
  }

  private static native void runTCPNow();
  public static native void runWebsocket(int port);
  public static native void runWebsocketInBackground(int port);
}