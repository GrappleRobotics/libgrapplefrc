package au.grapplerobotics;

import java.util.concurrent.atomic.AtomicBoolean;
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
        System.loadLibrary("grapplefrcdriver");
        // // RuntimeLoader.loadLibrary("grapplefrcdriver");
        // loader = new RuntimeLoader<>("grapplefrcdriver", RuntimeLoader.getDefaultExtractionRoot(), GrappleJNI.class);
        // loader.loadLibrary();
      } catch (UnsatisfiedLinkError ex) {
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
    // loader = new RuntimeLoader<>("grapplefrcdriver", RuntimeLoader.getDefaultExtractionRoot(), GrappleJNI.class);
    // loader.loadLibrary();
    System.loadLibrary("grapplefrcdriver");
    libraryLoaded = true;
  }
}
