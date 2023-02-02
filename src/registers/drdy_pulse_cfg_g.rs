//! DataReady configuration register (r/w).

#![allow(non_camel_case_types)]

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// DataReady configuration register (r/w).
#[derive(Default)]
pub struct DrdyPulseCfgG {
    /// Enable pulsed DataReady mode. Default value: 0
    /// (0: DataReady latched mode. Returns to 0 only after output data have been read;
    /// 1: DataReady pulsed mode. The DataReady pulses are 75 μs long.)
    pub pulsed_data_ready: RegisterBits<1, 7>,
    /// Wrist tilt interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub int2_wrist_tilt: RegisterBits<1, 0>,
}

impl DrdyPulseCfgG {
    pub fn address(&self) -> u8 {
        RegisterAddress::DRDY_PULSE_CFG_G.address()
    }

    pub fn value(&self) -> u8 {
        self.pulsed_data_ready.shifted() | self.int2_wrist_tilt.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
