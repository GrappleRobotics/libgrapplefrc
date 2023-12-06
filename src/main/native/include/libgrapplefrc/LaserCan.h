#pragma once

#include <stdint.h>
#include <memory>
#include "libgrapplefrcffi.h"

namespace libgrapplefrc {
  using LaserCanStatus = ffi::LaserCanStatusFrame;
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

    LaserCanStatus status() const;
    int set_ranging_mode(LaserCanRangingMode mode);
    int set_timing_budget(LaserCanTimingBudget budget);
    int set_roi(LaserCanROI roi);

  private:
    uint8_t _can_id;
    ffi::LaserCanDevice *_handle;
  };
};