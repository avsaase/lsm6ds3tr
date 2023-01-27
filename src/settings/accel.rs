#![allow(non_camel_case_types)]

pub struct AccelSettings {
    pub enable_x: bool,
    pub enable_y: bool,
    pub enable_z: bool,
    pub sample_rate: ODR,
    pub scale: Scale,
    pub bandwidth_selection: BandwidthSelection,
    pub bandwidth: Bandwidth,
    pub high_res_bandwidth: HighRes,
}

impl Default for AccelSettings {
    fn default() -> Self {
        AccelSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            sample_rate: ODR::_119Hz,
            scale: Scale::_2G,
            bandwidth_selection: BandwidthSelection::ByODR,
            bandwidth: Bandwidth::_408Hz,
            high_res_bandwidth: HighRes::Disabled,
        }
    }
}

impl AccelSettings {
    pub fn ctrl_reg5_xl(&self) -> u8 {
        let mut result = 0_u8;
        if self.enable_z {
            result |= 1 << 5;
        }
        if self.enable_y {
            result |= 1 << 4;
        }
        if self.enable_x {
            result |= 1 << 3;
        }
        result
    }

    pub fn ctrl_reg6_xl(&self) -> u8 {
        self.sample_rate.value()
            | self.scale.value()
            | self.bandwidth_selection.value()
            | self.bandwidth.value()
    }

    pub fn ctrl_reg7_xl(&self) -> u8 {
        self.high_res_bandwidth.value()
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
        (self as u8) << 3
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
    PowerDown = 0b000,
    _10Hz = 0b001,
    _50Hz = 0b010,
    _119Hz = 0b011,
    _238Hz = 0b100,
    _476Hz = 0b101,
    _952Hz = 0b110,
}

impl ODR {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}

#[derive(Debug)]
pub enum BandwidthSelection {
    ByODR,
    ByBW,
}

impl BandwidthSelection {
    pub fn value(&self) -> u8 {
        match self {
            BandwidthSelection::ByODR => 0 << 2,
            BandwidthSelection::ByBW => 1 << 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    _408Hz = 0b00,
    _211Hz = 0b01,
    _105Hz = 0b10,
    _50Hz = 0b11,
}

impl Bandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HighRes {
    Disabled = 0b000,
    ODR_50 = 0b100,
    ODR_100 = 0b101,
    ODR_9 = 0b110,
    ODR_400 = 0b111,
}

impl HighRes {
    pub fn value(self) -> u8 {
        (self as u8) << 5
    }
}
