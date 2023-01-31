//! Free-fall function duration setting register (r/w).

use crate::RegisterAddress;

/// Free-fall function duration setting register (r/w).
#[derive(Default)]
pub struct FreeFall {
    /// Free-fall duration event. Default: 0
    /// For the complete configuration of the free fall duration, refer to FF_DUR5 in WAKE_UP_DUR (5Ch) configuration
    pub duration_event: u8,
    /// Free fall threshold setting. Default: 000
    /// For details refer to Table 196.
    pub threshold: FreeFallThreshold,
}

impl FreeFall {
    pub fn address(self) -> u8 {
        RegisterAddress::FREE_FALL.address()
    }

    pub fn value(self) -> u8 {
        (self.duration_event & 0b11111) << 3 | self.threshold.value()
    }
}

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Default)]
pub enum FreeFallThreshold {
    #[default]
    _156_mg = 0b000,
    _219_mg = 0b001,
    _250_mg = 0b010,
    _312_mg = 0b011,
    _344_mg = 0b100,
    _406_mg = 0b101,
    _469_mg = 0b110,
    _500_mg = 0b111,
}

impl FreeFallThreshold {
    pub fn value(self) -> u8 {
        self as u8
    }
}