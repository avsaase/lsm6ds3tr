//! Function routing on INT1 register (r/w).

use crate::RegisterAddress;

/// Function routing on INT1 register (r/w).
#[derive(Default)]
pub struct Md1Cfg {
    /// Routing on INT1 of inactivity mode. Default: 0
    /// (0: routing on INT1 of inactivity disabled; 1: routing on INT1 of inactivity enabled)
    pub inactivity_event: bool,
    /// Single-tap recognition routing on INT1. Default: 0
    /// (0: routing of single-tap event on INT1 disabled;
    /// 1: routing of single-tap event on INT1 enabled)
    pub single_tap_event: bool,
    /// Routing of wakeup event on INT1. Default value: 0
    /// (0: routing of wakeup event on INT1 disabled;
    /// 1: routing of wakeup event on INT1 enabled)
    pub wake_up_event: bool,
    /// Routing of free-fall event on INT1. Default value: 0
    /// (0: routing of free-fall event on INT1 disabled;
    /// 1: routing of free-fall event on INT1 enabled)
    pub free_fall_event: bool,
    /// Routing of tap event on INT1. Default value: 0
    /// (0: routing of double-tap event on INT1 disabled;
    /// 1: routing of double-tap event on INT1 enabled)
    pub double_tap_event: bool,
    /// Routing of 6D event on INT1. Default value: 0
    /// (0: routing of 6D event on INT1 disabled; 1: routing of 6D event on INT1 enabled)
    pub d6d_event: bool,
    /// Routing of tilt event on INT1. Default value: 0
    /// (0: routing of tilt event on INT1 disabled; 1: routing of tilt event on INT1 enabled)
    pub tilt_event: bool,
    /// Routing of end counter event of timer on INT1. Default value: 0
    /// (0: routing of end counter event of timer on INT1 disabled;
    /// 1: routing of end counter event of timer event on INT1 enabled)
    pub timer_end_counter_event: bool,
}

impl Md1Cfg {
    pub fn address(self) -> u8 {
        RegisterAddress::MD1_CFG.address()
    }

    pub fn value(self) -> u8 {
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

        if self.d6d_event {
            value |= 1 << 2;
        }

        if self.tilt_event {
            value |= 1 << 1;
        }

        if self.timer_end_counter_event {
            value |= 1 << 0;
        }

        value
    }
}
