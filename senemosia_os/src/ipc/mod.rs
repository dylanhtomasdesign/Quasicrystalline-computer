//! IPC module - Inter-Process Communication via wave interference
//! 
//! Processes communicate through resonant wave channels rather than
//! traditional message queues

pub mod wave_channel;

pub use wave_channel::{WaveChannel, WAVE_BUFFER_SIZE};
