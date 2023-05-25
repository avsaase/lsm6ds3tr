//! Single and double-tap function threshold register (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Single and double-tap function threshold register (r/w).
#[derive(Default)]
pub struct WakeUpThs {
    /// Single/double-tap event enable. Default: 0
    /// (0: only single-tap event enabled;
    /// 1: both single and double-tap events enabled)
    pub single_double_tap_enabled: RegisterBits<1, 7>,
    /// Threshold for wakeup. Default value: 000000
    /// 1 LSb corresponds to FS_XL/2^6
    pub wake_up_threshold: RegisterBits<6, 0>,
}

impl WakeUpThs {
    pub fn address(&self) -> u8 {
        RegisterAddress::WAKE_UP_THS.address()
    }

    pub fn value(&self) -> u8 {
        self.single_double_tap_enabled.shifted() | self.wake_up_threshold.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
