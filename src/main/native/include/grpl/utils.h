#pragma once

#include <optional>
#include <variant>

#include <frc/Errors.h>

#include "tl/expected.h"
#include "libgrapplefrcffi.h"

namespace grpl {
  using tl::expected;
  using tl::unexpected;

  using empty = libgrapplefrc::ffi::Empty;

  struct GrappleError {
    std::string error_message;
    uint8_t error_code;
  };

  static constexpr int GRAPPLE_ERROR_PARAM_OUT_OF_BOUNDS = 0x00;
  static constexpr int GRAPPLE_ERROR_FAILED_ASSERTION = 0x01;
  static constexpr int GRAPPLE_ERROR_TIMED_OUT = 0xFE;
  static constexpr int GRAPPLE_ERROR_GENERIC = 0xFF;

  template<typename T>
  constexpr std::optional<T> conv_opt(libgrapplefrc::ffi::COptional<T> opt) {
    if (opt.tag == libgrapplefrc::ffi::COptional<T>::Tag::Some) {
      return opt.some._0;
    } else {
      return std::nullopt;
    }
  }

  template<typename T>
  constexpr grpl::expected<T, GrappleError> conv_result(libgrapplefrc::ffi::CGrappleResult<T> opt) {
    if (opt.tag == libgrapplefrc::ffi::CGrappleResult<T>::Tag::Ok) {
      return opt.ok._0;
    } else {
      auto err = opt.err._0;
      FRC_ReportError(frc::err::Error, "Grapple Error: {}", err.message);
      GrappleError new_err{
        .error_message = std::string(err.message),
        .error_code = err.code
      };
      libgrapplefrc::ffi::free_error(err);
      return grpl::unexpected(new_err);
    }
  }
}