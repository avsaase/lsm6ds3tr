#![allow(non_camel_case_types)]

pub struct GyroSettings {
    pub scale: Scale,
    pub sample_rate: ODR,
}

impl Default for GyroSettings {
    fn default() -> Self {
        GyroSettings {
            scale: Scale::_250DPS,
            sample_rate: ODR::_833Hz,
        }
    }
}

impl GyroSettings {
    pub fn ctrl2_g(&self) -> u8 {
        self.sample_rate.value() | self.scale.value()
    }

    pub fn ctrl7_g(&self) -> u8 {
        todo!();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    _125DPS = 0b001,
    _250DPS = 0b000,
    _500DPS = 0b010,
    _1000DPS = 0b100,
    _2000DPS = 0b110,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 1
    }

    pub fn sensitivity(self) -> f32 {
        use Scale::*;
        match self {
            _125DPS => 0.04375,
            _250DPS => 0.0875,
            _500DPS => 0.0175,
            _1000DPS => 0.035,
            _2000DPS => 0.07,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ODR {
    /// Power Down (disabled)
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

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}
