#![no_std]
#![allow(dead_code)]

pub mod consts;
pub mod interface;
pub mod measurements;
pub mod registers;
pub mod settings;

pub use crate::registers::RegisterConfig;
use crate::registers::{
    {AccelODR, AccelScale}, {GyroODR, GyroScale},
};
pub use consts::*;
pub use interface::Interface;
pub use measurements::XYZ;
pub use settings::GyroSettings;
pub use {registers::RegisterAddress, settings::AccelSettings};

#[derive(Default)]
pub struct LsmSettings {
    accel: AccelSettings,
    gyro: GyroSettings,
}

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
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            settings: Default::default(),
        }
    }

    pub fn new_enabled(interface: I) -> Self {
        Self {
            interface,
            settings: LsmSettings {
                accel: AccelSettings::new()
                    .with_sample_rate(AccelODR::_833Hz)
                    .with_scale(AccelScale::_2G),
                gyro: GyroSettings::new()
                    .with_sample_rate(GyroODR::_833Hz)
                    .with_scale(GyroScale::_250DPS),
            },
        }
    }

    pub fn is_reachable(&mut self) -> Result<bool, I::Error> {
        Ok(self.read_register(RegisterAddress::WHO_AM_I.address())? == LSM6DS3TR_ID)
    }

    pub fn begin_accel(&mut self) -> Result<(), I::Error> {
        self.write_register_config(self.settings.accel.config())?;
        Ok(())
    }

    pub fn begin_gyro(&mut self) -> Result<(), I::Error> {
        self.write_register_config(self.settings.gyro.config())?;
        Ok(())
    }

    pub fn read_accel_raw(&mut self) -> Result<XYZ<i16>, I::Error> {
        self.read_sensor_raw(RegisterAddress::OUTX_L_XL.address())
    }

    pub fn read_accel(&mut self) -> Result<XYZ<f32>, I::Error> {
        let xyz = self.read_accel_raw()?;
        let sensitivity = self.settings.accel.scale.sensitivity();
        Ok(XYZ {
            x: xyz.x as f32 * sensitivity,
            y: xyz.y as f32 * sensitivity,
            z: xyz.z as f32 * sensitivity,
        })
    }

    pub fn read_gyro_raw(&mut self) -> Result<XYZ<i16>, I::Error> {
        self.read_sensor_raw(RegisterAddress::OUTX_L_G.address())
    }

    pub fn read_gyro(&mut self) -> Result<XYZ<f32>, I::Error> {
        let xyz = self.read_gyro_raw()?;
        let sensitivity = self.settings.gyro.scale.sensitivity();
        Ok(XYZ {
            x: xyz.x as f32 * sensitivity,
            y: xyz.y as f32 * sensitivity,
            z: xyz.z as f32 * sensitivity,
        })
    }

    pub fn read_temp_raw(&mut self) -> Result<i16, I::Error> {
        let mut bytes = [0u8; 2];
        self.interface
            .read(RegisterAddress::OUT_TEMP_L.address(), &mut bytes)?;
        let temp: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        Ok(temp)
    }

    pub fn read_temp(&mut self) -> Result<f32, I::Error> {
        let temp = self.read_temp_raw()?;
        Ok(temp as f32 / TEMP_SCALE + TEMP_BIAS)
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
