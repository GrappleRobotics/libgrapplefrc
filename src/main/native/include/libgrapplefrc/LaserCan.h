#pragma once

#include <stdint.h>
#include <memory>
#include "libgrapplefrcffi.h"

namespace libgrapplefrc {
  inline constexpr uint8_t LASERCAN_STATUS_VALID_MEASUREMENT = 0;
  inline constexpr uint8_t LASERCAN_STATUS_NOISE_ISSUE = 1;
  inline constexpr uint8_t LASERCAN_STATUS_WEAK_SIGNAL = 2;
  inline constexpr uint8_t LASERCAN_STATUS_OUT_OF_BOUNDS = 4;
  inline constexpr uint8_t LASERCAN_STATUS_WRAPAROUND = 7;

  using LaserCanMeasurement = ffi::LaserCanStatusFrame;
  using LaserCanROI = ffi::LaserCanRoi;

  enum class LaserCanRangingMode {
    Long,
    Short
  };

  enum class LaserCanTimingBudget {
    TimingBudget20ms = 20,
    TimingBudget33ms = 33,
    TimingBudget50ms = 50,
    TimingBudget100ms = 100,
  };

  class LaserCan {
  public:
    LaserCan(uint8_t can_id);
    ~LaserCan();

    LaserCanMeasurement get_measurement() const;
    int set_ranging_mode(LaserCanRangingMode mode);
    int set_timing_budget(LaserCanTimingBudget budget);
    int set_roi(LaserCanROI roi);

  private:
    uint8_t _can_id;
    ffi::LaserCanDevice *_handle;
  };
};