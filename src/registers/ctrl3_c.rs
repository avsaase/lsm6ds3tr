//! Control register 3 (r/w).

#![allow(non_camel_case_types)]

use crate::{RegisterAddress, RegisterBits, RegisterConfig, RegisterValue};

/// Control register 3 (r/w).
pub struct Ctrl3C {
    /// Reboot memory content. Default value: 0
    /// (0: normal mode; 1: reboot memory content)
    pub reboot_memory_content: RegisterBits<1, 7>,
    /// Block Data Update. Default value: 0
    /// (0: continuous update; 1: output registers not updated until MSB and LSB have been read)
    pub block_data_update: RegisterBits<1, 6>,
    /// Interrupt activation level. Default value: 0
    /// (0: interrupt output pads active high; 1: interrupt output pads active low)
    pub high_low_active: RegisterBits<1, 5>,
    /// Push-pull/open-drain selection on INT1 and INT2 pads. Default value: 0
    /// (0: push-pull mode; 1: open-drain mode)
    pub push_pull_open_drain: RegisterBits<1, 4>,
    /// SPI Serial Interface Mode selection. Default value: 0
    /// (0: 4-wire interface; 1: 3-wire interface).
    pub spi_interface_mode: RegisterBits<1, 3>,
    /// Register address automatically incremented during a multiple byte access with a serial interface (I2C or SPI). Default value: 1
    /// (0: disabled; 1: enabled)
    pub interface_auto_address_increment: RegisterBits<1, 2>,
    /// Big/Little Endian Data selection. Default value 0
    /// (0: data LSB @ lower address; 1: data MSB @ lower address)
    pub big_little_endian: RegisterBits<1, 1>,
    /// Software reset. Default value: 0
    /// (0: normal mode; 1: reset device)
    /// This bit is automatically cleared.
    pub software_reset: RegisterBits<1, 0>,
}

impl Default for Ctrl3C {
    fn default() -> Self {
        Self {
            interface_auto_address_increment: RegisterBits::new(1),
            reboot_memory_content: RegisterBits::default(),
            block_data_update: RegisterBits::default(),
            high_low_active: RegisterBits::default(),
            push_pull_open_drain: RegisterBits::default(),
            spi_interface_mode: RegisterBits::default(),
            big_little_endian: RegisterBits::default(),
            software_reset: RegisterBits::default(),
        }
    }
}

impl Ctrl3C {
    pub fn address(&self) -> u8 {
        RegisterAddress::CTRL3_C.address()
    }

    pub fn value(&self) -> u8 {
        self.reboot_memory_content.shifted()
            | self.block_data_update.shifted()
            | self.high_low_active.shifted()
            | self.push_pull_open_drain.shifted()
            | self.spi_interface_mode.shifted()
            | self.interface_auto_address_increment.shifted()
            | self.big_little_endian.shifted()
            | self.software_reset.shifted()
    }

    pub fn config(&self) -> RegisterConfig {
        RegisterConfig {
            address: self.address(),
            value: self.value(),
        }
    }
}
