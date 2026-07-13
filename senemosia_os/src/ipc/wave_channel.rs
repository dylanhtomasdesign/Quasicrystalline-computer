//! IPC Wave Channel - Inter-Process Communication via wave interference
//! 
//! Instead of traditional message queues, IPC is modeled as wave propagation.
//! Writing to a channel emits a wave, reading decodes the interference spectrum.

use crate::geometry::projection::PHI;

/// Wave buffer size
pub const WAVE_BUFFER_SIZE: usize = 256;

/// IPC channel modeled as a resonant wave medium
pub struct WaveChannel {
    pub id: usize,
    /// Carrier frequency based on φ
    pub frequency_carrier: f64,
    /// Wave buffer (shared memory)
    pub wave_buffer: [f64; WAVE_BUFFER_SIZE],
}

impl WaveChannel {
    /// Create a new wave channel with φ-based carrier frequency
    pub fn new(id: usize) -> Self {
        WaveChannel {
            id,
            frequency_carrier: PHI,
            wave_buffer: [0.0; WAVE_BUFFER_SIZE],
        }
    }

    /// Emit a wave (IPC write)
    /// Serializes data as wave phase modulation
    pub fn emit_wave(&mut self, data: &[u8], amplitude: f64) {
        let pi = libm::acos(-1.0);
        for (t, &byte) in data.iter().enumerate() {
            if t >= WAVE_BUFFER_SIZE {
                break;
            }

            // Phase from byte value
            let phase = (byte as f64) / 255.0 * 2.0 * pi;

            // Harmonic wave equation
            let signal = amplitude * libm::cos(self.frequency_carrier * (t as f64) + phase);

            // Linear superposition - waves add constructively
            self.wave_buffer[t] += signal;
        }
    }

    /// Tune and read (IPC read)
    /// Decodes wave back to data by extracting phase
    pub fn tune_and_read(&self, expected_amplitude: f64) -> [u8; WAVE_BUFFER_SIZE] {
        let mut data = [0u8; WAVE_BUFFER_SIZE];
        let pi = libm::acos(-1.0);
        let two_pi = 2.0 * pi;

        for t in 0..WAVE_BUFFER_SIZE {
            let wave_value = self.wave_buffer[t];

            // Demodulation: extract phase
            let ratio = if wave_value / expected_amplitude > 1.0 { 1.0 }
                       else if wave_value / expected_amplitude < -1.0 { -1.0 }
                       else { wave_value / expected_amplitude };
            let arg = libm::acos(ratio);

            let phase = (arg - (self.frequency_carrier * (t as f64))) % two_pi;

            // Convert phase back to byte (0-255)
            let byte_val = ((if phase < 0.0 { -phase } else { phase }) / two_pi * 255.0) as u8;
            data[t] = byte_val;
        }

        data
    }

    /// Clear the wave buffer
    pub fn clear(&mut self) {
        self.wave_buffer = [0.0; WAVE_BUFFER_SIZE];
    }
}
