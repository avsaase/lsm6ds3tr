use super::Interface;
use embedded_hal::{blocking::spi::Transfer, blocking::spi::Write, digital::v2::OutputPin};

const SPI_READ: u8 = 0x80;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum InterfaceE<CommE, PinE> {
    Comm(CommE),
    Pin(PinE),
}

/// SPI communication interface
pub struct SpiInterface<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, CommE, PinE> SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }
}

impl<SPI, CS, CommE, PinE> Interface for SpiInterface<SPI, CS>
where
    SPI: Transfer<u8, Error = CommE> + Write<u8, Error = CommE>,
    CS: OutputPin<Error = PinE>,
{
    type Error = InterfaceE<CommE, PinE>;

    fn write(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        let bytes = [addr, value];
        self.cs.set_low().map_err(InterfaceE::Pin)?;
        self.spi.write(&bytes).map_err(InterfaceE::Comm)?;
        self.cs.set_high().map_err(InterfaceE::Pin)?;
        Ok(())
    }

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().map_err(InterfaceE::Pin)?;
        self.spi
            .write(&[SPI_READ | addr])
            .map_err(InterfaceE::Comm)?;
        self.spi.transfer(buffer).map_err(InterfaceE::Comm)?;
        self.cs.set_high().map_err(InterfaceE::Pin)?;
        Ok(())
    }
}
