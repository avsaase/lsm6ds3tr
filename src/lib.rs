#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

mod consts;
mod data;
pub mod interface;
mod registers;
mod settings;

use consts::*;
use data::XYZ;
use interface::Interface;
use registers::{
    Ctrl3C, GyroSampleRate, GyroScale, RegisterAddress, RegisterBits, RegisterConfig, RegisterValue,
    TapSrc, WakeUpSrc,
};

pub use registers::{AccelSampleRate, AccelScale};

pub use settings::{AccelSettings, GyroSettings, IrqSettings, LsmSettings};

extern crate alloc;
use alloc::vec::Vec;

/// Device driver
pub struct LSM6DS3TR<I>
where
    I: Interface,
{
    interface: I,
    pub settings: LsmSettings,
}

impl<I> LSM6DS3TR<I>
where
    I: Interface,
{
    /// Returns uninitialized device object with default settings
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            settings: Default::default(),
        }
    }

    /// Returns uninitialized device object with provided settings
    pub fn with_settings(mut self, settings: LsmSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Initializes device with stored settings
    pub fn init(&mut self) -> Result<(), I::Error> {
        self.init_accel()?;
        self.init_gyro()?;
        self.init_irqs()?;
        Ok(())
    }

    /// Returns if device is reachable
    pub fn is_reachable(&mut self) -> Result<bool, I::Error> {
        Ok(self.read_register(RegisterAddress::WHO_AM_I.address())? == LSM6DS3TR_ID)
    }

    /// Performs a software reset
    pub fn software_reset(&mut self) -> Result<(), I::Error> {
        let ctrl3_c = Ctrl3C {
            software_reset: RegisterBits::new(1),
            ..Default::default()
        };
        self.write_register_config(ctrl3_c.config())?;
        Ok(())
    }

    /// Initializes accelerometer with stored settings
    pub fn init_accel(&mut self) -> Result<(), I::Error> {
        self.write_register_config(self.settings.accel.config())?;
        Ok(())
    }

    /// Initializes gyroscope with stored settings
    pub fn init_gyro(&mut self) -> Result<(), I::Error> {
        self.write_register_config(self.settings.gyro.config())?;
        Ok(())
    }

    /// Initializes interrupts with stored settings
    pub fn init_irqs(&mut self) -> Result<(), I::Error> {
        for config in self.settings.irq.configs() {
            self.write_register_config(config)?;
        }
        Ok(())
    }

    /// Returns accelerometer raw readings
    pub fn read_accel_raw(&mut self) -> Result<XYZ<i16>, I::Error> {
        self.read_sensor_raw(RegisterAddress::OUTX_L_XL.address())
    }

    /// Returns accelerometer scaled readings \[g]
    pub fn read_accel(&mut self) -> Result<XYZ<f32>, I::Error> {
        let xyz = self.read_accel_raw()?;
        let sensitivity = self.settings.accel.scale.sensitivity();
        Ok(XYZ {
            x: xyz.x as f32 * sensitivity,
            y: xyz.y as f32 * sensitivity,
            z: xyz.z as f32 * sensitivity,
        })
    }

    /// Returns gyroscope raw readings
    pub fn read_gyro_raw(&mut self) -> Result<XYZ<i16>, I::Error> {
        self.read_sensor_raw(RegisterAddress::OUTX_L_G.address())
    }

    /// Returns gyroscope scaled readings [°/s]
    pub fn read_gyro(&mut self) -> Result<XYZ<f32>, I::Error> {
        let xyz = self.read_gyro_raw()?;
        let sensitivity = self.settings.gyro.scale.sensitivity();
        Ok(XYZ {
            x: xyz.x as f32 * sensitivity,
            y: xyz.y as f32 * sensitivity,
            z: xyz.z as f32 * sensitivity,
        })
    }

    /// Returns temperature sensor raw reading
    pub fn read_temp_raw(&mut self) -> Result<i16, I::Error> {
        let mut bytes = [0u8; 2];
        self.interface
            .read(RegisterAddress::OUT_TEMP_L.address(), &mut bytes)?;
        let temp: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        Ok(temp)
    }

    /// Returns temperature sensor scaled reading [°C]
    pub fn read_temp(&mut self) -> Result<f32, I::Error> {
        let temp = self.read_temp_raw()?;
        Ok(temp as f32 / TEMP_SCALE + TEMP_BIAS)
    }

    /// Returns last interrupt sources
    pub fn read_interrupt_sources(&mut self) -> Result<Vec<IrqSource>, I::Error> {
        let mut wake_up_src = WakeUpSrc::default();
        let mut tap_src = TapSrc::default();
        // TODO add FUNC_SRC1 reading
        // TODO add FUNC_SRC2 reading
        wake_up_src = self.read_register(wake_up_src.address())?.into();
        tap_src = self.read_register(tap_src.address())?.into();
        Ok([wake_up_src.get_irq_sources(), tap_src.get_irq_sources()].concat())
    }

    fn read_sensor_raw(&mut self, addr: u8) -> Result<XYZ<i16>, I::Error> {
        let mut bytes = [0u8; 6];
        self.interface.read(addr, &mut bytes)?;
        let x: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        let y: i16 = (bytes[3] as i16) << 8 | bytes[2] as i16;
        let z: i16 = (bytes[5] as i16) << 8 | bytes[4] as i16;
        Ok(XYZ { x, y, z })
    }

    fn read_register(&mut self, address: u8) -> Result<u8, I::Error> {
        let mut value = [0u8];
        self.interface.read(address, &mut value)?;
        Ok(value[0])
    }

    fn write_register(&mut self, address: u8, value: u8) -> Result<(), I::Error> {
        self.interface.write(address, value)?;
        Ok(())
    }

    fn read_register_config(&mut self, address: u8) -> Result<RegisterConfig, I::Error> {
        let mut value = [0u8];
        self.interface.read(address, &mut value)?;
        let value = value[0];
        Ok(RegisterConfig { address, value })
    }

    fn write_register_config(&mut self, register_config: RegisterConfig) -> Result<(), I::Error> {
        self.write_register(register_config.address, register_config.value)?;
        Ok(())
    }
}

/// Interrupt sources
#[derive(Clone)]
pub enum IrqSource {
    FreeFall,
    Sleep,
    WakeUp,
    WakeUpOnX,
    WakeUpOnY,
    WakeUpOnZ,
    Tap,
    SingleTap,
    DoubleTap,
    TapOnX,
    TapOnY,
    TapOnZ,
}
