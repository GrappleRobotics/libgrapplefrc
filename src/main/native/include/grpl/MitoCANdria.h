#pragma once

#include <stdint.h>
#include <memory>
#include <optional>
#include "libgrapplefrcffi.h"
#include "grpl/utils.h"

namespace grpl {
  inline constexpr uint8_t MITOCANDRIA_CHANNEL_USB1 = 0;
  inline constexpr uint8_t MITOCANDRIA_CHANNEL_USB2 = 1;
  inline constexpr uint8_t MITOCANDRIA_CHANNEL_5VA = 2;
  inline constexpr uint8_t MITOCANDRIA_CHANNEL_5VB = 3;
  inline constexpr uint8_t MITOCANDRIA_CHANNEL_ADJ = 4;

  /**
   * Class for the Grapple Robotics MitoCANdria Voltage Regulator.
  */
  class MitoCANdria {
  public:
    /**
     * Create a new MitoCANdria.
     * 
     * \param can_id The CAN ID for the MitoCANdria. This ID is unique, and set in GrappleHook.
     *               Note: one ID should be mapped to only one device, or else they will conflict.
    */
    MitoCANdria(uint8_t can_id);
    ~MitoCANdria();

    std::optional<grpl::expected<double, GrappleError>> get_channel_current(uint8_t channel) const;
    std::optional<grpl::expected<bool, GrappleError>> get_channel_enabled(uint8_t channel) const;
    std::optional<grpl::expected<double, GrappleError>> get_channel_voltage(uint8_t channel) const;
    std::optional<grpl::expected<double, GrappleError>> get_channel_voltage_setpoint(uint8_t channel) const;

    grpl::expected<grpl::empty, GrappleError> set_channel_enabled(uint8_t channel, bool enabled);
    grpl::expected<grpl::empty, GrappleError> set_channel_voltage(uint8_t channel, double voltage);

  private:
    uint8_t _can_id;
    libgrapplefrc::ffi::MitoCANdria *_handle;
  };
}