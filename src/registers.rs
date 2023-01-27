#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
    /// Activity threshold register.
    ACT_THS = 0x04,
    /// Inactivity duration register.
    ACT_DUR = 0x05,
    /// Linear acceleration sensor interrupt generator configuration register.
    INT_GEN_CFG_XL = 0x06,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_X_XL = 0x07,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_Y_XL = 0x08,
    /// Linear acceleration sensor interrupt threshold register.
    INT_GEN_THS_Z_XL = 0x09,
    /// Linear acceleration sensor interrupt duration register.
    INT_GEN_DUR_XL = 0x0A,
    /// Angular rate sensor reference value register for digital high-pass filter (r/w).
    REFERENCE_G = 0x0B,
    /// INT1_A/G pin control register.
    INT1_CTRL = 0x0C,
    /// INT2_A/G pin control register.
    INT2_CTRL = 0x0D,
    /// Who_AM_I register.
    WHO_AM_I = 0x0F,
    /// Angular rate sensor Control Register 1.
    CTRL_REG1_G = 0x10,
    /// Angular rate sensor Control Register 2.
    CTRL_REG2_G = 0x11,
    /// Angular rate sensor Control Register 3.
    CTRL_REG3_G = 0x12,
    /// Angular rate sensor sign and orientation register.
    ORIENT_CFG_G = 0x13,
    /// Angular rate sensor interrupt source register.
    INT_GEN_SRC_G = 0x14,
    /// Temperature data output register (L). L and H registers together express a 16-bit word in two’s complement right-justified.
    OUT_TEMP_L = 0x15,
    /// Temperature data output register (H). L and H registers together express a 16-bit word in two’s complement right-justified.
    OUT_TEMP_H = 0x16,
    /// Status register.
    STATUS_REG_0 = 0x17,
    /// Angular rate sensor pitch axis (X) angular rate output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_X_L_G = 0x18,
    /// Angular rate sensor pitch axis (X) angular rate output register (H). The value is expressed as a 16-bit word in two’s complement.
    OUT_X_H_G = 0x19,
    /// Angular rate sensor roll axis (Y) angular rate output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_L_G = 0x1A,
    /// Angular rate sensor roll axis (Y) angular rate output register (H). The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_H_G = 0x1B,
    /// Angular rate sensor Yaw axis (Z) angular rate output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_L_G = 0x1C,
    /// Angular rate sensor Yaw axis (Z) angular rate output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_H_G = 0x1D,
    /// Control register 4.
    CTRL_REG4 = 0x1E,
    /// Linear acceleration sensor Control Register 5.
    CTRL_REG5_XL = 0x1F,
    /// Linear acceleration sensor Control Register 6.
    CTRL_REG6_XL = 0x20,
    /// Linear acceleration sensor Control Register 7.
    CTRL_REG7_XL = 0x21,
    /// Control register 8.
    CTRL_REG8 = 0x22,
    /// Control register 9.
    CTRL_REG9 = 0x23,
    /// Control register 10.
    CTRL_REG10 = 0x24,
    /// Linear acceleration sensor interrupt source register.
    INT_GEN_SRC_XL = 0x26,
    /// Status register.
    STATUS_REG_1 = 0x27,
    /// Linear acceleration sensor X-axis output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_X_L_XL = 0x28,
    /// Linear acceleration sensor X-axis output register (H). The value is expressed as a 16-bit word in two’s complement.
    OUT_X_H_XL = 0x29,
    /// Linear acceleration sensor Y-axis output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_L_XL = 0x2A,
    /// Linear acceleration sensor Y-axis output register (H). The value is expressed as a 16-bit word in two’s complement.
    OUT_Y_H_XL = 0x2B,
    /// Linear acceleration sensor Z-axis output register (L). The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_L_XL = 0x2C,
    /// Linear acceleration sensor Z-axis output register (H). The value is expressed as a 16-bit word in two’s complement.
    OUT_Z_H_XL = 0x2D,
    /// FIFO control register.
    FIFO_CTRL = 0x2E,
    /// FIFO status control register.
    FIFO_SRC = 0x2F,
    /// Angular rate sensor interrupt generator configuration register.
    INT_GEN_CFG_G = 0x30,
    /// Angular rate sensor interrupt generator threshold register (H). The value is expressed as a 15- bit word in two’s complement.
    INT_GEN_THS_XH_G = 0x31,
    /// Angular rate sensor interrupt generator threshold register (L). The value is expressed as a 15- bit word in two’s complement.
    INT_GEN_THS_XL_G = 0x32,
    /// Angular rate sensor interrupt generator threshold register (H). The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_YH_G = 0x33,
    /// Angular rate sensor interrupt generator threshold register (L). The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_YL_G = 0x34,
    /// Angular rate sensor interrupt generator threshold register (H). The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_ZH_G = 0x35,
    /// Angular rate sensor interrupt generator threshold register (L). The value is expressed as a 15-bit word in two’s complement.
    INT_GEN_THS_ZL_G = 0x36,
    /// Angular rate sensor interrupt generator duration register.
    INT_GEN_DUR_G = 0x37,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}
