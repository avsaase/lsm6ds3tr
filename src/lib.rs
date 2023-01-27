#![no_std]
#![allow(dead_code)]

pub mod consts;
pub mod interface;
pub mod measurements;
pub mod registers;
pub mod settings;

pub use interface::Interface;
pub use measurements::XYZ;
pub use registers::Register;
pub use settings::accel::AccelSettings;
pub use settings::gyro::GyroSettings;

#[derive(Default)]
pub struct ImuSettings {
    accel: AccelSettings,
    gyro: GyroSettings,
}

pub struct LSM6DS3TR<I>
where
    I: Interface,
{
    interface: I,
    settings: ImuSettings,
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

    pub fn reachable(&mut self) -> Result<bool, I::Error> {
        Ok(self.read_register(Register::WHO_AM_I.addr())? == consts::LSM6DS3TR_ID)
    }

    pub fn read_accel_raw(&mut self) -> Result<XYZ<i16>, I::Error> {
        self.read_sensor_raw(Register::OUT_X_L_XL.addr())
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
        self.read_sensor_raw(Register::OUT_X_L_G.addr())
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

    fn read_sensor_raw(&mut self, addr: u8) -> Result<XYZ<i16>, I::Error> {
        let mut bytes = [0u8; 6];
        self.interface.read(addr, &mut bytes)?;
        let x: i16 = (bytes[1] as i16) << 8 | bytes[0] as i16;
        let y: i16 = (bytes[3] as i16) << 8 | bytes[2] as i16;
        let z: i16 = (bytes[5] as i16) << 8 | bytes[4] as i16;
        Ok(XYZ { x, y, z })
    }

    fn read_register(&mut self, address: u8) -> Result<u8, I::Error> {
        let mut reg_data = [0u8];
        self.interface.read(address, &mut reg_data)?;
        Ok(reg_data[0])
    }

    fn write_register(&mut self, address: u8, value: u8) -> Result<(), I::Error> {
        self.interface.write(address, value)?;
        Ok(())
    }
}
