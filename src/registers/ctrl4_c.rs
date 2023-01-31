//! Control register 4 (r/w).

#![allow(non_camel_case_types)]

use crate::registers::RegisterConfig;
use crate::RegisterAddress;

/// Control register 4 (r/w).
#[derive(Default)]
pub struct Ctrl4C {
    /// Extend DEN functionality to accelerometer sensor. Default value: 0
    /// (0: disabled; 1: enabled)
    pub extend_den: bool,
    /// Gyroscope sleep mode enable. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_sleep: bool,
    /// DEN DRDY signal on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub den_data_ready_int1: bool,
    /// All interrupt signals available on INT1 pad enable. Default value: 0
    /// (0: interrupt signals divided between INT1 and INT2 pads;
    /// 1: all interrupt signals in logic or on INT1 pad)
    pub int2_on_int1: bool,
    /// Configuration 1 data available enable bit. Default value: 0
    /// (0: DA timer disabled; 1: DA timer enabled)
    pub data_ready_mask: bool,
    /// Disable I2C interface. Default value: 0
    /// (0: both I2C and SPI enabled; 1: I2C disabled, SPI only enabled)
    pub i2c_disable: bool,
    /// Enable gyroscope digital LPF1. The bandwidth can be selected through
    /// FTYPE[1\:0] in FUNC_CFG_ACCESS (01h).
    /// (0: disabled; 1: enabled)
    pub gyroscope_low_pass_filter_selection: bool,
}

impl Ctrl4C {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL4_C.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.extend_den {
            value |= 1 << 7;
        }

        if self.gyroscope_sleep {
            value |= 1 << 6;
        }

        if self.den_data_ready_int1 {
            value |= 1 << 5;
        }

        if self.int2_on_int1 {
            value |= 1 << 4;
        }

        if self.data_ready_mask {
            value |= 1 << 3;
        }

        if self.i2c_disable {
            value |= 1 << 2;
        }

        if self.gyroscope_low_pass_filter_selection {
            value |= 1 << 1;
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
