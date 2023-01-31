//! Tap source register (r).

use crate::RegisterAddress;

/// Tap source register (r).
pub struct TapSrc {
    /// Tap event detection status. Default: 0
    /// (0: tap event not detected; 1: tap event detected)
    pub tap_event: bool,
    /// Single-tap event status. Default value: 0
    /// (0: single tap event not detected; 1: single tap event detected)
    pub single_tap_event: bool,
    /// Double-tap event detection status. Default value: 0
    /// (0: double-tap event not detected; 1: double-tap event detected.)
    pub double_tap_event: bool,
    /// Sign of acceleration detected by tap event. Default: 0
    /// (0: positive sign of acceleration detected by tap event;
    /// 1: negative sign of acceleration detected by tap event)
    pub tap_sign_acceleration: bool,
    /// Tap event detection status on X-axis. Default value: 0
    /// (0: tap event on X-axis not detected; 1: tap event on X-axis detected)
    pub tap_x_axis: bool,
    /// Tap event detection status on Y-axis. Default value: 0
    /// (0: tap event on Y-axis not detected; 1: tap event on Y-axis detected)
    pub tap_y_axis: bool,
    /// Tap event detection status on Z-axis. Default value: 0
    /// (0: tap event on Z-axis not detected; 1: tap event on Z-axis detected)
    pub tap_z_axis: bool,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TapSourceBits {
    TapEvent = 0b1000000,
    SingleTapEvent = 0b100000,
    DoubleTapEvent = 0b10000,
    TapSignAcceleration = 0b1000,
    TapXAxis = 0b100,
    TapYAxis = 0b10,
    TapZAxis = 0b1,
}

impl TapSourceBits {
    pub fn value(&self) -> u8 {
        *self as u8
    }
}

use TapSourceBits::*;

impl TapSrc {
    pub fn address(&self) -> u8 {
        RegisterAddress::TAP_SRC.address()
    }
}

impl From<u8> for TapSrc {
    fn from(value: u8) -> Self {
        Self {
            tap_event: value & TapEvent.value() != 0,
            single_tap_event: value & SingleTapEvent.value() != 0,
            double_tap_event: value & DoubleTapEvent.value() != 0,
            tap_sign_acceleration: value & TapSignAcceleration.value() != 0,
            tap_x_axis: value & TapXAxis.value() != 0,
            tap_y_axis: value & TapYAxis.value() != 0,
            tap_z_axis: value & TapZAxis.value() != 0,
        }
    }
}
