//! Neuromorphic Processor Interface

/// Neuromorphic backend
#[derive(Debug, Clone, Copy)]
pub enum NeuromorphicBackend { Loihi, TrueNorth, SpiNNaker, Simulation }

/// Configuration
#[derive(Debug, Clone, Copy)]
pub struct NeuromorphicConfig {
    pub backend: NeuromorphicBackend,
    pub num_cores: usize,
}

impl Default for NeuromorphicConfig {
    fn default() -> Self {
        NeuromorphicConfig { backend: NeuromorphicBackend::Simulation, num_cores: 128 }
    }
}

/// Bridge between neuromorphic and quasicrystalline
pub struct NeuromorphicGeometricBridge {
    pub num_neurons: usize,
}

impl NeuromorphicGeometricBridge {
    pub fn new(num_neurons: usize) -> Self {
        NeuromorphicGeometricBridge { num_neurons }
    }
}
