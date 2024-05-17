use embedded_hal_async::spi::SpiDevice;

const SPI_READ: u8 = 0x80;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum InterfaceE<CommE, PinE> {
    Comm(CommE),
    Pin(PinE),
}

/// SPI communication interface
pub struct SpiInterface<SPI> {
    spi: SPI,
}

impl<SPI> SpiInterface<SPI>
where
    SPI: SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    async fn write(&mut self, addr: u8, value: u8) -> Result<(), SPI::Error> {
        let bytes = [addr, value];
        self.spi.write(&bytes).await
    }

    async fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), SPI::Error> {
        self.spi.write(&[SPI_READ | addr]).await?;
        self.spi.read(buffer).await
    }
}
