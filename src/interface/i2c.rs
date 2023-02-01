use super::Interface;
use embedded_hal::blocking::i2c::{Write, WriteRead};

const I2C_ADDRESS: u8 = 0x6A;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InterfaceE<CommE> {
    Comm(CommE),
}

pub struct I2cInterface<I2C> {
    i2c: I2C,
}

impl<I2C> I2cInterface<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

impl<I2C, CommE> Interface for I2cInterface<I2C>
where
    I2C: WriteRead<Error = CommE> + Write<Error = CommE>,
{
    type Error = InterfaceE<CommE>;

    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        self.i2c
            .write(I2C_ADDRESS, &[addr, value])
            .map_err(InterfaceE::Comm)
    }

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write_read(I2C_ADDRESS, &[addr], buffer)
            .map_err(InterfaceE::Comm)
    }
}
