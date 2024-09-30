package au.grapplerobotics;

public class CouldNotGetException extends GrappleException {
  public CouldNotGetException(String message, int code) {
    super(message, code);
  }
}