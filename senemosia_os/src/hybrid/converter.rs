//! Analog-Digital Converter Module
//! 
//! Interfaces between continuous analog signals and digital values.

use crate::geometry::PHI;

/// ADC (Analog to Digital Converter)
#[derive(Debug, Clone)]
pub struct ADC {
    pub resolution: usize,   // Bits (e.g., 8, 12, 16)
    pub v_ref: f64,         // Reference voltage
    pub sample_rate: f64,   // Samples per second
    pub input_value: f64,   // Current analog input
}

impl ADC {
    pub fn new(resolution: usize, v_ref: f64, sample_rate: f64) -> Self {
        ADC {
            resolution,
            v_ref,
            sample_rate,
            input_value: 0.0,
        }
    }

    /// Sample and convert analog to digital
    pub fn convert(&mut self, analog_input: f64) -> u32 {
        self.input_value = analog_input;
        
        let max_digital = (1u32 << self.resolution) - 1;
        let normalized = (analog_input / self.v_ref).max(0.0).min(1.0);
        (normalized * max_digital as f64) as u32
    }

    /// Sample with quantization noise
    pub fn sample(&mut self, analog_input: f64) -> u32 {
        // Add quantization noise based on phi
        let noise = (PHI.fract() * 0.1 - 0.05) / (1 << self.resolution) as f64;
        let noisy_input = analog_input + noise;
        self.convert(noisy_input)
    }
}

/// DAC (Digital to Analog Converter)
#[derive(Debug, Clone)]
pub struct DAC {
    pub resolution: usize,   // Bits
    pub v_ref: f64,         // Reference voltage
    pub output_value: f64,   // Current analog output
}

impl DAC {
    pub fn new(resolution: usize, v_ref: f64) -> Self {
        DAC {
            resolution,
            v_ref,
            output_value: 0.0,
        }
    }

    /// Convert digital to analog
    pub fn convert(&mut self, digital_input: u32) -> f64 {
        let max_digital = (1u32 << self.resolution) - 1;
        let normalized = digital_input as f64 / max_digital as f64;
        self.output_value = normalized * self.v_ref;
        self.output_value
    }

    /// Apply to voltage source
    pub fn voltage(&self) -> f64 {
        self.output_value
    }
}

/// Sample and Hold circuit
#[derive(Debug, Clone)]
pub struct SampleHold {
    pub held_value: f64,
    pub sampling: bool,
}

impl SampleHold {
    pub fn new() -> Self {
        SampleHold {
            held_value: 0.0,
            sampling: false,
        }
    }

    pub fn sample(&mut self, input: f64) {
        if self.sampling {
            self.held_value = input;
        }
    }

    pub fn hold(&mut self) {
        self.sampling = false;
    }

    pub fn release(&mut self) {
        self.sampling = true;
    }
}

/// Continuous to discrete time converter
pub struct TimeDiscretizer {
    pub dt: f64,           // Time step
    pub current_time: f64,
}

impl TimeDiscretizer {
    pub fn new(dt: f64) -> Self {
        TimeDiscretizer {
            dt,
            current_time: 0.0,
        }
    }

    pub fn next(&mut self) -> bool {
        self.current_time += self.dt;
        true
    }

    pub fn time(&self) -> f64 {
        self.current_time
    }
}
