package au.grapplerobotics.interfaces;

import au.grapplerobotics.ConfigurationFailedException;

public interface LaserCanInterface {
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
    TIMING_BUDGET_100MS;

    public int asMilliseconds() {
      switch (this) {
        case TIMING_BUDGET_20MS:
          return 20;
        case TIMING_BUDGET_33MS:
          return 33;
        case TIMING_BUDGET_50MS:
          return 50;
        default:
          return 100;
      }
    }
  }

  /**
   * Get the most recent measurement from the sensor, if available.
   * May return null.
  */
  public Measurement getMeasurement();
  
  /**
   * Set the ranging mode for the sensor.
   * @see RangingMode
  */
  public void setRangingMode(RangingMode mode) throws ConfigurationFailedException;

  /**
   * Set the timing budget for the sensor.
   * @see TimingBudget
  */
  public void setTimingBudget(TimingBudget budget) throws ConfigurationFailedException;

  /**
   * Set the region of interest for the sensor.
   * @see RegionOfInterest
   */
  public void setRegionOfInterest(RegionOfInterest roi) throws ConfigurationFailedException;
}