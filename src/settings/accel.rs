use crate::registers::Ctrl1Xl;
use crate::{AccelSampleRate, AccelScale, RegisterConfig};

/// Accelerometer settings
#[derive(Default)]
pub struct AccelSettings {
    pub sample_rate: AccelSampleRate,
    pub scale: AccelScale,
}

impl AccelSettings {
    pub fn new() -> Self {
        Self::default()
            .with_sample_rate(AccelSampleRate::_416Hz)
            .with_scale(AccelScale::_2G)
    }

    pub fn with_sample_rate(mut self, sample_rate: AccelSampleRate) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_scale(mut self, scale: AccelScale) -> Self {
        self.scale = scale;
        self
    }

    /// Returns accelerometer-related register config to be written
    pub fn config(&self) -> RegisterConfig {
        Ctrl1Xl {
            sample_rate: self.sample_rate,
            scale: self.scale,
            ..Default::default()
        }
        .config()
    }
}
