//! INT1 pad control register (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// INT1 pad control register (r/w).
/// Each bit in this register enables a signal to be carried through INT1. The pad’s output will supply the OR combination of the selected signals.
#[derive(Default)]
pub struct Int1Ctrl {
    /// Pedometer step recognition interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub pedometer_step_recognition: RegisterBits<1, 7>,
    /// Significant motion interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub significant_motion: RegisterBits<1, 6>,
    /// FIFO full flag interrupt enable on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_full: RegisterBits<1, 5>,
    /// FIFO overrun interrupt on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_overrun: RegisterBits<1, 4>,
    /// FIFO threshold interrupt on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_threshold: RegisterBits<1, 3>,
    /// Boot status available on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub boot_status_available: RegisterBits<1, 2>,
    /// Gyroscope Data Ready on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_data_ready: RegisterBits<1, 1>,
    /// Accelerometer Data Ready on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub accelerometer_data_ready: RegisterBits<1, 0>,
}

impl Int1Ctrl {
    pub fn address(&self) -> u8 {
        RegisterAddress::INT1_CTRL.address()
    }

    pub fn value(&self) -> u8 {
        self.pedometer_step_recognition.shifted()
            | self.significant_motion.shifted()
            | self.fifo_full.shifted()
            | self.fifo_overrun.shifted()
            | self.fifo_threshold.shifted()
            | self.boot_status_available.shifted()
            | self.gyroscope_data_ready.shifted()
            | self.accelerometer_data_ready.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
