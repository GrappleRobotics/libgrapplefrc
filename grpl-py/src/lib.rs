use pyo3::prelude::*;

#[pyclass]
enum LaserCanRangingMode {
    Long,
    Short,
}

#[pyclass]
enum LaserCanTimingBudget {
    TimingBudget20ms = 20,
    TimingBudget33ms = 33,
    TimingBudget50ms = 50,
    TimingBudget100ms = 100,
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

    Ok(())
}
