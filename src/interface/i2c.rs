use embedded_hal::i2c::I2c;

const I2C_ADDRESS: u8 = 0x6A;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum InterfaceE<CommE> {
    Comm(CommE),
}

/// I2C communication interface
pub struct I2cInterface<I2C> {
    i2c: I2C,
}

impl<I2C> I2cInterface<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }
}

impl<I2C: I2c> I2cInterface<I2C> {
    fn write(&mut self, addr: u8, value: u8) -> Result<(), I2C::Error> {
        self.i2c.write(I2C_ADDRESS, &[addr, value])
    }

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2C::Error> {
        self.i2c.write_read(I2C_ADDRESS, &[addr], buffer)
    }
}
