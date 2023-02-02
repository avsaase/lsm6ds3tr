//! Enables interrupt and inactivity functions, configuration of filtering and tap recognition functions (r/w).

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Enables interrupt and inactivity functions, configuration of filtering and tap recognition functions (r/w).
#[derive(Default)]
pub struct TapCfg {
    /// Enable basic interrupts (6D/4D, free-fall, wake-up, tap, inactivity). Default 0.
    /// (0: interrupt disabled; 1: interrupt enabled)
    pub enable_basic_interrupts: RegisterBits<1, 7>,
    /// Enable inactivity function. Default value: 00
    /// (00: disabled
    /// 01: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro does not change;
    /// 10: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to sleep mode;
    /// 11: sets accelerometer ODR to 12.5 Hz (low-power mode), gyro to power-down mode)
    pub enable_inactivity_function: InactivityFunctionMode,
    /// HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions. Refer to Figure 8. Default value: 0
    /// (0: SLOPE filter applied; 1: HPF applied)
    pub slope_fds: FilterSelected,
    /// Enable X direction in tap recognition. Default value: 0
    /// (0: X direction disabled; 1: X direction enabled)
    pub enable_x_direction_tap_recognition: RegisterBits<1, 3>,
    /// Enable Y direction in tap recognition. Default value: 0
    /// (0: Y direction disabled; 1: Y direction enabled)
    pub enable_y_direction_tap_recognition: RegisterBits<1, 2>,
    /// Enable Z direction in tap recognition. Default value: 0
    /// (0: Z direction disabled; 1: Z direction enabled)
    pub enable_z_direction_tap_recognition: RegisterBits<1, 1>,
    /// Latched Interrupt. Default value: 0
    /// (0: interrupt request not latched; 1: interrupt request latched)
    pub latched_interrupt: RegisterBits<1, 0>,
}

impl TapCfg {
    pub fn address(&self) -> u8 {
        RegisterAddress::TAP_CFG.address()
    }

    pub fn value(&self) -> u8 {
        self.enable_basic_interrupts.shifted()
            | self.enable_inactivity_function.shifted()
            | self.slope_fds.shifted()
            | self.enable_x_direction_tap_recognition.shifted()
            | self.enable_y_direction_tap_recognition.shifted()
            | self.enable_z_direction_tap_recognition.shifted()
            | self.latched_interrupt.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}

#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub enum InactivityFunctionMode {
    #[default]
    Disabled = 0b00,
    AccelLowPowerGyroUnchanged = 0b01,
    AccelLowPowerGyroSleepMode = 0b10,
    AccelLowPowerGyroPowerDown = 0b11,
}

impl RegisterValue for InactivityFunctionMode {
    fn shifted(&self) -> u8 {
        (*self as u8) << 5
    }
}

#[repr(u8)]
#[derive(Default, Clone, Copy)]
pub enum FilterSelected {
    #[default]
    SLOPE = 0b0,
    HPF = 0b1,
}

impl RegisterValue for FilterSelected {
    fn shifted(&self) -> u8 {
        (*self as u8) << 4
    }
}
