//! Wake up interrupt source register (r).
//!

use heapless::Vec;

use crate::{IrqSource, RegisterAddress, RegisterBits};

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

    pub fn get_irq_sources(&self) -> Vec<IrqSource, 6> {
        let mut v = Vec::new();
        if self.free_fall_event.value() != 0 {
            v.push(IrqSource::FreeFall).unwrap();
        }
        if self.sleep_event.value() != 0 {
            v.push(IrqSource::Sleep).unwrap();
        }
        if self.wake_up_event.value() != 0 {
            v.push(IrqSource::WakeUp).unwrap();
        }
        if self.wake_up_event_x.value() != 0 {
            v.push(IrqSource::WakeUpOnX).unwrap();
        }
        if self.wake_up_event_y.value() != 0 {
            v.push(IrqSource::WakeUpOnY).unwrap();
        }
        if self.wake_up_event_z.value() != 0 {
            v.push(IrqSource::WakeUpOnZ).unwrap();
        }
        v
    }
}

impl From<u8> for WakeUpSrc {
    fn from(value: u8) -> Self {
        let mut s = Self::default();
        s.free_fall_event.set_from_reg(value);
        s.sleep_event.set_from_reg(value);
        s.wake_up_event.set_from_reg(value);
        s.wake_up_event_x.set_from_reg(value);
        s.wake_up_event_y.set_from_reg(value);
        s.wake_up_event_z.set_from_reg(value);
        s
    }
}
