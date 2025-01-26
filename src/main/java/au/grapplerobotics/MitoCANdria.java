package au.grapplerobotics;

import java.lang.AutoCloseable;
import java.lang.ref.Cleaner;

import java.util.OptionalDouble;
import java.util.OptionalInt;

import au.grapplerobotics.interfaces.MitoCANdriaInterface;

/**
 * Class for the Grapple Robotics MitoCANdria Voltage Regulator.
*/
public class MitoCANdria implements MitoCANdriaInterface, AutoCloseable {
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

  @Override
  public native OptionalDouble getChannelCurrent(int channel) throws CouldNotGetException;

  @Override
  public native OptionalDouble getChannelVoltage(int channel) throws CouldNotGetException;

  @Override
  public native OptionalDouble getChannelVoltageSetpoint(int channel) throws CouldNotGetException;

  @Override
  public native OptionalInt getChannelEnabled(int channel) throws CouldNotGetException;

  @Override
  public native void setChannelEnabled(int channel, boolean enabled) throws ConfigurationFailedException;

  @Override
  public native void setChannelVoltage(int channel, double voltage) throws ConfigurationFailedException;

  @Override
  public void close() throws Exception {
    cleanable.clean();
  }
}