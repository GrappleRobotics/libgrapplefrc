package au.grapplerobotics;

import java.lang.AutoCloseable;
import java.lang.ref.Cleaner;

import java.util.OptionalDouble;
import java.util.OptionalInt;

/**
 * Class for the Grapple Robotics MitoCANdria Voltage Regulator.
*/
public class MitoCANdria implements AutoCloseable {
  public static final int MITOCANDRIA_CHANNEL_USB1 = 0;
  public static final int MITOCANDRIA_CHANNEL_USB2 = 1;
  public static final int MITOCANDRIA_CHANNEL_5VA = 2;
  public static final int MITOCANDRIA_CHANNEL_5VB = 3;
  public static final int MITOCANDRIA_CHANNEL_ADJ = 4;

  static native long init(int can_id);
  static native void free(long handle);

  static class Handle implements Runnable {
    long handle; 

    Handle(int can_id) {
      this.handle = init(can_id);
    }

    @Override
    public void run() {
      free(this.handle);
    }
  }

  private final Handle handle;
  private final Cleaner.Cleanable cleanable;

  /**
   * Create a new MitoCANdria. 
   * 
   * @param can_id The CAN ID for the MitoCANdria. This ID is unique, and set in GrappleHook.
   *               Note: one ID should be mapped to only one sensor, or else measurements will conflict.
  */
  public MitoCANdria(int can_id) {
    try {
      GrappleJNI.forceLoad();
    } catch (UnsatisfiedLinkError e) {
      e.printStackTrace();
      System.exit(1);
    }
    
    this.handle = new Handle(can_id);
    this.cleanable = GrappleJNI.cleaner.register(this, this.handle);
  }

  /**
   * Get the current consumption of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The current consumption of the given channel in Amperes, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  public native OptionalDouble getChannelCurrent(int channel) throws CouldNotGetException;

  /**
   * Get the voltage of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  public native OptionalDouble getChannelVoltage(int channel) throws CouldNotGetException;

  /**
   * Get the voltage setpoint of a channel at this point in time.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return The voltage setpoint of the given channel in Volts, or empty if the MitoCANdria is not yet
   *         available on the bus.
   */
  public native OptionalDouble getChannelVoltageSetpoint(int channel) throws CouldNotGetException;

  /**
   * Get whether a given channel is enabled or not.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @throws CouldNotGetException Throws when the channel is out-of-bounds (not a valid channel)
   * @return A 0 if the channel is disabled, a 1 if the channel is enabled, or empty if the MitoCANdria
   *         is not yet available on the bus.
   */
  public native OptionalInt getChannelEnabled(int channel) throws CouldNotGetException;

  /**
   * Set a channel to be enabled or disabled.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param enabled The desired enabled state of the channel, where true is energised.
   * @throws ConfigurationFailedException If the channel could not be enabled/disabled. If this is
   *         thrown, try again in a little while.
   */
  public native void setChannelEnabled(int channel, boolean enabled) throws ConfigurationFailedException;

  /**
   * Set the voltage of a channel. This will also disable the channel as a safety precaution,
   * requiring @see setChannelEnabled to be called.
   * Note: only the adjustable channel can be targetted by this method.
   * @param channel The channel. Must be one of MitoCANdria.MITOCANDRIA_CHANNEL_*
   * @param voltage The desired voltage of the channel, in Volts.
   * @throws ConfigurationFailedException If the channel could not be enabled/disabled. If this is
   *         thrown, try again in a little while.
   */
  public native void setChannelVoltage(int channel, double voltage) throws ConfigurationFailedException;

  @Override
  public void close() throws Exception {
    cleanable.clean();
  }
}