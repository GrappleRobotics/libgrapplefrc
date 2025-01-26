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
   * Base class for the MitoCANdria
   */
  class MitoCANdriaInterface {
   public:
    /**
     * Get the current consumption of a channel, in Amperes, at this point in time.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return std::optional::nullopt if the MitoCANdria is not yet available on the bus.
     * Will return an error if the channel is out of bounds.
    */
    virtual std::optional<grpl::expected<double, GrappleError>> get_channel_current(uint8_t channel) const = 0;

    /**
     * Get the enabled state of a channel at this point in time.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return std::optional::nullopt if the MitoCANdria is not yet available on the bus.
     * Will return an error if the channel is out of bounds.
    */
    virtual std::optional<grpl::expected<bool, GrappleError>> get_channel_enabled(uint8_t channel) const = 0;

    /**
     * Get the voltage of a channel, in Volts, at this point in time.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return std::optional::nullopt if the MitoCANdria is not yet available on the bus.
     * Will return an error if the channel is out of bounds.
    */
    virtual std::optional<grpl::expected<double, GrappleError>> get_channel_voltage(uint8_t channel) const = 0;

    /**
     * Get the voltage setpoint of a channel, in Volts, at this point in time.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return std::optional::nullopt if the MitoCANdria is not yet available on the bus.
     * Will return an error if the channel is out of bounds.
    */
    virtual std::optional<grpl::expected<double, GrappleError>> get_channel_voltage_setpoint(uint8_t channel) const = 0;

    /**
     * Set the enabled state of a channel, where true is energised and false is deenergised.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return an error if the channel is out of bounds, or the MitoCANdria could not be configured.
     */
    virtual grpl::expected<grpl::empty, GrappleError> set_channel_enabled(uint8_t channel, bool enabled) = 0;

    /**
     * Set the voltage of a channel. This will also disable the channel as a safety precaution, requiring
     * set_channel_enabled to be called.
     * Note: only the adjustable channel can be targetted by this method.
     * Channel must be one of grpl::MITOCANDRIA_CHANNEL_*.
     * Will return an error if the channel is out of bounds, or the MitoCANdria could not be configured.
     */
    virtual grpl::expected<grpl::empty, GrappleError> set_channel_voltage(uint8_t channel, double voltage) = 0;
  };

  /**
   * Class for the Grapple Robotics MitoCANdria Voltage Regulator.
  */
  class MitoCANdria : public MitoCANdriaInterface {
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

  /**
   * Class for the Grapple Robotics MitoCANdria Voltage Regulator in a simulation environment
  */
  class MockMitoCANdria : public MitoCANdriaInterface {
  public:
    MockMitoCANdria() {}
    ~MockMitoCANdria() {}

    std::optional<grpl::expected<double, GrappleError>> get_channel_current(uint8_t channel) const {
      return _channelCurrent[channel];
    }

    std::optional<grpl::expected<bool, GrappleError>> get_channel_enabled(uint8_t channel) const {
      return _channelEnabled[channel];
    }

    std::optional<grpl::expected<double, GrappleError>> get_channel_voltage(uint8_t channel) const {
      return _channelVoltage[channel];
    }

    std::optional<grpl::expected<double, GrappleError>> get_channel_voltage_setpoint(uint8_t channel) const {
      return _channelVoltageSetpoint[channel];
    }

    grpl::expected<grpl::empty, GrappleError> set_channel_enabled(uint8_t channel, bool enabled) {
      _channelEnabled[channel] = enabled;
      return grpl::empty { 0 };
    }
    
    grpl::expected<grpl::empty, GrappleError> set_channel_voltage(uint8_t channel, double voltage) {
      if (channel != MITOCANDRIA_CHANNEL_ADJ) {
        return grpl::unexpected(GrappleError { "Invalid channel!", GRAPPLE_ERROR_PARAM_OUT_OF_BOUNDS });
      }
      _channelVoltageSetpoint[channel] = voltage;
      return grpl::empty { 0 };
    }

  private:
    double _channelCurrent[5] {0, 0, 0, 0, 0};
    double _channelVoltage[5] {0, 0, 0, 0, 0};
    double _channelVoltageSetpoint[5] {5, 5, 5, 5, 14.5};
    double _channelEnabled[5] {0, 0, 0, 0, 0};
  };
}