#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum RegisterAddress {
    // RESERVED = 0x00,
    /// Embedded functions configuration register
    FUNC_CFG_ACCESS = 0x01,
    // RESERVED = [0x02..0x03],
    /// Sensor sync configuration register
    SENSOR_SYNC_TIME_FRAME = 0x04,
    /// Sensor sync configuration register
    SENSOR_SYNC_RES_RATIO = 0x05,
    /// FIFO configuration registers
    FIFO_CTRL1 = 0x06,
    /// FIFO configuration registers
    FIFO_CTRL2 = 0x07,
    /// FIFO configuration registers
    FIFO_CTRL3 = 0x08,
    /// FIFO configuration registers
    FIFO_CTRL4 = 0x09,
    /// FIFO configuration registers
    FIFO_CTRL5 = 0x0A,
    /// (no comment)
    DRDY_PULSE_CFG_G = 0x0B,
    // RESERVED = 0x0C,
    /// INT1 pin control
    INT1_CTRL = 0x0D,
    /// INT2 pin control
    INT2_CTRL = 0x0E,
    /// Who I am ID
    WHO_AM_I = 0x0F,
    /// Accelerometer and gyroscope control registers
    CTRL1_XL = 0x10,
    /// Accelerometer and gyroscope control registers
    CTRL2_G = 0x11,
    /// Accelerometer and gyroscope control registers
    CTRL3_C = 0x12,
    /// Accelerometer and gyroscope control registers
    CTRL4_C = 0x13,
    /// Accelerometer and gyroscope control registers
    CTRL5_C = 0x14,
    /// Accelerometer and gyroscope control registers
    CTRL6_C = 0x15,
    /// Accelerometer and gyroscope control registers
    CTRL7_G = 0x16,
    /// Accelerometer and gyroscope control registers
    CTRL8_XL = 0x17,
    /// Accelerometer and gyroscope control registers
    CTRL9_XL = 0x18,
    /// Accelerometer and gyroscope control registers
    CTRL10_C = 0x19,
    /// I2C master configuration register
    MASTER_CONFIG = 0x1A,
    /// Interrupt registers
    WAKE_UP_SRC = 0x1B,
    /// Interrupt registers
    TAP_SRC = 0x1C,
    /// Interrupt registers
    D6D_SRC = 0x1D,
    /// Status data register for user interface
    STATUS_REG = 0x1E,
    // RESERVED = 0x1F,
    /// Temperature output data registers
    OUT_TEMP_L = 0x20,
    /// Temperature output data registers
    OUT_TEMP_H = 0x21,
    /// Gyroscope output registers for user interface
    OUTX_L_G = 0x22,
    /// Gyroscope output registers for user interface
    OUTX_H_G = 0x23,
    /// Gyroscope output registers for user interface
    OUTY_L_G = 0x24,
    /// Gyroscope output registers for user interface
    OUTY_H_G = 0x25,
    /// Gyroscope output registers for user interface
    OUTZ_L_G = 0x26,
    /// Gyroscope output registers for user interface
    OUTZ_H_G = 0x27,
    /// Accelerometer output registers
    OUTX_L_XL = 0x28,
    /// Accelerometer output registers
    OUTX_H_XL = 0x29,
    /// Accelerometer output registers
    OUTY_L_XL = 0x2A,
    /// Accelerometer output registers
    OUTY_H_XL = 0x2B,
    /// Accelerometer output registers
    OUTZ_L_XL = 0x2C,
    /// Accelerometer output registers
    OUTZ_H_XL = 0x2D,
    /// Sensor hub output registers
    SENSORHUB1_REG = 0x2E,
    /// Sensor hub output registers
    SENSORHUB2_REG = 0x2F,
    /// Sensor hub output registers
    SENSORHUB3_REG = 0x30,
    /// Sensor hub output registers
    SENSORHUB4_REG = 0x31,
    /// Sensor hub output registers
    SENSORHUB5_REG = 0x32,
    /// Sensor hub output registers
    SENSORHUB6_REG = 0x33,
    /// Sensor hub output registers
    SENSORHUB7_REG = 0x34,
    /// Sensor hub output registers
    SENSORHUB8_REG = 0x35,
    /// Sensor hub output registers
    SENSORHUB9_REG = 0x36,
    /// Sensor hub output registers
    SENSORHUB10_REG = 0x37,
    /// Sensor hub output registers
    SENSORHUB11_REG = 0x38,
    /// Sensor hub output registers
    SENSORHUB12_REG = 0x39,
    /// FIFO status registers
    FIFO_STATUS1 = 0x3A,
    /// FIFO status registers
    FIFO_STATUS2 = 0x3B,
    /// FIFO status registers
    FIFO_STATUS3 = 0x3C,
    /// FIFO status registers
    FIFO_STATUS4 = 0x3D,
    /// FIFO data output registers
    FIFO_DATA_OUT_L = 0x3E,
    /// FIFO data output registers
    FIFO_DATA_OUT_H = 0x3F,
    /// Timestamp output registers
    TIMESTAMP0_REG = 0x40,
    /// Timestamp output registers
    TIMESTAMP1_REG = 0x41,
    /// Timestamp output registers
    TIMESTAMP2_REG = 0x42,
    // RESERVED = [0x43..0x48],
    /// Step counter timestamp registers
    STEP_TIMESTAMP_L = 0x49,
    /// Step counter timestamp registers
    STEP_TIMESTAMP_H = 0x4A,
    /// Step counter output registers
    STEP_COUNTER_L = 0x4B,
    /// Step counter output registers
    STEP_COUNTER_H = 0x4C,
    /// Sensor hub output registers
    SENSORHUB13_REG = 0x4D,
    /// Sensor hub output registers
    SENSORHUB14_REG = 0x4E,
    /// Sensor hub output registers
    SENSORHUB15_REG = 0x4F,
    /// Sensor hub output registers
    SENSORHUB16_REG = 0x50,
    /// Sensor hub output registers
    SENSORHUB17_REG = 0x51,
    /// Sensor hub output registers
    SENSORHUB18_REG = 0x52,
    /// Interrupt registers
    FUNC_SRC1 = 0x53,
    /// Interrupt registers
    FUNC_SRC2 = 0x54,
    /// Interrupt register
    WRIST_TILT_IA = 0x55,
    // RESERVED = [0x56..0x57],
    /// Interrupt registers
    TAP_CFG = 0x58,
    /// Interrupt registers
    TAP_THS_6D = 0x59,
    /// Interrupt registers
    INT_DUR2 = 0x5A,
    /// Interrupt registers
    WAKE_UP_THS = 0x5B,
    /// Interrupt registers
    WAKE_UP_DUR = 0x5C,
    /// Interrupt registers
    FREE_FALL = 0x5D,
    /// Interrupt registers
    MD1_CFG = 0x5E,
    /// Interrupt registers
    MD2_CFG = 0x5F,
    /// (no comment)
    MASTER_CMD_CODE = 0x60,
    /// (no comment)
    SENS_SYNC_SPI_ERROR_CODE = 0x61,
    // RESERVED = [0x62..0x65],
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_X_L = 0x66,
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_X_H = 0x67,
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_Y_L = 0x68,
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_Y_H = 0x69,
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_Z_L = 0x6A,
    /// External magnetometer raw data output registers
    OUT_MAG_RAW_Z_H = 0x6B,
    // RESERVED = [0x6C..0x72],
    /// Accelerometer user offset correction
    X_OFS_USR = 0x73,
    /// Accelerometer user offset correction
    Y_OFS_USR = 0x74,
    /// Accelerometer user offset correction
    Z_OFS_USR = 0x75,
    // RESERVED = [0x76..0x7F],
}

impl RegisterAddress {
    pub fn address(self) -> u8 {
        self as u8
    }
}

pub struct RegisterConfig {
    pub address: u8,
    pub value: u8,
}

#[derive(Default)]
pub struct RegisterBits<const BITS_NUM: u8, const BITS_POS: u8> {
    value: u8,
}

impl<const BITS_NUM: u8, const BITS_POS: u8> RegisterBits<BITS_NUM, BITS_POS> {
    pub fn new(value: u8) -> Self {
        Self::from(value)
    }

    pub fn new_from_reg(value: u8) -> Self {
        Self::from((value >> BITS_POS) & Self::bit_mask())
    }

    pub fn from_reg(&mut self, value: u8) {
        self.value = (value >> BITS_POS) & Self::bit_mask();
    }

    pub fn bit_mask() -> u8 {
        (1 << BITS_NUM) - 1
    }

    pub fn bit_shifted_mask() -> u8 {
        Self::bit_mask() << BITS_POS
    }
}

pub trait RegisterValue {
    fn shifted(&self) -> u8;
}

impl<const BITS_NUM: u8, const BITS_POS: u8> RegisterValue for RegisterBits<BITS_NUM, BITS_POS> {
    fn shifted(&self) -> u8 {
        (self.value & Self::bit_mask()) << BITS_POS
    }
}

impl<const BITS_NUM: u8, const BITS_POS: u8> From<u8> for RegisterBits<BITS_NUM, BITS_POS> {
    fn from(value: u8) -> Self {
        Self {
            value: value & Self::bit_mask(),
        }
    }
}

impl<const BITS_NUM: u8, const BITS_POS: u8> From<bool> for RegisterBits<BITS_NUM, BITS_POS> {
    fn from(value: bool) -> Self {
        Self::from(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::{RegisterBits, RegisterValue};

    #[test]
    fn new_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
    }

    #[test]
    fn new_1_1() {
        const BITS: u8 = 1;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
    }

    #[test]
    fn new_1_2() {
        const BITS: u8 = 1;
        const POS: u8 = 2;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
    }

    #[test]
    fn new_2_0() {
        const BITS: u8 = 2;
        const POS: u8 = 0;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b11);
    }

    #[test]
    fn new_2_1() {
        const BITS: u8 = 2;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b11);
    }

    #[test]
    fn new_2_2() {
        const BITS: u8 = 2;
        const POS: u8 = 2;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b11);
    }

    #[test]
    fn shifted_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b1);
    }

    #[test]
    fn shifted_1_1() {
        const BITS: u8 = 1;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b10);
    }

    #[test]
    fn shifted_1_2() {
        const BITS: u8 = 1;
        const POS: u8 = 2;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b100);
    }

    #[test]
    fn shifted_2_1() {
        const BITS: u8 = 2;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b110);
    }

    #[test]
    fn shifted_2_2() {
        const BITS: u8 = 2;
        const POS: u8 = 2;
        let rb = RegisterBits::<BITS, POS>::new(0b11111111);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b1100);
    }

    #[test]
    fn new_from_reg_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        let rb = RegisterBits::<BITS, POS>::new_from_reg(0b11111111);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b1);
    }

    #[test]
    fn new_from_reg_1_1() {
        const BITS: u8 = 1;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new_from_reg(0b11111111);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b10);
    }

    #[test]
    fn new_from_reg_2_0() {
        const BITS: u8 = 2;
        const POS: u8 = 0;
        let rb = RegisterBits::<BITS, POS>::new_from_reg(0b11111111);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b11);
    }

    #[test]
    fn new_from_reg_2_1() {
        const BITS: u8 = 2;
        const POS: u8 = 1;
        let rb = RegisterBits::<BITS, POS>::new_from_reg(0b11111111);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b110);
    }

    #[test]
    fn new_from_reg_2_2() {
        const BITS: u8 = 2;
        const POS: u8 = 2;
        let rb = RegisterBits::<BITS, POS>::new_from_reg(0b11111111);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b1100);
    }

    #[test]
    fn from_reg_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        let mut rb = RegisterBits::<BITS, POS>::default();
        let reg: u8 = 0b11111111;
        rb.from_reg(reg);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b1);
    }

    #[test]
    fn from_reg_1_1() {
        const BITS: u8 = 1;
        const POS: u8 = 1;
        let mut rb = RegisterBits::<BITS, POS>::default();
        let reg: u8 = 0b11111111;
        rb.from_reg(reg);
        assert_eq!(rb.value, 0b1);
        assert_eq!(rb.shifted(), 0b10);
    }

    #[test]
    fn from_reg_2_0() {
        const BITS: u8 = 2;
        const POS: u8 = 0;
        let mut rb = RegisterBits::<BITS, POS>::default();
        let reg: u8 = 0b11111111;
        rb.from_reg(reg);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b11);
    }

    #[test]
    fn from_reg_2_1() {
        const BITS: u8 = 2;
        const POS: u8 = 1;
        let mut rb = RegisterBits::<BITS, POS>::default();
        let reg: u8 = 0b11111111;
        rb.from_reg(reg);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b110);
    }

    #[test]
    fn from_reg_2_2() {
        const BITS: u8 = 2;
        const POS: u8 = 2;
        let mut rb = RegisterBits::<BITS, POS>::default();
        let reg: u8 = 0b11111111;
        rb.from_reg(reg);
        assert_eq!(rb.value, 0b11);
        assert_eq!(rb.shifted(), 0b1100);
    }

    #[test]
    fn bit_mask_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_mask(), 0b1);
    }

    #[test]
    fn bit_mask_2_0() {
        const BITS: u8 = 2;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_mask(), 0b11);
    }

    #[test]
    fn bit_mask_3_0() {
        const BITS: u8 = 3;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_mask(), 0b111);
    }

    #[test]
    fn bit_shifted_mask_1_0() {
        const BITS: u8 = 1;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b1);
    }

    #[test]
    fn bit_shifted_mask_1_1() {
        const BITS: u8 = 1;
        const POS: u8 = 1;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b10);
    }

    #[test]
    fn bit_shifted_mask_2_0() {
        const BITS: u8 = 2;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b11);
    }

    #[test]
    fn bit_shifted_mask_2_1() {
        const BITS: u8 = 2;
        const POS: u8 = 1;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b110);
    }

    #[test]
    fn bit_shifted_mask_3_0() {
        const BITS: u8 = 3;
        const POS: u8 = 0;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b111);
    }

    #[test]
    fn bit_shifted_mask_3_1() {
        const BITS: u8 = 3;
        const POS: u8 = 1;
        assert_eq!(RegisterBits::<BITS, POS>::bit_shifted_mask(), 0b1110);
    }
}
