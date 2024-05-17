use embedded_hal_async::spi::{Operation, SpiDevice};

use super::Interface;

/// SPI communication interface
pub struct SpiInterface<SPI> {
    spi: SPI,
}

impl<SPI> SpiInterface<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

impl<SPI> Interface for SpiInterface<SPI>
where
    SPI: SpiDevice,
{
    type Error = SPI::Error;

    async fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        let bytes = [addr, value];
        self.spi.write(&bytes).await
    }

    async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .transaction(&mut [
                Operation::Write(&[0b1000_0000 | addr]),
                Operation::Read(buffer),
            ])
            .await?;
        Ok(())
    }
}
