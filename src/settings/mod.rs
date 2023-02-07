pub mod accel;
pub mod gyro;
pub mod irq;

pub use accel::*;
pub use gyro::*;
pub use irq::*;

#[derive(Default)]
pub struct LsmSettings {
    pub accel: AccelSettings,
    pub gyro: GyroSettings,
    pub irq: IrqSettings,
}
