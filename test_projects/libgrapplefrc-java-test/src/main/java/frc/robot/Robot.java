// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

package frc.robot;

import au.grapplerobotics.LaserCan;
import edu.wpi.first.wpilibj.TimedRobot;

/**
 * The VM is configured to automatically run this class, and to call the functions corresponding to
 * each mode, as described in the TimedRobot documentation. If you change the name of this class or
 * the package after creating this project, you must also update the build.gradle file in the
 * project.
 */
public class Robot extends TimedRobot {
  private LaserCan lasercan;

  @Override
  public void robotInit() {
    lasercan = new LaserCan(1);
    lasercan.setRangingMode(LaserCan.RangingMode.SHORT);
    lasercan.setRegionOfInterest(new LaserCan.RegionOfInterest(8, 8, 16, 16));
    lasercan.setTimingBudget(LaserCan.TimingBudget.TIMING_BUDGET_33MS);
  }

  @Override
  public void robotPeriodic() {
    LaserCan.Measurement status = lasercan.getMeasurement();
    if (status != null && status.status == 0)
      System.out.println("The target is " + status.distance_mm + "mm away!");
    else
      System.out.println("Oh no! The target is out of range, or we can't get a reliable measurement!");
  }

  @Override
  public void autonomousInit() {}

  @Override
  public void autonomousPeriodic() {}

  @Override
  public void teleopInit() {}

  @Override
  public void teleopPeriodic() {}

  @Override
  public void disabledInit() {}

  @Override
  public void disabledPeriodic() {}

  @Override
  public void testInit() {}

  @Override
  public void testPeriodic() {}
}
