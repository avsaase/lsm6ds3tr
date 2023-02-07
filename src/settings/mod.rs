pub mod accel;
pub mod gyro;
pub mod irq;

pub use accel::*;
pub use gyro::*;
pub use irq::*;

/// Device settings
#[derive(Default)]
pub struct LsmSettings {
    pub accel: AccelSettings,
    pub gyro: GyroSettings,
    pub irq: IrqSettings,
}

impl LsmSettings {
    pub fn with_accel(mut self, accel_settings: AccelSettings) -> Self {
        self.accel = accel_settings;
        self
    }

    pub fn with_gyro(mut self, gyro_settings: GyroSettings) -> Self {
        self.gyro = gyro_settings;
        self
    }

    pub fn with_irq(mut self, irq_settings: IrqSettings) -> Self {
        self.irq = irq_settings;
        self
    }
}
