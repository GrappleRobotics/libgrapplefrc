#pragma once

#include <optional>

#include "libgrapplefrcffi.h"

namespace grpl {
  template<typename T>
  constexpr std::optional<T> conv_opt(libgrapplefrc::ffi::COptional<T> opt) {
    if (opt.tag == libgrapplefrc::ffi::COptional<T>::Tag::Some) {
      return opt.some._0;
    } else {
      return std::nullopt;
    }
  }

  int wrap_error(int code);
}