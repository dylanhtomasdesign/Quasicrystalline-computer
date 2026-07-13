//! OpenQASM 3.0 Generator - minimal no_std version

/// OpenQASM 3.0 circuit generator
pub struct QASMGenerator {
    pub qubits: usize,
    pub backend: QASMBackend,
}

/// Supported QPU backends
#[derive(Debug, Clone, Copy)]
pub enum QASMBackend {
    IBMQ,
    Rigetti,
    IonQ,
}

impl QASMGenerator {
    pub fn new(qubits: usize) -> Self {
        QASMGenerator { qubits, backend: QASMBackend::IBMQ }
    }
}

pub struct OpenQASM3 { pub gen: QASMGenerator }
impl OpenQASM3 { pub fn new(q: usize) -> Self { OpenQASM3 { gen: QASMGenerator::new(q) } } }
