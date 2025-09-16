from libgrapplefrc import (
    can_bridge_tcp,
    LaserCAN as _LaserCAN,
    LaserCanMeasurement as _LaserCanMeasurement,
    LaserCanRangingMode,
    LaserCanRoi as _LaserCanRoi,
    LaserCanTimingBudget,
    MitoCANdria as _MitoCANdria,
)

__all__ = [
    "can_bridge_tcp",
    "LaserCAN",
    "LaserCanMeasurement",
    "LaserCanRangingMode",
    "LaserCanRoi",
    "LaserCanTimingBudget",
    "MitoCANdria",
]


class LaserCAN(_LaserCAN):
    """Wrapper for LaserCAN sensor."""

    pass


class LaserCanMeasurement(_LaserCanMeasurement):
    """Measurement result from LaserCAN."""

    pass


class LaserCanRoi(_LaserCanRoi):
    """Region of interest for LaserCAN."""

    pass


class MitoCANdria(_MitoCANdria):
    """CAN communication abstraction."""

    pass
