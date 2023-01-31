use crate::{registers::ctrl1_xl::*, RegisterConfig};

#[derive(Default)]
pub struct AccelSettings {
    pub sample_rate: AccelODR,
    pub scale: AccelScale,
}

impl AccelSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sample_rate(mut self, sample_rate: AccelODR) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_scale(mut self, scale: AccelScale) -> Self {
        self.scale = scale;
        self
    }

    pub fn config(&self) -> RegisterConfig {
        Ctrl1Xl {
            sample_rate: self.sample_rate,
            scale: self.scale,
            ..Default::default()
        }
        .config()
    }
}
