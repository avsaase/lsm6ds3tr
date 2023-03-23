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
    pub low_performance_mode: bool,
}

impl LsmSettings {
    pub fn basic() -> Self {
        Self::default()
            .with_accel(AccelSettings::new())
            .with_gyro(GyroSettings::new())
    }

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

    pub fn with_low_performance_mode(mut self) -> Self {
        self.low_performance_mode = true;
        self
    }
}
