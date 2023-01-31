//! DataReady configuration register (r/w).

#![allow(non_camel_case_types)]

use crate::registers::RegisterConfig;
use crate::RegisterAddress;

/// DataReady configuration register (r/w).
#[derive(Default)]
pub struct DrdyPulseCfgG {
    /// Enable pulsed DataReady mode. Default value: 0
    /// (0: DataReady latched mode. Returns to 0 only after output data have been read;
    /// 1: DataReady pulsed mode. The DataReady pulses are 75 μs long.)
    pub pulsed_data_ready: bool,
    /// Wrist tilt interrupt on INT2 pad. Default value: 0
    /// (0: disabled; 1: enabled)
    pub int2_wrist_tilt: bool,
}

impl DrdyPulseCfgG {
    pub fn address(&self) -> u8 {
        RegisterAddress::DRDY_PULSE_CFG_G.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.pulsed_data_ready {
            value |= 1 << 7;
        }

        if self.int2_wrist_tilt {
            value |= 1 << 0;
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
