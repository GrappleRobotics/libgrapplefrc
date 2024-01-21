package au.grapplerobotics;

import java.io.IOException;
import java.lang.AutoCloseable;
import java.lang.ref.Cleaner;

/**
 * Class for the Grapple Robotics LaserCAN sensor. The LaserCAN is a 0-4m laser ranging 
 * sensor addressable over the CAN bus. 
*/
public class LaserCan implements AutoCloseable {
  /**
   * Status marker for a valid measurement
  */
  public static final int LASERCAN_STATUS_VALID_MEASUREMENT = 0;

  /**
   * Status marker for a measurement that has a noise issue. This usually means
   * that the signal is obtained in a high-noise environment. Increasing the
   * timing budget may increase the reliability of this measurement.
  */
  public static final int LASERCAN_STATUS_NOISE_ISSUE = 1;

  /**
   * Status marker for a measurement that is too weak. This usually means
   * the target is too far away, not reflective enough, or too small. 
   * Try adjusting your ROI or Timing Budget, or accept the less accurate
   * measurement.
  */
  public static final int LASERCAN_STATUS_WEAK_SIGNAL = 2;

  /**
   * Status marker for a measurement that is out of bounds. This usually means
   * the sensor has detected an object on the limits of its range. This usually
   * only applies to bright targets.
  */
  public static final int LASERCAN_STATUS_OUT_OF_BOUNDS = 4;

  /**
   * Status marker for a measurement that has 'wrapped around'. For highly reflective
   * targets, this means the target is out of the theoretical range of the sensor, but 
   * still detected. The distance value hence 'wraps around', reading a smaller distance.
  */
  public static final int LASERCAN_STATUS_WRAPAROUND = 7;

  /**
   * A Region of Interest for the LaserCAN sensor. The Region of Interest is the target area
   * on which the sensor will detect objects. GrappleHook can be used to interactively set the
   * Region of Interest.
  */
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
  
  /**
   * A Measurement obtained from a LaserCAN Sensor.
  */
  public static class Measurement {
    /**
     * The measurement status.
     * @see LASERCAN_STATUS_VALID_MEASUREMENT
     * @see LASERCAN_STATUS_NOISE_ISSUE
     * @see LASERCAN_STATUS_WEAK_SIGNAL
     * @see LASERCAN_STATUS_OUT_OF_BOUNDS
     * @see LASERCAN_STATUS_WRAPAROUND
     */
    public int status;

    /**
     * The distance to the target, in millimeters.
     */
    public int distance_mm;

    /**
     * The approximate ambient light level.
     */
    public int ambient;

    /**
     * Was this measurement taken with the "long" distance mode?
     */
    public boolean is_long;

    /**
     * The timing budget the measurement was taken with.
     */
    public int budget_ms;

    /**
     * The region of interest the measurement was taken with.
     */
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

  /**
   * The Ranging Mode for the LaserCAN sensor.
  */
  public static enum RangingMode {
    /**
     * The Long Ranging Mode can be used to identify targets at longer distances
     * than the short ranging mode (up to 4m), but is more susceptible to ambient
     * light.
    */
    LONG,

    /**
     * The Short Ranging Mode is used to detect targets at 1.3m and lower. Although 
     * shorter than the Long ranging mode, this mode is less susceptible to ambient
     * light.
    */
    SHORT
  }

  /**
   * The Timing Budget for the LaserCAN Sensor. Higher timing budgets provide more accurate
   * and repeatable results, however at a lower rate than smaller timing budgets.
  */
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

  /**
   * Create a new LaserCAN sensor. 
   * 
   * @param can_id The CAN ID for the LaserCAN sensor. This ID is unique, and set in GrappleHook.
   *               Note: one ID should be mapped to only one sensor, or else measurements will conflict.
  */
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

  /**
   * Get the most recent measurement from the sensor, if available.
   * May return null.
  */
  public native Measurement getMeasurement();

  /**
   * Set the ranging mode for the sensor.
   * @see RangingMode
  */
  public void setRangingMode(RangingMode mode) throws ConfigurationFailedException {
    setRangingMode(mode == RangingMode.LONG);
  }

  /**
   * Set the timing budget for the sensor.
   * @see TimingBudget
  */
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

  /**
   * Set the region of interest for the sensor.
   * @see RegionOfInterest
   */
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