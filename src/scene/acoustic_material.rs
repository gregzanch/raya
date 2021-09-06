use super::super::utils::bands::octave;
use super::super::utils::math::interpolate_log;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AcousticMaterial {
    /// octave band absorption coefficients (63hz to 16000hz)
    absorption: Vec<f32>,
    frequencies: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsorptionData(Vec<[f32; 2]>);

impl AbsorptionData {
    pub fn new(absorption: Vec<[f32; 2]>) -> Self {
        Self(absorption)
    }
}

impl AcousticMaterial {
    pub fn new(absorption: Vec<f32>) -> Self {
        Self {
            absorption,
            frequencies: octave(63.0, 16000.0),
        }
    }
    pub fn from_absorption_data(data: AbsorptionData) -> Self {
        let mut absorption: Vec<f32> = vec![];
        let mut frequencies: Vec<f32> = vec![];
        for point in data.0.iter() {
            frequencies.push(point[0]);
            absorption.push(point[1]);
        }
        Self {
            absorption,
            frequencies,
        }
    }
    pub fn absorption_function(&self, frequency: f32) -> f32 {
        let mut i = 0;
        while frequency > self.frequencies[i] && i < self.frequencies.len() {
            i += 1;
        }
        if i > 0 && i < self.frequencies.len() {
            let x1 = self.frequencies[i - 1];
            let y1 = self.absorption[i - 1];
            let x2 = self.frequencies[i];
            let y2 = self.absorption[i];
            let xi = frequency;
            return interpolate_log(x1, y1, x2, y2, xi);
        } else {
            if i == 0 {
                return self.absorption[i];
            } else {
                return self.absorption[self.frequencies.len() - 1];
            }
        }
    }
}

impl Default for AcousticMaterial {
    fn default() -> AcousticMaterial {
        AcousticMaterial {
            absorption: vec![0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01],
            frequencies: octave(63.0, 16000.0),
        }
    }
}
