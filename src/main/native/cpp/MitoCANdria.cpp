#include "grpl/MitoCANdria.h"
#include "grpl/utils.h"

#include <iostream>

using namespace libgrapplefrc;
using namespace grpl;

MitoCANdria::MitoCANdria(uint8_t can_id) : _can_id(can_id) {
  _handle = ffi::mitocandria_new(can_id);
}

MitoCANdria::~MitoCANdria() {
  ffi::mitocandria_free(_handle);
}

std::optional<grpl::expected<double, GrappleError>> MitoCANdria::get_channel_current(uint8_t channel) const {
  auto v = ffi::mitocandria_get_channel_current(_handle, channel);
  auto opt = conv_opt(v._0);
  if (!opt.has_value()) {
    return std::nullopt;
  } else {
    return conv_result(opt.value());
  }
}

std::optional<grpl::expected<bool, GrappleError>> MitoCANdria::get_channel_enabled(uint8_t channel) const {
  auto v = ffi::mitocandria_get_channel_enabled(_handle, channel);
  auto opt = conv_opt(v._0);
  if (!opt.has_value()) {
    return std::nullopt;
  } else {
    return conv_result(opt.value());
  }
}

std::optional<grpl::expected<double, GrappleError>> MitoCANdria::get_channel_voltage(uint8_t channel) const {
  auto v = ffi::mitocandria_get_channel_voltage(_handle, channel);
  auto opt = conv_opt(v._0);
  if (!opt.has_value()) {
    return std::nullopt;
  } else {
    return conv_result(opt.value());
  }
}

std::optional<grpl::expected<double, GrappleError>> MitoCANdria::get_channel_voltage_setpoint(uint8_t channel) const {
  auto v = ffi::mitocandria_get_channel_voltage_setpoint(_handle, channel);
  auto opt = conv_opt(v._0);
  if (!opt.has_value()) {
    return std::nullopt;
  } else {
    return conv_result(opt.value());
  }
}

grpl::expected<grpl::empty, GrappleError> MitoCANdria::set_channel_enabled(uint8_t channel, bool enabled) {
  return conv_result(ffi::mitocandria_set_channel_enabled(_handle, channel, enabled)._0);
}

grpl::expected<grpl::empty, GrappleError> MitoCANdria::set_channel_voltage(uint8_t channel, double voltage) {
  return conv_result(ffi::mitocandria_set_channel_voltage(_handle, channel, voltage)._0);
}
