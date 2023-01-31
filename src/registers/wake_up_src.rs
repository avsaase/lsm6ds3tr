//! Wake up interrupt source register (r).

use crate::RegisterAddress;

/// Wake up interrupt source register (r).
pub struct WakeUpSrc {
    /// Free-fall event detection status. Default: 0
    /// (0: free-fall event not detected; 1: free-fall event detected)
    pub free_fall_event: bool,
    /// Sleep event status. Default value: 0
    /// (0: sleep event not detected; 1: sleep event detected)
    pub sleep_event: bool,
    /// Wakeup event detection status. Default value: 0
    /// (0: wakeup event not detected; 1: wakeup event detected.)
    pub wake_up_event: bool,
    /// Wakeup event detection status on X-axis. Default value: 0
    /// (0: wakeup event on X-axis not detected; 1: wakeup event on X-axis detected)
    pub wake_up_event_x: bool,
    /// Wakeup event detection status on Y-axis. Default value: 0
    /// (0: wakeup event on Y-axis not detected; 1: wakeup event on Y-axis detected)
    pub wake_up_event_y: bool,
    /// Wakeup event detection status on Z-axis. Default value: 0
    /// (0: wakeup event on Z-axis not detected; 1: wakeup event on Z-axis detected)
    pub wake_up_event_z: bool,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum WakeUpSourceBits {
    FreeFallEvent = 0b100000,
    SleepEvent = 0b10000,
    WakeUpEvent = 0b1000,
    WakeUpEventX = 0b100,
    WakeUpEventY = 0b10,
    WakeUpEventZ = 0b1,
}

impl WakeUpSourceBits {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

use WakeUpSourceBits::*;

impl WakeUpSrc {
    pub fn address(&self) -> u8 {
        RegisterAddress::WAKE_UP_SRC.address()
    }
}

impl From<u8> for WakeUpSrc {
    fn from(value: u8) -> Self {
        Self {
            free_fall_event: value & FreeFallEvent.value() != 0,
            sleep_event: value & SleepEvent.value() != 0,
            wake_up_event: value & WakeUpEvent.value() != 0,
            wake_up_event_x: value & WakeUpEventX.value() != 0,
            wake_up_event_y: value & WakeUpEventY.value() != 0,
            wake_up_event_z: value & WakeUpEventZ.value() != 0,
        }
    }
}
