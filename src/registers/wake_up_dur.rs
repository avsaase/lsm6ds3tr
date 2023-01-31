//! Free-fall, wakeup, timestamp and sleep mode functions duration setting register (r/w).

use crate::{RegisterAddress, RegisterConfig};

/// Free-fall, wakeup, timestamp and sleep mode functions duration setting register (r/w).
#[derive(Default)]
pub struct WakeUpDur {
    /// Free fall duration event. Default: 0
    /// For the complete configuration of the free-fall duration, refer to FF_DUR[4\:0] in FREE_FALL (5Dh) configuration.
    /// 1 LSB = 1 ODR_time
    pub free_fall_duration_event: bool,
    /// Wake up duration event. Default: 00
    /// 1LSB = 1 ODR_time
    pub wake_up_duration_event: u8,
    /// Timestamp register resolution setting. Default value: 0
    /// (0: 1LSB = 6.4 ms; 1: 1LSB = 25 μs)
    pub timestamp_resolution: bool,
    /// Duration to go in sleep mode. Default value: 0000 (this corresponds to 16 ODR)
    /// 1 LSB = 512 ODR
    pub sleep_duration_event: u8,
}

impl WakeUpDur {
    pub fn address(&self) -> u8 {
        RegisterAddress::WAKE_UP_DUR.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.free_fall_duration_event {
            value |= 1 << 7;
        }

        value |= (self.wake_up_duration_event & 0b11) << 5;

        if self.timestamp_resolution {
            value |= 1 << 4;
        }

        value |= self.sleep_duration_event & 0b1111;

        value
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
