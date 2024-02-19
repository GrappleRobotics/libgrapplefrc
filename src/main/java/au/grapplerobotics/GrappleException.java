package au.grapplerobotics;

public class GrappleException extends Exception {
  public static final int GRAPPLE_ERROR_PARAM_OUT_OF_BOUNDS = 0x00;
  public static final int GRAPPLE_ERROR_FAILED_ASSERTION = 0x01;
  public static final int GRAPPLE_ERROR_TIMED_OUT = 0xFE;
  public static final int GRAPPLE_ERROR_GENERIC = 0xFF;

  private int errorCode;
  public GrappleException(String message, int code) {
    super(message);
    this.errorCode = code;
  }

  public int getErrorCode() {
    return this.errorCode;
  }
}