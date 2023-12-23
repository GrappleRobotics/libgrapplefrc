use grapple_frc_msgs::grapple::lasercan::{LaserCanRoi, LaserCanStatusFrame};
use grapplefrcdriver::{lasercan::LaserCanDevice, with_err};
use pyo3::prelude::*;

#[pyclass]
#[derive(PartialEq)]
enum LaserCanRangingMode {
    Long,
    Short,
}

impl LaserCanRangingMode {
    fn is_long(&self) -> bool {
        if *self == LaserCanRangingMode::Long {
            return true;
        }
        false
    }
}

#[pyclass]
enum LaserCanTimingBudget {
    TimingBudget20ms,
    TimingBudget33ms,
    TimingBudget50ms,
    TimingBudget100ms,
}

impl LaserCanTimingBudget {
    fn as_u8(&self) -> u8 {
        match self {
            LaserCanTimingBudget::TimingBudget20ms => 20,
            LaserCanTimingBudget::TimingBudget33ms => 33,
            LaserCanTimingBudget::TimingBudget50ms => 50,
            LaserCanTimingBudget::TimingBudget100ms => 100,
        }
    }
}

#[pyclass]
struct LaserCAN {
    can_id: u8,
    handle: LaserCanDevice,
}

impl LaserCAN {
    fn new(can_id: u8) -> Self {
        let handle = LaserCanDevice::new(can_id);
        LaserCAN { handle, can_id }
    }

    fn get_measurement(&mut self) -> Option<LaserCanStatusFrame> {
        let status = self.handle.status();
        if status == None {
            return None;
        }

        if status.clone().unwrap().status != 0xFF {
            return status;
        }

        None
    }

    fn set_ranging_mode(&mut self, mode: LaserCanRangingMode) -> i32 {
        with_err(self.handle.set_range(mode.is_long()))
    }

    fn set_timing_range_budget(&mut self, budget: LaserCanTimingBudget) -> i32 {
        with_err(self.handle.set_timing_budget(budget.as_u8()))
    }

    fn set_roi(&mut self, roi: LaserCanRoi) -> i32 {
        with_err(self.handle.set_roi(roi))
    }
}

unsafe impl Send for LaserCAN {}

#[pymodule]
fn grpl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("LASERCAN_STATUS_VALID_MEASUREMENT", 0)?;
    m.add("LASERCAN_STATUS_NOISE_ISSUE", 1)?;
    m.add("LASERCAN_STATUS_WEAK_SIGNAL", 2)?;
    m.add("LASERCAN_STATUS_OUT_OF_BOUNDS", 4)?;
    m.add("LASERCAN_STATUS_WRAPAROUND", 7)?;

    m.add_class::<LaserCanRangingMode>()?;
    m.add_class::<LaserCanTimingBudget>()?;
    m.add_class::<LaserCAN>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{LaserCanRangingMode, LaserCanTimingBudget};

    fn laser_can_budget_tester(budget: &LaserCanTimingBudget) {
        match budget {
            LaserCanTimingBudget::TimingBudget20ms => assert_eq!(budget.as_u8(), 20),
            LaserCanTimingBudget::TimingBudget33ms => assert_eq!(budget.as_u8(), 33),
            LaserCanTimingBudget::TimingBudget50ms => assert_eq!(budget.as_u8(), 50),
            LaserCanTimingBudget::TimingBudget100ms => assert_eq!(budget.as_u8(), 100),
        }
    }

    #[test]
    fn test_laser_can_budget_u8() {
        let mut budget = LaserCanTimingBudget::TimingBudget20ms;
        laser_can_budget_tester(&budget);

        budget = LaserCanTimingBudget::TimingBudget33ms;
        laser_can_budget_tester(&budget);

        budget = LaserCanTimingBudget::TimingBudget50ms;
        laser_can_budget_tester(&budget);

        budget = LaserCanTimingBudget::TimingBudget100ms;
        laser_can_budget_tester(&budget);
    }

    #[test]
    fn test_ranging_is_long() {
        let mut ranging_mode = LaserCanRangingMode::Long;
        assert_eq!(ranging_mode.is_long(), true);

        ranging_mode = LaserCanRangingMode::Short;
        assert_eq!(ranging_mode.is_long(), false);
    }
}
