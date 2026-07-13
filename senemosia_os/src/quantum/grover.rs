//! Grover's Algorithm - minimal no_std version

use crate::geometry::{PHI, PHI_INV};

pub struct GroverOracle {
    pub target_index: usize,
    pub num_qubits: usize,
}

impl GroverOracle {
    pub fn new(target_index: usize, num_qubits: usize) -> Self {
        GroverOracle { target_index, num_qubits }
    }
}

pub struct DiffusionOperator {
    pub num_qubits: usize,
}

impl DiffusionOperator {
    pub fn new(n: usize) -> Self {
        DiffusionOperator { num_qubits: n }
    }
}

pub struct GroverSearch {
    pub num_qubits: usize,
    pub num_iterations: usize,
    pub oracle: GroverOracle,
    pub diffusion: DiffusionOperator,
}

impl GroverSearch {
    pub fn new(num_qubits: usize, target_index: usize) -> Self {
        let n = 1 << (num_qubits.saturating_sub(1));
        let iterations = ((core::f64::consts::PI / 4.0) * libm::sqrt(n as f64)) as usize;
        GroverSearch {
            num_qubits,
            num_iterations: iterations.max(1),
            oracle: GroverOracle::new(target_index, num_qubits),
            diffusion: DiffusionOperator::new(num_qubits),
        }
    }

    pub fn target_coordinates(&self) -> [f64; 6] {
        let idx = self.oracle.target_index as f64;
        [
            idx * PHI,
            idx * PHI_INV,
            idx * 0.382,
            PHI * (idx + 1.0),
            PHI_INV * (idx + 1.0),
            1.618 * idx,
        ]
    }
}
