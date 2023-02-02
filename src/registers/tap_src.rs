//! Tap source register (r).

use crate::{RegisterAddress, RegisterBits};

/// Tap source register (r).
#[derive(Default)]
pub struct TapSrc {
    /// Tap event detection status. Default: 0
    /// (0: tap event not detected; 1: tap event detected)
    pub tap_event: RegisterBits<1, 6>,
    /// Single-tap event status. Default value: 0
    /// (0: single tap event not detected; 1: single tap event detected)
    pub single_tap_event: RegisterBits<1, 5>,
    /// Double-tap event detection status. Default value: 0
    /// (0: double-tap event not detected; 1: double-tap event detected.)
    pub double_tap_event: RegisterBits<1, 4>,
    /// Sign of acceleration detected by tap event. Default: 0
    /// (0: positive sign of acceleration detected by tap event;
    /// 1: negative sign of acceleration detected by tap event)
    pub tap_sign_acceleration: RegisterBits<1, 3>,
    /// Tap event detection status on X-axis. Default value: 0
    /// (0: tap event on X-axis not detected; 1: tap event on X-axis detected)
    pub tap_x_axis: RegisterBits<1, 2>,
    /// Tap event detection status on Y-axis. Default value: 0
    /// (0: tap event on Y-axis not detected; 1: tap event on Y-axis detected)
    pub tap_y_axis: RegisterBits<1, 1>,
    /// Tap event detection status on Z-axis. Default value: 0
    /// (0: tap event on Z-axis not detected; 1: tap event on Z-axis detected)
    pub tap_z_axis: RegisterBits<1, 0>,
}

impl TapSrc {
    pub fn address(&self) -> u8 {
        RegisterAddress::TAP_SRC.address()
    }
}

impl From<u8> for TapSrc {
    fn from(value: u8) -> Self {
        let mut s = Self::default();
        s.tap_event.from_reg(value);
        s.single_tap_event.from_reg(value);
        s.double_tap_event.from_reg(value);
        s.tap_sign_acceleration.from_reg(value);
        s.tap_x_axis.from_reg(value);
        s.tap_y_axis.from_reg(value);
        s.tap_z_axis.from_reg(value);
        s
    }
}
