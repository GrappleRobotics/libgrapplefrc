#include "libgrapplefrc/LaserCan.h"
#include "libgrapplefrcffi.h"

#include <frc/Errors.h>

#include <iostream>

using namespace libgrapplefrc;
using namespace grpl;

int wrap_error(int code) {
  if (code != 0) {
    char *last_error = ffi::last_error();
    FRC_ReportError(frc::err::Error, "LaserCAN: {}", last_error);
    ffi::free_error(last_error);
  }
  return code;
}

LaserCan::LaserCan(uint8_t can_id) : _can_id(can_id) {
  _handle = ffi::lasercan_new(can_id);
}

LaserCan::~LaserCan() {
  ffi::lasercan_free(_handle);
}

std::optional<LaserCanMeasurement> LaserCan::get_measurement() const {
  LaserCanMeasurement status = ffi::lasercan_get_status(_handle);
  if (status.status != 0xFF) {
    return status;
  } else {
    return std::nullopt;
  }
}

int LaserCan::set_ranging_mode(LaserCanRangingMode mode) {
  return wrap_error(ffi::lasercan_set_range(_handle, mode == LaserCanRangingMode::Long));
}

int LaserCan::set_timing_budget(LaserCanTimingBudget budget) {
  return wrap_error(ffi::lasercan_set_timing_budget(_handle, (uint8_t)budget));
}

int LaserCan::set_roi(LaserCanROI roi) {
  return wrap_error(ffi::lasercan_set_roi(_handle, roi));
}
