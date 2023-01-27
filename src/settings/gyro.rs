#![allow(non_camel_case_types)]

pub struct GyroSettings {
    pub enable_x: bool,
    pub enable_y: bool,
    pub enable_z: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub flip_z: bool,
    pub scale: Scale,
    pub sample_rate: ODR,
    pub bandwidth: Bandwidth,
    pub int_selection: GyroIntSelection,
    pub out_selection: GyroOutSelection,
    pub low_power_mode: LowPowerMode,
    pub hpf_mode: HpFilter,
    pub hpf_cutoff: HpFilterCutoff,
    pub latch_interrupt: LatchInterrupt,
}

impl Default for GyroSettings {
    fn default() -> Self {
        GyroSettings {
            enable_x: true,
            enable_y: true,
            enable_z: true,
            flip_x: false,
            flip_y: false,
            flip_z: false,
            scale: Scale::_245DPS,
            sample_rate: ODR::_952Hz,
            bandwidth: Bandwidth::LPF_0,
            int_selection: GyroIntSelection::SEL_0,
            out_selection: GyroOutSelection::SEL_0,
            low_power_mode: LowPowerMode::Disabled,
            hpf_mode: HpFilter::Disabled,
            hpf_cutoff: HpFilterCutoff::HPCF_1,
            latch_interrupt: LatchInterrupt::Disabled,
        }
    }
}

impl GyroSettings {
    pub fn ctrl_reg1_g(&self) -> u8 {
        self.sample_rate.value() | self.scale.value() | self.bandwidth.value()
    }

    pub fn ctrl_reg2_g(&self) -> u8 {
        self.int_selection.value() | self.out_selection.value()
    }

    pub fn ctrl_reg3_g(&self) -> u8 {
        self.low_power_mode.value() | self.hpf_mode.value() | self.hpf_cutoff.value()
    }

    pub fn ctrl_reg4(&self) -> u8 {
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
        result | self.latch_interrupt.value()
    }

    pub fn orient_cfg_g(&self) -> u8 {
        let mut result = 0_u8;
        if self.flip_x {
            result |= 1 << 5;
        }
        if self.flip_y {
            result |= 1 << 4;
        }
        if self.flip_z {
            result |= 1 << 3;
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    _245DPS = 0b00,
    _500DPS = 0b01,
    _2000DPS = 0b11,
}

impl Scale {
    pub fn value(self) -> u8 {
        (self as u8) << 3
    }

    pub fn sensitivity(self) -> f32 {
        use Scale::*;
        match self {
            _245DPS => 0.00875,
            _500DPS => 0.0175,
            _2000DPS => 0.07,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ODR {
    PowerDown = 0b000,
    _14_9Hz = 0b001,
    _59_5Hz = 0b010,
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

#[derive(Debug, Clone, Copy)]
pub enum Bandwidth {
    LPF_0 = 0b00,
    LPF_1 = 0b01,
    LPF_2 = 0b10,
    LPF_3 = 0b11,
}

impl Bandwidth {
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GyroIntSelection {
    SEL_0 = 0b00,
    SEL_1 = 0b01,
    SEL_2 = 0b10,
    SEL_3 = 0b11,
}

impl GyroIntSelection {
    pub fn value(self) -> u8 {
        (self as u8) << 2
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GyroOutSelection {
    SEL_0 = 0b00,
    SEL_1 = 0b01,
    SEL_2 = 0b10,
    SEL_3 = 0b11,
}

impl GyroOutSelection {
    pub fn value(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LowPowerMode {
    Disabled = 0,
    Enabled = 1,
}

impl LowPowerMode {
    pub fn value(self) -> u8 {
        (self as u8) << 7
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HpFilter {
    Disabled = 0,
    Enabled = 1,
}

impl HpFilter {
    pub fn value(self) -> u8 {
        (self as u8) << 6
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LatchInterrupt {
    Disabled = 0,
    Enabled = 1,
}

impl LatchInterrupt {
    pub fn value(self) -> u8 {
        (self as u8) << 1
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HpFilterCutoff {
    HPCF_1 = 0b0000,
    HPCF_2 = 0b0001,
    HPCF_3 = 0b0010,
    HPCF_4 = 0b0011,
    HPCF_5 = 0b0100,
    HPCF_6 = 0b0101,
    HPCF_7 = 0b0110,
    HPCF_8 = 0b0111,
    HPCF_9 = 0b1000,
    HPCF_10 = 0b1001,
}

impl HpFilterCutoff {
    pub fn value(self) -> u8 {
        self as u8
    }
}
