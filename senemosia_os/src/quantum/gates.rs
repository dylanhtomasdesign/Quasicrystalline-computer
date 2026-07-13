//! Quantum Gates Module
//! 
//! Implements universal quantum gate set for quasicrystalline computing.

use super::qubit::{Complex, QuantumGateImpl, GateType};

/// Gate type for public API
pub type QuantumGate = QuantumGateImpl;

/// Universal gate library
pub struct GateLibrary;

impl GateLibrary {
    /// Get gate by name
    pub fn get(name: &str) -> Option<QuantumGate> {
        match name.to_lowercase().as_str() {
            "i" | "identity" => Some(QuantumGate::identity()),
            "x" | "not" | "pauli-x" => Some(QuantumGate::pauli_x()),
            "y" | "pauli-y" => Some(Self::pauli_y()),
            "z" | "pauli-z" => Some(Self::pauli_z()),
            "h" | "hadamard" => Some(QuantumGate::hadamard()),
            "s" | "phase" | "s-gate" => Some(Self::s_gate()),
            "t" | "t-gate" => Some(QuantumGate::t()),
            _ => None,
        }
    }

    /// Pauli-Y gate
    pub fn pauli_y() -> QuantumGate {
        QuantumGate {
            gate_type: GateType::PauliY,
            mat: [
                [Complex::zero(), Complex::i().mul_scalar(-1.0)],
                [Complex::i(), Complex::zero()],
            ],
            params: [0.0; 3],
        }
    }

    /// Pauli-Z gate
    pub fn pauli_z() -> QuantumGate {
        QuantumGate {
            gate_type: GateType::PauliZ,
            mat: [
                [Complex::one(), Complex::zero()],
                [Complex::zero(), Complex::one().mul_scalar(-1.0)],
            ],
            params: [0.0; 3],
        }
    }

    /// S gate (√Z = phase gate)
    pub fn s_gate() -> QuantumGate {
        let exp = Complex::new(
            libm::sqrt(2.0) / 2.0,
            libm::sqrt(2.0) / 2.0,
        );
        QuantumGate {
            gate_type: GateType::S,
            mat: [
                [Complex::one(), Complex::zero()],
                [Complex::zero(), exp],
            ],
            params: [0.0; 3],
        }
    }
}

impl Complex {
    pub fn mul_scalar(&self, scalar: f64) -> Self {
        Complex {
            re: self.re * scalar,
            im: self.im * scalar,
        }
    }
}

/// Maximum gates in sequence
const MAX_GATES: usize = 256;

/// Composite gate sequence
#[derive(Debug, Clone)]
pub struct GateSequence {
    pub gates: [(usize, QuantumGate); MAX_GATES],
    pub count: usize,
}

impl GateSequence {
    pub fn new() -> Self {
        GateSequence {
            gates: [(0, QuantumGate::identity()); MAX_GATES],
            count: 0,
        }
    }

    pub fn add(&mut self, qubit: usize, gate: QuantumGate) -> &mut Self {
        if self.count < MAX_GATES {
            self.gates[self.count] = (qubit, gate);
            self.count += 1;
        }
        self
    }

    pub fn hadamard(&mut self, qubit: usize) -> &mut Self {
        self.add(qubit, QuantumGate::hadamard())
    }

    pub fn pauli_x(&mut self, qubit: usize) -> &mut Self {
        self.add(qubit, QuantumGate::pauli_x())
    }

    pub fn t(&mut self, qubit: usize) -> &mut Self {
        self.add(qubit, QuantumGate::t())
    }
}

impl Default for GateSequence {
    fn default() -> Self {
        Self::new()
    }
}
