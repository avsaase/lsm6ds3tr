//! Function routing on INT1 register (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Function routing on INT1 register (r/w).
#[derive(Default)]
pub struct Md1Cfg {
    /// Routing on INT1 of inactivity mode. Default: 0
    /// (0: routing on INT1 of inactivity disabled; 1: routing on INT1 of inactivity enabled)
    pub inactivity_event: RegisterBits<1, 7>,
    /// Single-tap recognition routing on INT1. Default: 0
    /// (0: routing of single-tap event on INT1 disabled;
    /// 1: routing of single-tap event on INT1 enabled)
    pub single_tap_event: RegisterBits<1, 6>,
    /// Routing of wakeup event on INT1. Default value: 0
    /// (0: routing of wakeup event on INT1 disabled;
    /// 1: routing of wakeup event on INT1 enabled)
    pub wake_up_event: RegisterBits<1, 5>,
    /// Routing of free-fall event on INT1. Default value: 0
    /// (0: routing of free-fall event on INT1 disabled;
    /// 1: routing of free-fall event on INT1 enabled)
    pub free_fall_event: RegisterBits<1, 4>,
    /// Routing of tap event on INT1. Default value: 0
    /// (0: routing of double-tap event on INT1 disabled;
    /// 1: routing of double-tap event on INT1 enabled)
    pub double_tap_event: RegisterBits<1, 3>,
    /// Routing of 6D event on INT1. Default value: 0
    /// (0: routing of 6D event on INT1 disabled; 1: routing of 6D event on INT1 enabled)
    pub six_degrees_event: RegisterBits<1, 2>,
    /// Routing of tilt event on INT1. Default value: 0
    /// (0: routing of tilt event on INT1 disabled; 1: routing of tilt event on INT1 enabled)
    pub tilt_event: RegisterBits<1, 1>,
    /// Routing of end counter event of timer on INT1. Default value: 0
    /// (0: routing of end counter event of timer on INT1 disabled;
    /// 1: routing of end counter event of timer event on INT1 enabled)
    pub timer_end_counter_event: RegisterBits<1, 0>,
}

impl Md1Cfg {
    pub fn address(&self) -> u8 {
        RegisterAddress::MD1_CFG.address()
    }

    pub fn value(&self) -> u8 {
        self.inactivity_event.shifted()
            | self.single_tap_event.shifted()
            | self.wake_up_event.shifted()
            | self.free_fall_event.shifted()
            | self.double_tap_event.shifted()
            | self.six_degrees_event.shifted()
            | self.tilt_event.shifted()
            | self.timer_end_counter_event.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
