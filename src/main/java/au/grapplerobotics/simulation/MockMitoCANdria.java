package au.grapplerobotics;

import java.util.OptionalDouble;
import java.util.OptionalInt;

import au.grapplerobotics.interfaces.MitoCANdriaInterface;
import au.grapplerobotics.ConfigurationFailedException;
import au.grapplerobotics.CouldNotGetException;
import au.grapplerobotics.GrappleException;

/**
 * Class for the Grapple Robotics MitoCANdria Voltage Regulator in a simulation environment
*/
public class MockMitoCANdria implements MitoCANdriaInterface {
  private OptionalDouble _channelCurrent[] = new OptionalDouble[5];
  private OptionalDouble _channelVoltage[] = new OptionalDouble[5];
  private OptionalDouble _channelVoltageSetpoint[] = new OptionalDouble[5];
  private OptionalInt _channelEnabled[] = new OptionalInt[5];

  /**
   * Create a new (mock) MitoCANdria.
  */
  public MockMitoCANdria() {
    for (int i = 0; i < 5; i++) {
      _channelCurrent[i] = OptionalDouble.empty();
      _channelVoltage[i] = OptionalDouble.empty();
      _channelVoltageSetpoint[i] = OptionalDouble.of(5.0);
      _channelEnabled[i] = OptionalInt.empty();
    }
    _channelVoltageSetpoint[MITOCANDRIA_CHANNEL_ADJ] = OptionalDouble.empty();
  }

  /**
   * Get the current consumption of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The current consumption of the given channel in Amperes, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  @Override
  public OptionalDouble getChannelCurrent(int channel) throws CouldNotGetException {
    return _channelCurrent[channel];
  }

  /**
   * Get the voltage of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  @Override
  public OptionalDouble getChannelVoltage(int channel) throws CouldNotGetException {
    return _channelVoltage[5];
  }

  /**
   * Get the voltage setpoint of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage setpoint of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  @Override
  public OptionalDouble getChannelVoltageSetpoint(int channel) throws CouldNotGetException {
    return _channelVoltageSetpoint[channel];
  }

  /**
   * Get whether a given channel is enabled or not.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return A 0 if the channel is disabled, a 1 if the channel is enabled, or empty if the MitoCANdria
   *         is not yet available on the bus.
   */
  @Override
  public OptionalInt getChannelEnabled(int channel) throws CouldNotGetException {
    return _channelEnabled[channel];
  }

  /**
   * Set a channel to be enabled or disabled.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param enabled The desired enabled state of the channel, where true is energised.
   * @throws ConfigurationFailedException If the channel could not be enabled/disabled. If this is
   *         thrown, try again in a little while.
   */
  @Override
  public void setChannelEnabled(int channel, boolean enabled) throws ConfigurationFailedException {
    _channelEnabled[channel] = OptionalInt.of(enabled ? 1 : 0);
  }

  /**
   * Set the voltage of a channel. This will also disable the channel as a safety precaution,
   * requiring @see setChannelEnabled to be called.
   * Note: only the adjustable channel can be targetted by this method.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param voltage The desired voltage of the channel, in Volts.
   * @throws ConfigurationFailedException If the channel could not be enabled/disabled. If this is
   *         thrown, try again in a little while.
   */
  @Override
  public void setChannelVoltage(int channel, double voltage) throws ConfigurationFailedException {
    if (channel != MITOCANDRIA_CHANNEL_ADJ) {
      throw new ConfigurationFailedException("Invalid channel!", GrappleException.GRAPPLE_ERROR_PARAM_OUT_OF_BOUNDS);
    }
    _channelVoltageSetpoint[channel] = OptionalDouble.of(voltage);
  }

  /**
   * Set the channel current in simulation mode
   */
  public void setChannelCurrentSim(int channel, double current) {
    _channelCurrent[channel] = OptionalDouble.of(current);
  }

  /**
   * Set the channel voltage in simulation mode.
  */
  public void setChannelVoltageSim(int channel, double voltage) {
    _channelVoltage[channel] = OptionalDouble.of(voltage);
  }
}