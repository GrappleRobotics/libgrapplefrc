package au.grapplerobotics;

import edu.wpi.first.util.RuntimeLoader;
import java.util.concurrent.atomic.AtomicBoolean;
import java.io.IOException;
import java.lang.ref.Cleaner;

public class GrappleJNI {
  public static final Cleaner cleaner = Cleaner.create();

  static boolean libraryLoaded = false;

  public static class Helper {
    private static AtomicBoolean extractOnStaticLoad = new AtomicBoolean(true);

    public static boolean getExtractOnStaticLoad() {
      return extractOnStaticLoad.get();
    }

    public static void setExtractOnStaticLoad(boolean load) {
      extractOnStaticLoad.set(load);
    }
  }

  static {
    if (Helper.getExtractOnStaticLoad()) {
      try {
        RuntimeLoader.loadLibrary("grapplefrcdriver");
      } catch (IOException ex) {
        ex.printStackTrace();
        System.exit(1);
      }
      libraryLoaded = true;
    }
  }

  /**
   * Force load the library.
   * @throws java.lang.UnsatisfiedLinkError thrown if the native library cannot be found
   */
  public static synchronized void forceLoad() throws UnsatisfiedLinkError {
    if (libraryLoaded) {
      return;
    }
    System.loadLibrary("grapplefrcdriver");
    libraryLoaded = true;
  }
}
