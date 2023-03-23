use crate::registers::Ctrl2G;
use crate::{GyroSampleRate, GyroScale, RegisterConfig};

/// Gyroscope settings
#[derive(Default)]
pub struct GyroSettings {
    pub scale: GyroScale,
    pub sample_rate: GyroSampleRate,
}

impl GyroSettings {
    pub fn new() -> Self {
        Self::default()
            .with_sample_rate(GyroSampleRate::_416Hz)
            .with_scale(GyroScale::_250DPS)
    }

    pub fn with_sample_rate(mut self, sample_rate: GyroSampleRate) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn with_scale(mut self, scale: GyroScale) -> Self {
        self.scale = scale;
        self
    }

    /// Returns gyroscope-related register config to be written
    pub fn config(&self) -> RegisterConfig {
        Ctrl2G {
            sample_rate: self.sample_rate,
            scale: self.scale,
        }
        .config()
    }
}
