//! Linear acceleration sensor control register 1 (r/w).

#![allow(non_camel_case_types)]

use crate::registers::RegisterConfig;
use crate::RegisterAddress;

/// Linear acceleration sensor control register 1 (r/w).
#[derive(Default)]
pub struct Ctrl1Xl {
    /// Output data rate and power mode selection. Default value: 0000 (see Table 52).
    pub sample_rate: AccelODR,
    /// Accelerometer full-scale selection. Default value: 00.
    /// (00: ±2 g; 01: ±16 g; 10: ±4 g; 11: ±8 g)
    pub scale: AccelScale,
    /// Accelerometer digital LPF (LPF1) bandwidth selection. For bandwidth selection refer to CTRL8_XL (17h).
    pub low_pass_filter: u8, // TODO
    /// Accelerometer analog chain bandwidth selection (only for accelerometer ODR ≥ 1.67 kHz).
    /// (0: BW @ 1.5 kHz; 1: BW @ 400 Hz)
    pub analog_chain_bandwidth: u8, // TODO
}

impl Ctrl1Xl {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL1_XL.address()
    }

    pub fn value(&self) -> u8 {
        self.sample_rate.value() | self.scale.value()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum AccelScale {
    #[default]
    _2G = 0b00,
    _16G = 0b01,
    _4G = 0b10,
    _8G = 0b11,
}

impl AccelScale {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }

    pub fn sensitivity(self) -> f32 {
        use AccelScale::*;
        match self {
            _2G => 0.000_061,
            _4G => 0.000_122,
            _8G => 0.000_244,
            _16G => 0.000_732,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum AccelODR {
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
    /// 1.6 Hz in low power mode; 12.5 Hz in high performance mode
    _1_6Hz_LP_or_12_5Hz_HP = 0b1011,
    // Not allowed = [0b1100..0b1111]
}

impl AccelODR {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}
