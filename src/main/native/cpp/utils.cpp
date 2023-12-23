#include "grpl/utils.h"
#include "libgrapplefrcffi.h"

#include <frc/Errors.h>

int grpl::wrap_error(int code) {
  if (code != 0) {
    char *last_error = libgrapplefrc::ffi::last_error();
    FRC_ReportError(frc::err::Error, "LaserCAN: {}", last_error);
    libgrapplefrc::ffi::free_error(last_error);
  }
  return code;
}