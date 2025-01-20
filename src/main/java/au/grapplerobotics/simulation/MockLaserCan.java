package au.grapplerobotics.simulation;

import au.grapplerobotics.interfaces.LaserCanInterface;
import au.grapplerobotics.ConfigurationFailedException;

/**
 * Class for the Grapple Robotics LaserCAN sensor, as a mock sim object. The LaserCAN is a 0-4m laser ranging 
 * sensor addressable over the CAN bus. 
*/
public class MockLaserCan implements LaserCanInterface {
  private Measurement _measurement;

  /**
   * Create a new (mock) LaserCAN sensor. 
  */
  public MockLaserCan() {
    _measurement = new Measurement(0, 0, 0, false, TimingBudget.TIMING_BUDGET_20MS.asMilliseconds(), new RegionOfInterest(0, 0, 16, 16));
  }

  @Override
  public Measurement getMeasurement() {
    return _measurement;
  }

  @Override
  public void setRangingMode(RangingMode mode) throws ConfigurationFailedException {
    _measurement.is_long = mode == RangingMode.LONG;
  }

  @Override
  public void setTimingBudget(TimingBudget budget) throws ConfigurationFailedException {
    _measurement.budget_ms = budget.asMilliseconds();
  }

  @Override
  public void setRegionOfInterest(RegionOfInterest roi) throws ConfigurationFailedException {
    _measurement.roi = roi;
  }

  /**
   * Set the whole measurement class in simulation mode
   */
  public void setMeasurementFullSim(Measurement measurement) {
    _measurement = measurement;
  }

  /**
   * Set the measurement (only the measurement parameters, not including ranging mode, timing budget, or ROI - these
   * will be set automatically) in simulation mode
   */
  public void setMeasurementPartialSim(int status, int distance_mm, int ambient) {
    _measurement.status = status;
    _measurement.distance_mm = distance_mm;
    _measurement.ambient = ambient;
  }
}