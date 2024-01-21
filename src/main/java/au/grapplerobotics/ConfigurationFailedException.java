package au.grapplerobotics;

public class ConfigurationFailedException extends GrappleException {
  public ConfigurationFailedException(String message, int code) {
    super(message, code);
  }
}