package au.grapplerobotics;

import java.io.IOException;
import java.lang.AutoCloseable;
import java.lang.ref.Cleaner;

public class LaserCan implements AutoCloseable {
  public static final int LASERCAN_STATUS_VALID_MEASUREMENT = 0;
  public static final int LASERCAN_STATUS_NOISE_ISSUE = 1;
  public static final int LASERCAN_STATUS_WEAK_SIGNAL = 2;
  public static final int LASERCAN_STATUS_OUT_OF_BOUNDS = 4;
  public static final int LASERCAN_STATUS_WRAPAROUND = 7;

  public static class RegionOfInterest {
    public int x, y;
    public int w, h;

    public RegionOfInterest(int x, int y, int w, int h) {
      this.x = x;
      this.y = y;
      this.w = w;
      this.h = h;
    }
  }
  
  public static class Measurement {
    public int status;
    public int distance_mm;
    public int ambient;
    public boolean is_long;
    public int budget_ms;
    public RegionOfInterest roi;

    public Measurement(int status, int distance_mm, int ambient, boolean is_long, int budget_ms, RegionOfInterest roi) {
      this.status = status;
      this.distance_mm = distance_mm;
      this.ambient = ambient;
      this.is_long = is_long;
      this.budget_ms = budget_ms;
      this.roi = roi;
    }
  }

  public static enum RangingMode {
    LONG,
    SHORT
  }

  public static enum TimingBudget {
    TIMING_BUDGET_20MS,
    TIMING_BUDGET_33MS,
    TIMING_BUDGET_50MS,
    TIMING_BUDGET_100MS,
  }

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

  public LaserCan(int can_id) {
    try {
      GrappleJNI.forceLoad();
    } catch (IOException e) {
      e.printStackTrace();
      System.exit(1);
    }
    
    this.handle = new Handle(can_id);
    this.cleanable = GrappleJNI.cleaner.register(this, this.handle);
  }

  public native Measurement getMeasurement();

  public void setRangingMode(RangingMode mode) {
    setRangingMode(mode == RangingMode.LONG);
  }

  public void setTimingBudget(TimingBudget budget) {
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

  public void setRegionOfInterest(RegionOfInterest roi) {
    setRoi(roi.x, roi.y, roi.w, roi.h);
  }

  native void setRangingMode(boolean is_long);
  native void setTimingBudget(int budget);
  native void setRoi(int x, int y, int w, int h);

  @Override
  public void close() throws Exception {
    cleanable.clean();
  }
}