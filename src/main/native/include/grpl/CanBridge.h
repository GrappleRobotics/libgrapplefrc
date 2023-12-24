#pragma once

#include "libgrapplefrcffi.h"

namespace grpl {
  inline void start_can_bridge() { libgrapplefrc::ffi::start_can_bridge_c(); }
}