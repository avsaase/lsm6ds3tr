# LSM6DS3TR-C Rust

[![Crates.io](https://img.shields.io/crates/v/lsm6ds3tr)](https://crates.io/crates/lsm6ds3tr)
[![Docs](https://docs.rs/lsm6ds3tr/badge.svg)](https://docs.rs/lsm6ds3tr/latest/lsm6ds3tr/)

LSM6DS3TR-C 6-axis (DOF) IMU accelerometer & gyroscope rust driver library.

_Inspired by [LSM9DS1 rust driver](https://github.com/lonesometraveler/lsm9ds1)._

## Examples

```rust
use lsm6ds3tr::{interface::SpiInterface, AccelScale, LsmSettings, LSM6DS3TR};

//...

let spi_interface = SpiInterface::new(spi, spi_cs);
let mut imu = LSM6DS3TR::new(spi_interface).with_settings(LsmSettings::basic());
imu.init().expect("LSM6DS3TR-C initialization failure!");
if let (Ok(xyz_a), Ok(xyz_g)) = (imu.read_accel(), imu.read_gyro()) {
    dbg!(xyz_a, xyz_g);
}
```

See my [LightCube](https://gitlab.com/mtczekajlo/lightcube) project for application usages.
