package au.grapplerobotics.simulation;

import au.grapplerobotics.interfaces.LaserCanInterface;
import au.grapplerobotics.ConfigurationFailedException;

/**
 * Class for the Grapple Robotics LaserCAN sensor, as a mock sim object. The LaserCAN is a 0-4m laser ranging 
 * sensor addressable over the CAN bus. 
*/
public class MockLaserCan implements LaserCanInterface {
  private Measurement _measurement;
  private RangingMode _rangingMode;
  private TimingBudget _timingBudget;
  private RegionOfInterest _roi;

  /**
   * Create a new (mock) LaserCAN sensor. 
  */
  public MockLaserCan() {
    _measurement = null;
    _rangingMode = RangingMode.SHORT;
    _timingBudget = TimingBudget.TIMING_BUDGET_20MS;
    _roi = new RegionOfInterest(0, 0, 16, 16);
  }

  @Override
  public Measurement getMeasurement() {
    return _measurement;
  }

  @Override
  public void setRangingMode(RangingMode mode) throws ConfigurationFailedException {
    _rangingMode = mode;
  }

  @Override
  public void setTimingBudget(TimingBudget budget) throws ConfigurationFailedException {
    _timingBudget = budget;
  }

  @Override
  public void setRegionOfInterest(RegionOfInterest roi) throws ConfigurationFailedException {
    _roi = roi;
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
    _measurement = new Measurement(status, distance_mm, ambient, _rangingMode == RangingMode.LONG, _timingBudget.asMilliseconds(), _roi);
  }
}