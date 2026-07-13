//! Quantum Processor Interface

/// Backend type
#[derive(Debug, Clone, Copy)]
pub enum QuantumBackend { Simulation, IBMQ, Rigetti, IonQ }

/// Quantum processor
pub struct QuantumProcessor {
    pub backend: QuantumBackend,
    pub num_qubits: usize,
}

impl QuantumProcessor {
    pub fn new(backend: QuantumBackend, num_qubits: usize) -> Self {
        QuantumProcessor { backend, num_qubits }
    }
}

/// Bridge between quantum and quasicrystalline
pub struct QuantumGeometricBridge {
    pub processor: QuantumProcessor,
}

impl QuantumGeometricBridge {
    pub fn new() -> Self {
        QuantumGeometricBridge {
            processor: QuantumProcessor::new(QuantumBackend::Simulation, 16),
        }
    }
}
