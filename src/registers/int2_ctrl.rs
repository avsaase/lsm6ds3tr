//! INT2 pad control register (r/w).

use crate::RegisterAddress;

/// INT2 pad control register (r/w).
/// Each bit in this register enables a signal to be carried through INT2. The pad’s output will supply the OR combination of the selected signals.
#[derive(Default)]
pub struct Int2Ctrl {
    /// Pedometer step recognition interrupt on delta time(1) enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub pedometer_step_recognition_delta_time: bool,
    /// Step counter overflow interrupt enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub step_counter_overflow: bool,
    /// FIFO full flag interrupt enable on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_full: bool,
    /// FIFO overrun interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_overrun: bool,
    /// FIFO threshold interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub fifo_threshold: bool,
    /// Temperature Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub temperature_data_ready: bool,
    /// Gyroscope Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_data_ready: bool,
    /// Accelerometer Data Ready on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub accelerometer_data_ready: bool,
}

impl Int2Ctrl {
    pub fn address(self) -> u8 {
        RegisterAddress::INT2_CTRL.address()
    }

    pub fn value(self) -> u8 {
        let mut value: u8 = 0;

        if self.pedometer_step_recognition_delta_time {
            value |= 1 << 7;
        }

        if self.step_counter_overflow {
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

        if self.temperature_data_ready {
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
}
