//! Tap recognition function setting register (r/w).

use crate::{RegisterAddress, RegisterConfig};

/// Tap recognition function setting register (r/w).
#[derive(Default)]
pub struct IntDur2 {
    /// Duration of maximum time gap for double tap recognition. Default: 0000
    /// When double tap recognition is enabled, this register expresses the maximum time between two consecutive detected taps to determine a double tap event. The default value of these bits is 0000b which corresponds to 16*ODR_XL time. If the DUR[3\:0] bits are set to a different value, 1LSB corresponds to 32*ODR_XL time.
    pub duration: u8,
    /// Expected quiet time after a tap detection. Default value: 00
    /// Quiet time is the time after the first detected tap in which there must not be any overthreshold event. The default value of these bits is 00b which corresponds to 2*ODR_XL time. If the QUIET[1\:0] bits are set to a different value, 1LSB corresponds to 4*ODR_XL time.
    pub quiet: u8,
    /// Maximum duration of overthreshold event. Default value: 00
    /// Maximum duration is the maximum time of an overthreshold signal detection to be recognized as a tap event. The default value of these bits is 00b which corresponds to 4*ODR_XL time. If the SHOCK[1\:0] bits are set to a different value, 1LSB corresponds to 8*ODR_XL time.
    pub shock: u8,
}

impl IntDur2 {
    pub fn address(&self) -> u8 {
        RegisterAddress::INT_DUR2.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        value |= (self.duration & 0b1111) << 4;

        value |= (self.quiet & 0b11) << 2;

        value |= self.shock & 0b11;

        value
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
