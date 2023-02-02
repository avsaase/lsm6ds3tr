//! Portrait/landscape position and tap function threshold register (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Portrait/landscape position and tap function threshold register (r/w).
#[derive(Default)]
pub struct TapThs6d {
    /// 4D orientation detection enable. Z-axis position detection is disabled. Default value: 0
    /// (0: enabled; 1: disabled)
    pub four_degrees_detection_enable: RegisterBits<1, 7>,
    /// Threshold for 4D/6D function. Default value: 00
    /// For details, refer to Table 187.
    pub six_degrees_threshold: RegisterBits<2, 5>,
    /// Threshold for tap recognition. Default value: 00000
    /// 1 LSb corresponds to FS_XL/2^5
    pub tap_threshold: RegisterBits<5, 0>,
}

impl TapThs6d {
    pub fn address(&self) -> u8 {
        RegisterAddress::TAP_THS_6D.address()
    }

    pub fn value(&self) -> u8 {
        self.four_degrees_detection_enable.shifted()
            | self.six_degrees_threshold.shifted()
            | self.tap_threshold.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
