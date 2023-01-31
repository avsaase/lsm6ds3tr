use crate::registers::{
    FreeFall, Int1Ctrl, Int2Ctrl, Md1Cfg, Md2Cfg, RegisterConfig, TapCfg, WakeUpDur,
};

pub use crate::registers::FreeFallThreshold;

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
}

#[derive(Default)]
pub struct OrientationDetectionIrqSettings {
    pub interrupt_route: InterruptRoute,
}

#[derive(Default)]
pub struct TapIrqSettings {
    pub interrupt_route: InterruptRoute,
    pub recognition_mode: TapRecognitionMode,
}

#[derive(Default)]
pub struct ActivityIrqSettings {
    pub interrupt_route: InterruptRoute,
}

#[derive(Default)]
pub struct IrqSettings {
    pub free_fall: FreeFallIrqSettings,
    pub wake_up: WakeUpIrqSettings,
    pub orientation_detection: OrientationDetectionIrqSettings,
    pub tap: TapIrqSettings,
    pub activity: ActivityIrqSettings,
    registers: IrqRegisters,
}

#[derive(Default)]
struct IrqRegisters {
    int1_ctrl: Int1Ctrl,
    int2_ctrl: Int2Ctrl,
    md1_cfg: Md1Cfg,
    md2_cfg: Md2Cfg,
    tap_cfg: TapCfg,
    free_fall: FreeFall,
    wake_up_dur: WakeUpDur,
}

impl IrqSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_free_fall_irq(
        &mut self,
        threshold: FreeFallThreshold,
        duration_samples: u8,
        interrupt_route: InterruptRoute,
        latched_irq: bool,
    ) {
        self.enable_irqs(latched_irq);

        self.free_fall.interrupt_route = interrupt_route;
        self.free_fall.threshold = threshold;
        self.free_fall.duration_samples = duration_samples;

        self.update_registers();
    }

    pub fn enable_tap_irq(&mut self, tap_recognition_mode: TapRecognitionMode, latched_irq: bool) {
        self.enable_irqs(latched_irq);

        self.tap.recognition_mode = tap_recognition_mode;
        // TODO continue here

        self.update_registers();
    }

    pub fn enable_irqs(&mut self, latched_irq: bool) {
        self.registers.tap_cfg.enable_basic_interrupts = true;
        self.registers.tap_cfg.latched_interrupt = latched_irq;
    }

    pub fn disable_irqs(&mut self) {
        self.registers.tap_cfg.enable_basic_interrupts = false;
    }

    pub fn configs(&self) -> [RegisterConfig; 7] {
        [
            self.registers.int1_ctrl.config(),
            self.registers.int2_ctrl.config(),
            self.registers.md1_cfg.config(),
            self.registers.md2_cfg.config(),
            self.registers.tap_cfg.config(),
            self.registers.free_fall.config(),
            self.registers.wake_up_dur.config(),
        ]
    }

    fn update_registers(&mut self) {
        self.update_free_fall_registers();
        self.update_tap_registers();
    }

    fn update_free_fall_registers(&mut self) {
        (
            self.registers.md1_cfg.free_fall_event,
            self.registers.md2_cfg.free_fall_event,
        ) = match self.tap.interrupt_route {
            InterruptRoute::None => (false, false),
            InterruptRoute::Int1 => (true, false),
            InterruptRoute::Int2 => (false, true),
            InterruptRoute::Both => (true, true),
        };
        self.registers.free_fall.threshold = self.free_fall.threshold;
        self.registers.free_fall.duration_event = self.free_fall.duration_samples;
    }

    fn update_tap_registers(&mut self) {
        match self.tap.recognition_mode {
            TapRecognitionMode::Single => self.update_single_tap_registers(),
            TapRecognitionMode::Double => self.update_double_tap_registers(),
            TapRecognitionMode::Both => {
                self.update_single_tap_registers();
                self.update_double_tap_registers();
            }
        };
    }

    fn update_single_tap_registers(&mut self) {
        (
            self.registers.md1_cfg.single_tap_event,
            self.registers.md2_cfg.single_tap_event,
        ) = match self.tap.interrupt_route {
            InterruptRoute::None => (false, false),
            InterruptRoute::Int1 => (true, false),
            InterruptRoute::Int2 => (false, true),
            InterruptRoute::Both => (true, true),
        };
    }

    fn update_double_tap_registers(&mut self) {
        (
            self.registers.md1_cfg.double_tap_event,
            self.registers.md2_cfg.double_tap_event,
        ) = match self.tap.interrupt_route {
            InterruptRoute::None => (false, false),
            InterruptRoute::Int1 => (true, false),
            InterruptRoute::Int2 => (false, true),
            InterruptRoute::Both => (true, true),
        };
    }
}
