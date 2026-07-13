//! Qubit representation

use crate::geometry::PHI;

/// Complex number for quantum amplitudes
#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn zero() -> Self { Complex { re: 0.0, im: 0.0 } }
    pub fn one() -> Self { Complex { re: 1.0, im: 0.0 } }
    pub fn abs(&self) -> f64 { libm::sqrt(self.re * self.re + self.im * self.im) }
    pub fn scale(&self, f: f64) -> Self { Complex { re: self.re * f, im: self.im * f } }
}

/// Quantum state
#[derive(Debug, Clone, Copy)]
pub struct QuantumState {
    pub alpha: Complex,
    pub beta: Complex,
}

impl QuantumState {
    pub fn zero() -> Self { QuantumState { alpha: Complex::one(), beta: Complex::zero() } }
    pub fn to_6d(&self) -> [f64; 6] {
        [self.alpha.re * PHI, self.beta.re, self.alpha.im, self.beta.im, PHI * 0.618, 0.0]
    }
}

/// Single qubit
#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub id: usize,
    pub state: QuantumState,
}

impl Qubit {
    pub fn new(id: usize) -> Self { Qubit { id, state: QuantumState::zero() } }
}
