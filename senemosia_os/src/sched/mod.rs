//! Scheduler module with Laplacian-based spectral scheduling
//! 
//! The spectral scheduler determines process execution order by analyzing
//! the Laplacian matrix eigenvalues, maximizing overall system resonance

pub mod spectral;

pub use spectral::{SpectralScheduler, Process, ProcessState, MAX_PROCESSES};
