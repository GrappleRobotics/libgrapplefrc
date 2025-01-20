package au.grapplerobotics.interfaces;

import java.util.OptionalDouble;
import java.util.OptionalInt;

import au.grapplerobotics.ConfigurationFailedException;
import au.grapplerobotics.CouldNotGetException;

public interface MitoCANdriaInterface {
  int MITOCANDRIA_CHANNEL_USB1 = 0;
  int MITOCANDRIA_CHANNEL_USB2 = 1;
  int MITOCANDRIA_CHANNEL_5VA = 2;
  int MITOCANDRIA_CHANNEL_5VB = 3;
  int MITOCANDRIA_CHANNEL_ADJ = 4;

  /**
   * Get the current consumption of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The current consumption of the given channel in Amperes, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  OptionalDouble getChannelCurrent(int channel) throws CouldNotGetException;

  /**
   * Get the voltage of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  OptionalDouble getChannelVoltage(int channel) throws CouldNotGetException;

  /**
   * Get the voltage setpoint of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage setpoint of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  OptionalDouble getChannelVoltageSetpoint(int channel) throws CouldNotGetException;

  /**
   * Get whether a given channel is enabled or not.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return A 0 if the channel is disabled, a 1 if the channel is enabled, or empty if the MitoCANdria
   *         is not yet available on the bus.
   */
  OptionalInt getChannelEnabled(int channel) throws CouldNotGetException;

  /**
   * Set a channel to be enabled or disabled.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param enabled The desired enabled state of the channel, where true is energised.
   * @throws ConfigurationFailedException If the channel could not be enabled/disabled. If this is
   *         thrown, try again in a little while.
   */
  void setChannelEnabled(int channel, boolean enabled) throws ConfigurationFailedException;

  /**
   * Set the voltage of a channel. This will also disable the channel as a safety precaution,
   * requiring @see setChannelEnabled to be called.
   * Note: only the adjustable channel can be targetted by this method.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param voltage The desired voltage of the channel, in Volts.
   * @throws ConfigurationFailedException If the channel voltage could not be set. If this is
   *         thrown, try again in a little while. This will always throw if the channel is not
   *         the adjustable rail.
   */
  void setChannelVoltage(int channel, double voltage) throws ConfigurationFailedException;
}