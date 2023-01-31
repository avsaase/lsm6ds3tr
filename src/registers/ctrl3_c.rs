//! Control register 3 (r/w).

#![allow(non_camel_case_types)]

use crate::registers::RegisterConfig;
use crate::RegisterAddress;

/// Control register 3 (r/w).
pub struct Ctrl3C {
    /// Reboot memory content. Default value: 0
    /// (0: normal mode; 1: reboot memory content)
    pub reboot_memory_content: bool,
    /// Block Data Update. Default value: 0
    /// (0: continuous update; 1: output registers not updated until MSB and LSB have been read)
    pub block_data_update: bool,
    /// Interrupt activation level. Default value: 0
    /// (0: interrupt output pads active high; 1: interrupt output pads active low)
    pub high_low_active: bool,
    /// Push-pull/open-drain selection on INT1 and INT2 pads. Default value: 0
    /// (0: push-pull mode; 1: open-drain mode)
    pub push_pull_open_drain: bool,
    /// SPI Serial Interface Mode selection. Default value: 0
    /// (0: 4-wire interface; 1: 3-wire interface).
    pub spi_interface_mode: bool,
    /// Register address automatically incremented during a multiple byte access with a serial interface (I2C or SPI). Default value: 1
    /// (0: disabled; 1: enabled)
    pub interface_auto_address_increment: bool,
    /// Big/Little Endian Data selection. Default value 0
    /// (0: data LSB @ lower address; 1: data MSB @ lower address)
    pub big_little_endian: bool,
    /// Software reset. Default value: 0
    /// (0: normal mode; 1: reset device)
    /// This bit is automatically cleared.
    pub software_reset: bool,
}

impl Default for Ctrl3C {
    fn default() -> Self {
        Self {
            interface_auto_address_increment: true,
            reboot_memory_content: false,
            block_data_update: false,
            high_low_active: false,
            push_pull_open_drain: false,
            spi_interface_mode: false,
            big_little_endian: false,
            software_reset: false,
        }
    }
}

impl Ctrl3C {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL3_C.address()
    }

    pub fn value(&self) -> u8 {
        let mut value: u8 = 0;

        if self.reboot_memory_content {
            value |= 1 << 7;
        }

        if self.block_data_update {
            value |= 1 << 6;
        }

        if self.high_low_active {
            value |= 1 << 5;
        }

        if self.push_pull_open_drain {
            value |= 1 << 4;
        }

        if self.spi_interface_mode {
            value |= 1 << 3;
        }

        if self.interface_auto_address_increment {
            value |= 1 << 2;
        }

        if self.big_little_endian {
            value |= 1 << 1;
        }

        if self.software_reset {
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
