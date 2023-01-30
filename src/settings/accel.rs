#![allow(non_camel_case_types)]

pub struct AccelSettings {
    pub sample_rate: ODR,
    pub scale: Scale,
}

impl Default for AccelSettings {
    fn default() -> Self {
        AccelSettings {
            sample_rate: ODR::_833Hz,
            scale: Scale::_2G,
        }
    }
}

impl AccelSettings {
    pub fn ctrl1_xl(&self) -> u8 {
        self.sample_rate.value() | self.scale.value()
    }

    pub fn ctrl8_xl(&self) -> u8 {
        todo!()
    }

    pub fn ctrl9_xl(&self) -> u8 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    _2G = 0b00,
    _16G = 0b01,
    _4G = 0b10,
    _8G = 0b11,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }

    pub fn sensitivity(self) -> f32 {
        use Scale::*;
        match self {
            _2G => 0.000_061,
            _4G => 0.000_122,
            _8G => 0.000_244,
            _16G => 0.000_732,
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
    /// 1.6 Hz in low power mode; 12.5 Hz in high performance mode
    _1_6Hz_LP_or_12_5Hz_HP = 0b1011,
    // Not allowed = [0b1100..0b1111]
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 4
    }
}
