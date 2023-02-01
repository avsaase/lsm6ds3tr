//! Function routing on INT2 register (r/w).

use crate::RegisterAddress;
use crate::RegisterConfig;

/// Function routing on INT2 register (r/w).
#[derive(Default)]
pub struct Md2Cfg {
    /// Routing on INT2 of inactivity mode. Default: 0
    /// (0: routing on INT2 of inactivity disabled; 1: routing on INT2 of inactivity enabled)
    pub inactivity_event: bool,
    /// Single-tap recognition routing on INT2. Default: 0
    /// (0: routing of single-tap event on INT2 disabled;
    /// 1: routing of single-tap event on INT2 enabled)
    pub single_tap_event: bool,
    /// Routing of wakeup event on INT2. Default value: 0
    /// (0: routing of wakeup event on INT2 disabled;
    /// 1: routing of wake-up event on INT2 enabled)
    pub wake_up_event: bool,
    /// Routing of free-fall event on INT2. Default value: 0
    /// (0: routing of free-fall event on INT2 disabled;
    /// 1: routing of free-fall event on INT2 enabled)
    pub free_fall_event: bool,
    /// Routing of tap event on INT2. Default value: 0
    /// (0: routing of double-tap event on INT2 disabled;
    /// 1: routing of double-tap event on INT2 enabled)
    pub double_tap_event: bool,
    /// Routing of 6D event on INT2. Default value: 0
    /// (0: routing of 6D event on INT2 disabled; 1: routing of 6D event on INT2 enabled)
    pub six_degrees_event: bool,
    /// Routing of tilt event on INT2. Default value: 0
    /// (0: routing of tilt event on INT2 disabled; 1: routing of tilt event on INT2 enabled)
    pub tilt_event: bool,
    /// Routing of soft-iron/hard-iron algorithm end event on INT2. Default value: 0
    /// (0: routing of soft-iron/hard-iron algorithm end event on INT2 disabled;
    /// 1: routing of soft-iron/hard-iron algorithm end event on INT2 enabled)
    pub soft_hard_iron_algorithm_end_event: bool,
}

impl Md2Cfg {
    pub fn address(&self) -> u8 {
        RegisterAddress::MD2_CFG.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.inactivity_event {
            value |= 1 << 7;
        }

        if self.single_tap_event {
            value |= 1 << 6;
        }

        if self.wake_up_event {
            value |= 1 << 5;
        }

        if self.free_fall_event {
            value |= 1 << 4;
        }

        if self.double_tap_event {
            value |= 1 << 3;
        }

        if self.six_degrees_event {
            value |= 1 << 2;
        }

        if self.tilt_event {
            value |= 1 << 1;
        }

        if self.soft_hard_iron_algorithm_end_event {
            value |= 1 << 0;
        }

        value
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
