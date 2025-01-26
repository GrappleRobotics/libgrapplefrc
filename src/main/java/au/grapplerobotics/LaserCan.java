package au.grapplerobotics;

import java.lang.AutoCloseable;
import java.lang.ref.Cleaner;

import au.grapplerobotics.interfaces.LaserCanInterface;

/**
 * Class for the Grapple Robotics LaserCAN sensor. The LaserCAN is a 0-4m laser ranging 
 * sensor addressable over the CAN bus. 
*/
public class LaserCan implements AutoCloseable, LaserCanInterface {
  static native long init(int can_id);
  static native void free(long handle);

  static class Handle implements Runnable {
    long handle; 

    Handle(int can_id) {
      this.handle = init(can_id);
    }

    @Override
    public void run() {
      free(this.handle);
    }
  }

  private final Handle handle;
  private final Cleaner.Cleanable cleanable;

  /**
   * Create a new LaserCAN sensor. 
   * 
   * @param can_id The CAN ID for the LaserCAN sensor. This ID is unique, and set in GrappleHook.
   *               Note: one ID should be mapped to only one sensor, or else measurements will conflict.
  */
  public LaserCan(int can_id) {
    try {
      GrappleJNI.forceLoad();
    } catch (UnsatisfiedLinkError e) {
      e.printStackTrace();
      System.exit(1);
    }
    
    this.handle = new Handle(can_id);
    this.cleanable = GrappleJNI.cleaner.register(this, this.handle);
  }

  native Measurement getMeasurementInternal();

  @Override
  public Measurement getMeasurement() {
    return getMeasurementInternal();
  }

  @Override
  public void setRangingMode(RangingMode mode) throws ConfigurationFailedException {
    setRangingMode(mode == RangingMode.LONG);
  }

  @Override
  public void setTimingBudget(TimingBudget budget) throws ConfigurationFailedException {
    switch (budget) {
      case TIMING_BUDGET_20MS:
        setTimingBudget(20);
        break;
      case TIMING_BUDGET_33MS:
        setTimingBudget(33);
        break;
      case TIMING_BUDGET_50MS:
        setTimingBudget(50);
        break;
      case TIMING_BUDGET_100MS:
        setTimingBudget(100);
        break;
    }
  }

  @Override
  public void setRegionOfInterest(RegionOfInterest roi) throws ConfigurationFailedException {
    setRoi(roi.x, roi.y, roi.w, roi.h);
  }

  native void setRangingMode(boolean is_long) throws ConfigurationFailedException;
  native void setTimingBudget(int budget) throws ConfigurationFailedException;
  native void setRoi(int x, int y, int w, int h) throws ConfigurationFailedException;

  @Override
  public void close() throws Exception {
    cleanable.clean();
  }
}