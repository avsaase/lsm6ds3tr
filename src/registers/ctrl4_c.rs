//! Control register 4 (r/w).

#![allow(non_camel_case_types)]

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Control register 4 (r/w).
#[derive(Default)]
pub struct Ctrl4C {
    /// Extend DEN functionality to accelerometer sensor. Default value: 0
    /// (0: disabled; 1: enabled)
    pub extend_den: RegisterBits<1, 7>,
    /// Gyroscope sleep mode enable. Default value: 0
    /// (0: disabled; 1: enabled)
    pub gyroscope_sleep: RegisterBits<1, 6>,
    /// DEN DRDY signal on INT1 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub den_data_ready_int1: RegisterBits<1, 5>,
    /// All interrupt signals available on INT1 pad enable. Default value: 0
    /// (0: interrupt signals divided between INT1 and INT2 pads;
    /// 1: all interrupt signals in logic or on INT1 pad)
    pub int2_on_int1: RegisterBits<1, 4>,
    /// Configuration 1 data available enable bit. Default value: 0
    /// (0: DA timer disabled; 1: DA timer enabled)
    pub data_ready_mask: RegisterBits<1, 3>,
    /// Disable I2C interface. Default value: 0
    /// (0: both I2C and SPI enabled; 1: I2C disabled, SPI only enabled)
    pub i2c_disable: RegisterBits<1, 2>,
    /// Enable gyroscope digital LPF1. The bandwidth can be selected through
    /// FTYPE[1\:0] in FUNC_CFG_ACCESS (01h).
    /// (0: disabled; 1: enabled)
    pub gyroscope_low_pass_filter_selection: RegisterBits<1, 1>,
}

impl Ctrl4C {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL4_C.address()
    }

    pub fn value(&self) -> u8 {
        self.extend_den.shifted()
            | self.gyroscope_sleep.shifted()
            | self.den_data_ready_int1.shifted()
            | self.int2_on_int1.shifted()
            | self.data_ready_mask.shifted()
            | self.i2c_disable.shifted()
            | self.gyroscope_low_pass_filter_selection.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
