use pyo3::prelude::*;
use grapplefrcdriver::lasercan::{LaserCanDevice, lasercan_get_status, lasercan_set_range, lasercan_set_timing_budget, lasercan_set_roi};
use grapple_frc_msgs::grapple::lasercan::{LaserCanStatusFrame, LaserCanRoi};
use std::ptr::addr_of_mut;

#[pyclass]
#[derive(PartialEq)]
enum LaserCanRangingMode {
    Long,
    Short,
}

impl LaserCanRangingMode {
    fn is_long(&self) -> bool {
        if *self == LaserCanRangingMode::Long {
            return true
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
        LaserCAN {
            handle,
            can_id,
        }
    }

    fn get_measurement(&mut self) -> Option<LaserCanStatusFrame> {
        let status = lasercan_get_status(addr_of_mut!(self.handle));
        if  status.status != 0xFF {
            return Some(status)
        }
        None
    }

    fn set_ranging_mode(&mut self, mode : LaserCanRangingMode) -> i32 {
        lasercan_set_range(addr_of_mut!(self.handle), mode.is_long())
    }

    fn set_timing_range_budget(&mut self, budget: LaserCanTimingBudget) -> i32 {
        lasercan_set_timing_budget(addr_of_mut!(self.handle), budget.as_u8())
    }

    fn set_roi(&mut self, roi: LaserCanRoi) -> i32 {
        lasercan_set_roi(addr_of_mut!(self.handle), roi)
    }
}

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
