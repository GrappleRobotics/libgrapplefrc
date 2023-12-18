#pragma once

#include <stdint.h>
#include <memory>
#include <optional>
#include "libgrapplefrcffi.h"

namespace grpl {
  /**
   * Status marker for a valid measurement
  */
  inline constexpr uint8_t LASERCAN_STATUS_VALID_MEASUREMENT = 0;

  /**
   * Status marker for a measurement that has a noise issue. This usually means
   * that the signal is obtained in a high-noise environment. Increasing the
   * timing budget may increase the reliability of this measurement.
  */
  inline constexpr uint8_t LASERCAN_STATUS_NOISE_ISSUE = 1;

  /**
   * Status marker for a measurement that is too weak. This usually means
   * the target is too far away, not reflective enough, or too small. 
   * Try adjusting your ROI or Timing Budget, or accept the less accurate
   * measurement.
  */
  inline constexpr uint8_t LASERCAN_STATUS_WEAK_SIGNAL = 2;

  /**
   * Status marker for a measurement that is out of bounds. This usually means
   * the sensor has detected an object on the limits of its range. This usually
   * only applies to bright targets.
  */
  inline constexpr uint8_t LASERCAN_STATUS_OUT_OF_BOUNDS = 4;

  /**
   * Status marker for a measurement that has 'wrapped around'. For highly reflective
   * targets, this means the target is out of the theoretical range of the sensor, but 
   * still detected. The distance value hence 'wraps around', reading a smaller distance.
  */
  inline constexpr uint8_t LASERCAN_STATUS_WRAPAROUND = 7;

  /**
   * A Measurement obtained from a LaserCAN Sensor.
  */
  using LaserCanMeasurement = libgrapplefrc::ffi::LaserCanStatusFrame;

  /**
   * A Region of Interest for the LaserCAN sensor. The Region of Interest is the target area
   * on which the sensor will detect objects. GrappleHook can be used to interactively set the
   * Region of Interest.
  */
  using LaserCanROI = libgrapplefrc::ffi::LaserCanRoi;

  /**
   * The Ranging Mode for the LaserCAN sensor.
  */
  enum class LaserCanRangingMode {
    /**
     * The Long Ranging Mode can be used to identify targets at longer distances
     * than the short ranging mode (up to 4m), but is more susceptible to ambient
     * light.
    */
    Long,
    /**
     * The Short Ranging Mode is used to detect targets at 1.3m and lower. Although 
     * shorter than the Long ranging mode, this mode is less susceptible to ambient
     * light.
    */
    Short
  };

  /**
   * The Timing Budget for the LaserCAN Sensor. Higher timing budgets provide more accurate
   * and repeatable results, however at a lower rate than smaller timing budgets.
  */
  enum class LaserCanTimingBudget {
    TimingBudget20ms = 20,
    TimingBudget33ms = 33,
    TimingBudget50ms = 50,
    TimingBudget100ms = 100,
  };

  /**
   * Class for the Grapple Robotics LaserCAN sensor. The LaserCAN is a 0-4m laser ranging 
   * sensor addressable over the CAN bus. 
  */
  class LaserCan {
  public:
    /**
     * Create a new LaserCAN sensor. 
     * 
     * \param can_id The CAN ID for the LaserCAN sensor. This ID is unique, and set in GrappleHook.
     *               Note: one ID should be mapped to only one sensor, or else measurements will conflict.
    */
    LaserCan(uint8_t can_id);
    ~LaserCan();

    /**
     * Get the most recent measurement from the sensor, if available.
    */
    std::optional<LaserCanMeasurement> get_measurement() const;

    /**
     * Set the ranging mode for the sensor. \see libgrapplefrc::LaserCanRangingMode
    */
    int set_ranging_mode(LaserCanRangingMode mode);

    /**
     * Set the timing budget for the sensor. \see libgrapplefrc::LaserCanTimingBudget
    */
    int set_timing_budget(LaserCanTimingBudget budget);

    /**
     * Set the region of interest for the sensor. \see libgrapplefrc::LaserCanROI
    */
    int set_roi(LaserCanROI roi);

  private:
    uint8_t _can_id;
    libgrapplefrc::ffi::LaserCanDevice *_handle;
  };
}