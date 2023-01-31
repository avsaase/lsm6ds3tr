//! Single and double-tap function threshold register (r/w).

use crate::{RegisterAddress, RegisterConfig};

/// Single and double-tap function threshold register (r/w).
pub struct WakeUpThs {
    /// Single/double-tap event enable. Default: 0
    /// (0: only single-tap event enabled;
    /// 1: both single and double-tap events enabled)
    pub single_double_tap_enabled: bool,
    /// Threshold for wakeup. Default value: 000000
    /// 1 LSb corresponds to FS_XL/26
    pub wake_up_threshold: u8,
}

impl WakeUpThs {
    pub fn address(&self) -> u8 {
        RegisterAddress::WAKE_UP_THS.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.single_double_tap_enabled {
            value |= 1 << 7;
        }

        value |= self.wake_up_threshold & 0b111111;

        value
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
