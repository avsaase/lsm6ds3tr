pub mod spi;
pub use self::spi::SpiInterface;

pub trait Interface {
    type Error;

    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
}
