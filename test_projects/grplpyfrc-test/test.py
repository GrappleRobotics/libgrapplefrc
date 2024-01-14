import wpilib
import grplpyfrc

class MyRobot(wpilib.TimedRobot):
  def robotInit(self):
    self.lasercan = grplpyfrc.LaserCAN(0)

  def teleopPeriodic(self) -> None:
    print(self.lasercan.get_measurement())