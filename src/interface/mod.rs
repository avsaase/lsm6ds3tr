pub mod i2c;
pub mod spi;
pub use self::i2c::I2cInterface;
pub use self::spi::SpiInterface;

/// Communication interface
pub trait Interface {
    type Error;

    async fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;

    async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
