#pragma once

#include "libgrapplefrcffi.h"

namespace grpl {
  inline void start_can_bridge() { libgrapplefrc::ffi::start_can_bridge_c(); }
  inline void start_ws_can_bridge(uint32_t port = 0) { libgrapplefrc::ffi::run_ws_can_bridge_c(port); }
  inline void start_ws_can_bridge_in_background(uint32_t port = 0) { libgrapplefrc::ffi::run_ws_can_bridge_in_background_c(port); }
}