//! Angular rate sensor control register 2 (r/w).

#![allow(non_camel_case_types)]

use crate::registers::RegisterConfig;
use crate::RegisterAddress;

use super::RegisterValue;

#[derive(Default)]
/// Angular rate sensor control register 2 (r/w).
pub struct Ctrl2G {
    /// Gyroscope output data rate selection. Default value: 0000 (Refer to Table 55)
    pub sample_rate: GyroODR,
    /// Gyroscope full-scale selection. Default value: 00 (00: 245 dps; 01: 500 dps; 10: 1000 dps; 11: 2000 dps)
    /// Gyroscope full-scale at 125 dps. Default value: 0 (0: disabled; 1: enabled)
    pub scale: GyroScale,
}

impl Ctrl2G {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL2_G.address()
    }

    pub fn value(&self) -> u8 {
        self.sample_rate.shifted() | self.scale.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum GyroScale {
    _125DPS = 0b001,
    #[default]
    _250DPS = 0b000,
    _500DPS = 0b010,
    _1000DPS = 0b100,
    _2000DPS = 0b110,
}

impl GyroScale {
    pub fn sensitivity(self) -> f32 {
        use GyroScale::*;
        match self {
            _125DPS => 0.004_375,
            _250DPS => 0.008_750,
            _500DPS => 0.017_500,
            _1000DPS => 0.035_000,
            _2000DPS => 0.070_000,
        }
    }
}

impl RegisterValue for GyroScale {
    fn shifted(&self) -> u8 {
        (*self as u8) << 1
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum GyroODR {
    /// Power Down (disabled)
    #[default]
    PowerDown = 0b0000,
    /// 12.5 Hz
    _12_5Hz = 0b0001,
    /// 26 Hz
    _26Hz = 0b0010,
    /// 52 Hz
    _52Hz = 0b0011,
    /// 104 Hz
    _104Hz = 0b0100,
    /// 208 Hz
    _208Hz = 0b0101,
    /// 416 Hz
    _416Hz = 0b0110,
    /// 833 Hz
    _833Hz = 0b0111,
    /// 1.66 kHz
    _1660Hz = 0b1000,
    /// 3.33 kHz
    _3330Hz = 0b1001,
    /// 6.66 kHz
    _6660Hz = 0b1010,
    // Not allowed = [0b1011..0b1111]
}

impl RegisterValue for GyroODR {
    fn shifted(&self) -> u8 {
        (*self as u8) << 4
    }
}
