//! INT2 pad control register (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// INT2 pad control register (r/w).
/// Each bit in this register enables a signal to be carried through INT2. The pad’s output will supply the OR combination of the selected signals.
#[derive(Default)]
pub struct Int2Ctrl {
    /// Pedometer step recognition interrupt on delta time(1) enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub pedometer_step_recognition_delta_time: RegisterBits<1, 7>,
    /// Step counter overflow interrupt enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub step_counter_overflow: RegisterBits<1, 6>,
    /// FIFO full flag interrupt enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_full: RegisterBits<1, 5>,
    /// FIFO overrun interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_overrun: RegisterBits<1, 4>,
    /// FIFO threshold interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_threshold: RegisterBits<1, 3>,
    /// Temperature Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub temperature_data_ready: RegisterBits<1, 2>,
    /// Gyroscope Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_data_ready: RegisterBits<1, 1>,
    /// Accelerometer Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub accelerometer_data_ready: RegisterBits<1, 0>,
}

impl Int2Ctrl {
    pub fn address(&self) -> u8 {
        RegisterAddress::INT2_CTRL.address()
    }

    pub fn value(&self) -> u8 {
        self.pedometer_step_recognition_delta_time.shifted()
            | self.step_counter_overflow.shifted()
            | self.fifo_full.shifted()
            | self.fifo_overrun.shifted()
            | self.fifo_threshold.shifted()
            | self.temperature_data_ready.shifted()
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
