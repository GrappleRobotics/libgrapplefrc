// Copyright (c) FIRST and other WPILib contributors.
// Open Source Software; you can modify and/or share it under the terms of
// the WPILib BSD license file in the root directory of this project.

#include "Robot.h"

#include <iostream>

void Robot::RobotInit() {
  lc = new grpl::LaserCan(0);
  if (lc->set_ranging_mode(grpl::LaserCanRangingMode::Long))
    std::cout << "ERROR 1" << std::endl;
  if (lc->set_timing_budget(grpl::LaserCanTimingBudget::TimingBudget100ms))
    std::cout << "ERROR 2" << std::endl;
  if (lc->set_roi(grpl::LaserCanROI{ 8, 8, 16, 16 }))
    std::cout << "ERROR 3" << std::endl;
}

void Robot::RobotPeriodic() {
  std::optional<grpl::LaserCanMeasurement> measurement = lc->get_measurement();
  if (measurement.has_value()) {
    std::cout << "The target is " << measurement.value().distance_mm << "mm away!" << std::endl;
  } else {
    std::cout << "Oh no! The target is out of range, or we can't get a reliable measurement!" << std::endl;
  }
}

void Robot::AutonomousInit() {}
void Robot::AutonomousPeriodic() {}

void Robot::TeleopInit() {}
void Robot::TeleopPeriodic() {}

void Robot::DisabledInit() {}
void Robot::DisabledPeriodic() {}

void Robot::TestInit() {}
void Robot::TestPeriodic() {}

#ifndef RUNNING_FRC_TESTS
int main() {
  return frc::StartRobot<Robot>();
}
#endif
