//! Quantum Computing Module
//! Interfaces with quantum hardware or simulates quantum circuits.

pub mod qubit;
pub mod interface;

pub use qubit::{Qubit, QuantumState};
pub use interface::{QuantumProcessor, QuantumBackend, QuantumGeometricBridge};
