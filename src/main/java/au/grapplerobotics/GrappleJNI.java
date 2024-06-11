package au.grapplerobotics;

import java.io.IOException;
import java.util.concurrent.atomic.AtomicBoolean;
import java.lang.ref.Cleaner;

import edu.wpi.first.util.RuntimeLoader;

public class GrappleJNI {
  public static final Cleaner cleaner = Cleaner.create();

  static boolean libraryLoaded = false;
  // static RuntimeLoader<GrappleJNI> loader = null;

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
        // loader = new RuntimeLoader<>("grapplefrcdriver", RuntimeLoader.getDefaultExtractionRoot(), GrappleJNI.class);
        // loader.loadLibrary();
      } catch (IOException ex) {
        ex.printStackTrace();
        System.exit(1);
      }
      libraryLoaded = true;
    }
  }

  /**
   * Force load the library.
   * @throws java.io.IOException thrown if the native library cannot be found
   */
  public static synchronized void forceLoad() throws IOException {
    if (libraryLoaded) {
      return;
    }
    loader = new RuntimeLoader<>("grapplefrcdriver", RuntimeLoader.getDefaultExtractionRoot(), GrappleJNI.class);
    loader.loadLibrary();
    libraryLoaded = true;
  }
}
