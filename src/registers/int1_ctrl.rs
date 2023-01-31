//! INT1 pad control register (r/w).

use crate::RegisterAddress;
use crate::RegisterConfig;

/// INT1 pad control register (r/w).
/// Each bit in this register enables a signal to be carried through INT1. The pad’s output will supply the OR combination of the selected signals.
#[derive(Default)]
pub struct Int1Ctrl {
    /// Pedometer step recognition interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub pedometer_step_recognition: bool,
    /// Significant motion interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub significant_motion: bool,
    /// FIFO full flag interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_full: bool,
    /// FIFO overrun interrupt on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_overrun: bool,
    /// FIFO threshold interrupt on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_threshold: bool,
    /// Boot status available on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub boot_status_available: bool,
    /// Gyroscope Data Ready on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_data_ready: bool,
    /// Accelerometer Data Ready on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub accelerometer_data_ready: bool,
}

impl Int1Ctrl {
    pub fn address(&self) -> u8 {
        RegisterAddress::INT1_CTRL.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.pedometer_step_recognition {
            value |= 1 << 7;
        }

        if self.significant_motion {
            value |= 1 << 6;
        }

        if self.fifo_full {
            value |= 1 << 5;
        }

        if self.fifo_overrun {
            value |= 1 << 4;
        }

        if self.fifo_threshold {
            value |= 1 << 3;
        }

        if self.boot_status_available {
            value |= 1 << 2;
        }

        if self.gyroscope_data_ready {
            value |= 1 << 1;
        }

        if self.accelerometer_data_ready {
            value |= 1 << 0;
        }

        value
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
