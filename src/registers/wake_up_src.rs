//! Wake up interrupt source register (r).

use crate::{RegisterAddress, RegisterBits};

/// Wake up interrupt source register (r).
#[derive(Default)]
pub struct WakeUpSrc {
    /// Free-fall event detection status. Default: 0
    /// (0: free-fall event not detected; 1: free-fall event detected)
    pub free_fall_event: RegisterBits<1, 5>,
    /// Sleep event status. Default value: 0
    /// (0: sleep event not detected; 1: sleep event detected)
    pub sleep_event: RegisterBits<1, 4>,
    /// Wakeup event detection status. Default value: 0
    /// (0: wakeup event not detected; 1: wakeup event detected.)
    pub wake_up_event: RegisterBits<1, 3>,
    /// Wakeup event detection status on X-axis. Default value: 0
    /// (0: wakeup event on X-axis not detected; 1: wakeup event on X-axis detected)
    pub wake_up_event_x: RegisterBits<1, 2>,
    /// Wakeup event detection status on Y-axis. Default value: 0
    /// (0: wakeup event on Y-axis not detected; 1: wakeup event on Y-axis detected)
    pub wake_up_event_y: RegisterBits<1, 1>,
    /// Wakeup event detection status on Z-axis. Default value: 0
    /// (0: wakeup event on Z-axis not detected; 1: wakeup event on Z-axis detected)
    pub wake_up_event_z: RegisterBits<1, 0>,
}

impl WakeUpSrc {
    pub fn address(&self) -> u8 {
        RegisterAddress::WAKE_UP_SRC.address()
    }
}

impl From<u8> for WakeUpSrc {
    fn from(value: u8) -> Self {
        let mut s = Self::default();
        s.free_fall_event.from_reg(value);
        s.sleep_event.from_reg(value);
        s.wake_up_event.from_reg(value);
        s.wake_up_event_x.from_reg(value);
        s.wake_up_event_y.from_reg(value);
        s.wake_up_event_z.from_reg(value);
        s
    }
}
