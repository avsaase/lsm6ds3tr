#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
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
    MASTER_CMD_CODE = 0x60,
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

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}
