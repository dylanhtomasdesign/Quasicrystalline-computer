//! Quantum Circuits Module
//! 
//! Implements quantum circuits for quasicrystalline algorithms.

use super::qubit::Qubit;
use super::gates::GateSequence;

/// Maximum qubits supported
const MAX_QUBITS: usize = 32;

/// Quantum circuit - sequence of gates on qubits
#[derive(Debug, Clone)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub qubits: [Qubit; MAX_QUBITS],
    pub gate_sequence: GateSequence,
    pub depth: usize,
}

impl QuantumCircuit {
    /// Create a new circuit with n qubits
    pub fn new(num_qubits: usize) -> Self {
        let n = num_qubits.min(MAX_QUBITS);
        let mut qubits = [Qubit::new(0); MAX_QUBITS];
        for i in 0..n {
            qubits[i] = Qubit::new(i);
        }
        QuantumCircuit {
            num_qubits: n,
            qubits,
            gate_sequence: GateSequence::new(),
            depth: 0,
        }
    }

    /// Add a single-qubit gate
    pub fn gate(&mut self, qubit: usize, gate: super::qubit::QuantumGateImpl) -> &mut Self {
        if qubit < self.num_qubits {
            self.gate_sequence.add(qubit, gate);
            self.depth += 1;
        }
        self
    }

    /// Apply Hadamard to create superposition
    pub fn hadamard(&mut self, qubit: usize) -> &mut Self {
        self.gate(qubit, super::qubit::QuantumGateImpl::hadamard())
    }

    /// Apply Pauli-X (NOT)
    pub fn not_gate(&mut self, qubit: usize) -> &mut Self {
        self.gate(qubit, super::qubit::QuantumGateImpl::pauli_x())
    }

    /// Apply T gate
    pub fn t_gate(&mut self, qubit: usize) -> &mut Self {
        self.gate(qubit, super::qubit::QuantumGateImpl::t())
    }

    /// Measure all qubits
    pub fn measure(&mut self) -> [u8; MAX_QUBITS] {
        let mut result = [0u8; MAX_QUBITS];
        for i in 0..self.num_qubits {
            result[i] = self.qubits[i].state.measure();
        }
        result
    }

    /// Execute the circuit
    pub fn execute(&mut self) {
        for (qubit_idx, gate) in &self.gate_sequence.gates {
            if *qubit_idx < self.num_qubits {
                self.qubits[*qubit_idx].apply_gate(gate);
            }
        }
    }
}

/// Quantum Fourier Transform circuit
pub struct QFTCircuit {
    pub circuit: QuantumCircuit,
}

impl QFTCircuit {
    /// Create QFT circuit for n qubits
    pub fn new(num_qubits: usize) -> Self {
        let mut circuit = QuantumCircuit::new(num_qubits);
        
        for i in 0..num_qubits {
            circuit.hadamard(i);
            for j in (i + 1)..num_qubits {
                let angle = core::f64::consts::PI / (1 << (j - i));
                circuit.gate(j, super::qubit::QuantumGateImpl::rotation("z", angle));
            }
        }

        QFTCircuit { circuit }
    }

    pub fn execute(&mut self) {
        self.circuit.execute();
    }

    pub fn measure(&mut self) -> [u8; MAX_QUBITS] {
        self.circuit.measure()
    }
}

/// Phase estimation circuit for spectral analysis
pub struct PhaseEstimationCircuit {
    pub circuit: QuantumCircuit,
    pub counting_qubits: usize,
    pub eigenstate_qubits: usize,
}

impl PhaseEstimationCircuit {
    pub fn new(num_counting: usize, num_eigenstate: usize) -> Self {
        let total = num_counting + num_eigenstate;
        let mut circuit = QuantumCircuit::new(total);

        for i in 0..num_counting {
            circuit.hadamard(i);
        }

        PhaseEstimationCircuit {
            circuit,
            counting_qubits: num_counting,
            eigenstate_qubits: num_eigenstate,
        }
    }
}
