use crate::{
    data::XYZ,
    registers::{
        FreeFall, FreeFallThreshold, InactivityGyroMode, Int1Ctrl, Int2Ctrl, IntDur2, Md1Cfg,
        Md2Cfg, RegisterConfig, TapCfg, TapThs6d, WakeUpDur, WakeUpThs,
    },
};

/// Interrupt pin/pad routing selection
#[derive(Default, Clone, Copy)]
pub enum InterruptRoute {
    #[default]
    None,
    Int1,
    Int2,
    Both,
}

#[derive(Default, Clone, Copy)]
pub enum TapRecognitionMode {
    #[default]
    None,
    Single,
    Double,
    Both,
}

#[derive(Default, Clone, Copy)]
pub struct FreeFallIrqSettings {
    pub interrupt_route: InterruptRoute,
    pub threshold: FreeFallThreshold,
    pub duration_samples: u8,
}

#[derive(Default)]
pub struct WakeUpIrqSettings {
    pub interrupt_route: InterruptRoute,
    pub threshold: u8,
}

#[derive(Default)]
pub struct OrientationDetectionIrqSettings {
    pub interrupt_route: InterruptRoute,
    // TODO
}

#[derive(Default)]
pub struct TapIrqSettings {
    pub interrupt_route: InterruptRoute,
    pub recognition_mode: TapRecognitionMode,
    pub direction_enable: XYZ<bool>,
    pub threshold: u8,
    pub shock_samples: u8,
    pub quiet_samples: u8,
    pub duration_samples: u8,
}

#[derive(Default)]
pub struct InactivityIrqSettings {
    pub interrupt_route: InterruptRoute,
    pub inactivity_gyro_mode: InactivityGyroMode,
    pub threshold: u8,
    pub sleep_duration: u8,
}

/// Interrupt settings
#[derive(Default)]
pub struct IrqSettings {
    pub free_fall: FreeFallIrqSettings,
    pub wake_up: WakeUpIrqSettings,
    pub orientation_detection: OrientationDetectionIrqSettings,
    pub tap: TapIrqSettings,
    pub inactivity: InactivityIrqSettings,
    registers: IrqRegisters,
}

#[derive(Default)]
struct IrqRegisters {
    int1_ctrl: Int1Ctrl,
    int2_ctrl: Int2Ctrl,
    md1_cfg: Md1Cfg,
    md2_cfg: Md2Cfg,
    tap_cfg: TapCfg,
    tap_ths_6d: TapThs6d,
    free_fall: FreeFall,
    wake_up_dur: WakeUpDur,
    wake_up_ths: WakeUpThs,
    int_dur2: IntDur2,
}

impl IrqSettings {
    /// Enables Free Fall interrupt
    pub fn enable_free_fall_irq(
        &mut self,
        threshold: FreeFallThreshold,
        duration_samples: u8,
        interrupt_route: InterruptRoute,
        latched_irq: bool,
    ) {
        self.enable_irqs(latched_irq);

        self.free_fall.threshold = threshold;
        self.free_fall.duration_samples = duration_samples;
        self.free_fall.interrupt_route = interrupt_route;

        self.update_registers();
    }

    /// Enables Wake Up interrupt
    pub fn enable_wake_up_irq(
        &mut self,
        threshold: u8,
        interrupt_route: InterruptRoute,
        latched_irq: bool,
    ) {
        self.enable_irqs(latched_irq);

        self.wake_up.threshold = threshold;
        self.wake_up.interrupt_route = interrupt_route;

        self.update_registers();
    }

    // TODO provide helper function calculating desired tap time to register values
    /// Enables Single/Double Tap interrupt
    pub fn enable_tap_irq(
        &mut self,
        recognition_mode: TapRecognitionMode,
        direction_enable: XYZ<bool>,
        // threshold: u8,
        // shock_samples: u8,
        // quiet_samples: u8,
        // duration_samples: u8,
        interrupt_route: InterruptRoute,
        latched_irq: bool,
    ) {
        self.enable_irqs(latched_irq);

        self.tap.recognition_mode = recognition_mode;
        self.tap.direction_enable = direction_enable;
        // self.tap.threshold = threshold;
        // self.tap.shock_samples = shock_samples;
        // self.tap.quiet_samples = quiet_samples;
        // self.tap.duration_samples = duration_samples;
        self.tap.interrupt_route = interrupt_route;

        self.update_registers();
    }

    /// Enables Activity/Inactivity interrupt
    pub fn enable_inactivity_irq(
        &mut self,
        inactivity_gyro_mode: InactivityGyroMode,
        threshold: u8,
        duration_samples: u8,
        interrupt_route: InterruptRoute,
        latched_irq: bool,
    ) {
        self.enable_irqs(latched_irq);

        self.inactivity.inactivity_gyro_mode = inactivity_gyro_mode;
        self.inactivity.threshold = threshold;
        self.inactivity.sleep_duration = duration_samples;
        self.inactivity.interrupt_route = interrupt_route;

        self.update_registers();
    }

    /// Enables basic interrupts
    pub fn enable_irqs(&mut self, latched_irq: bool) {
        self.registers.tap_cfg.enable_basic_interrupts = 1.into();
        self.registers.tap_cfg.latched_interrupt = latched_irq.into();

        self.update_registers();
    }

    /// Disables basic interrupts
    pub fn disable_irqs(&mut self) {
        self.registers.tap_cfg.enable_basic_interrupts = 0.into();

        self.update_registers();
    }

    /// Returns interrupt-related registers configs to be written
    pub fn configs(&self) -> [RegisterConfig; 10] {
        [
            self.registers.int1_ctrl.config(),
            self.registers.int2_ctrl.config(),
            self.registers.int_dur2.config(),
            self.registers.md1_cfg.config(),
            self.registers.md2_cfg.config(),
            self.registers.tap_cfg.config(),
            self.registers.tap_ths_6d.config(),
            self.registers.free_fall.config(),
            self.registers.wake_up_dur.config(),
            self.registers.wake_up_ths.config(),
        ]
    }

    fn update_registers(&mut self) {
        self.update_free_fall_registers();
        self.update_wake_up_registers();
        self.update_tap_registers();
        self.update_inactivity_registers();
    }

    fn update_free_fall_registers(&mut self) {
        (
            self.registers.md1_cfg.free_fall_event,
            self.registers.md2_cfg.free_fall_event,
        ) = match self.free_fall.interrupt_route {
            InterruptRoute::None => (0.into(), 0.into()),
            InterruptRoute::Int1 => (1.into(), 0.into()),
            InterruptRoute::Int2 => (0.into(), 1.into()),
            InterruptRoute::Both => (1.into(), 1.into()),
        };
        self.registers.free_fall.threshold = self.free_fall.threshold;
        self.registers.wake_up_dur.free_fall_duration_event =
            (((self.free_fall.threshold as u8) >> 5) & 0b1).into();
        self.registers.free_fall.duration_event = self.free_fall.duration_samples.into();
    }

    fn update_wake_up_registers(&mut self) {
        (
            self.registers.md1_cfg.wake_up_event,
            self.registers.md2_cfg.wake_up_event,
        ) = match self.wake_up.interrupt_route {
            InterruptRoute::None => (0.into(), 0.into()),
            InterruptRoute::Int1 => (1.into(), 0.into()),
            InterruptRoute::Int2 => (0.into(), 1.into()),
            InterruptRoute::Both => (1.into(), 1.into()),
        };
        self.registers.wake_up_ths.wake_up_threshold = self.wake_up.threshold.into();
    }

    fn update_tap_registers(&mut self) {
        match self.tap.recognition_mode {
            TapRecognitionMode::None | TapRecognitionMode::Single => {
                self.registers.wake_up_ths.single_double_tap_enabled = 0.into()
            }
            TapRecognitionMode::Double | TapRecognitionMode::Both => {
                self.registers.wake_up_ths.single_double_tap_enabled = 1.into()
            }
        }
        match self.tap.recognition_mode {
            TapRecognitionMode::None => {
                self.registers.tap_cfg.enable_basic_interrupts = false.into()
            }
            TapRecognitionMode::Single => {
                self.registers.tap_cfg.enable_basic_interrupts = true.into();
                self.update_single_tap_registers()
            }
            TapRecognitionMode::Double => {
                self.registers.tap_cfg.enable_basic_interrupts = true.into();
                self.update_double_tap_registers()
            }
            TapRecognitionMode::Both => {
                self.registers.tap_cfg.enable_basic_interrupts = true.into();
                self.update_single_tap_registers();
                self.update_double_tap_registers();
            }
        };
        self.registers.tap_cfg.enable_basic_interrupts = 1.into();
        self.registers.tap_cfg.enable_x_direction_tap_recognition =
            self.tap.direction_enable.x.into();
        self.registers.tap_cfg.enable_y_direction_tap_recognition =
            self.tap.direction_enable.y.into();
        self.registers.tap_cfg.enable_z_direction_tap_recognition =
            self.tap.direction_enable.z.into();
        self.registers.tap_ths_6d.tap_threshold = self.tap.threshold.into();
        self.registers.int_dur2.duration = self.tap.duration_samples.into();
        self.registers.int_dur2.quiet = self.tap.quiet_samples.into();
        self.registers.int_dur2.shock = self.tap.shock_samples.into();
    }

    fn update_single_tap_registers(&mut self) {
        (
            self.registers.md1_cfg.single_tap_event,
            self.registers.md2_cfg.single_tap_event,
        ) = match self.tap.interrupt_route {
            InterruptRoute::None => (0.into(), 0.into()),
            InterruptRoute::Int1 => (1.into(), 0.into()),
            InterruptRoute::Int2 => (0.into(), 1.into()),
            InterruptRoute::Both => (1.into(), 1.into()),
        };
    }

    fn update_double_tap_registers(&mut self) {
        (
            self.registers.md1_cfg.double_tap_event,
            self.registers.md2_cfg.double_tap_event,
        ) = match self.tap.interrupt_route {
            InterruptRoute::None => (0.into(), 0.into()),
            InterruptRoute::Int1 => (1.into(), 0.into()),
            InterruptRoute::Int2 => (0.into(), 1.into()),
            InterruptRoute::Both => (1.into(), 1.into()),
        };
    }

    fn update_inactivity_registers(&mut self) {
        (
            self.registers.md1_cfg.inactivity_event,
            self.registers.md2_cfg.inactivity_event,
        ) = match self.inactivity.interrupt_route {
            InterruptRoute::None => (0.into(), 0.into()),
            InterruptRoute::Int1 => (1.into(), 0.into()),
            InterruptRoute::Int2 => (0.into(), 1.into()),
            InterruptRoute::Both => (1.into(), 1.into()),
        };
        self.registers.wake_up_ths.wake_up_threshold = self.inactivity.threshold.into();
        self.registers.wake_up_dur.sleep_duration_event = self.inactivity.sleep_duration.into();
        self.registers.tap_cfg.enable_inactivity_function = self.inactivity.inactivity_gyro_mode;
    }
}
