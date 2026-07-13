//! Hybrid Processor Interface

/// Hybrid backend
#[derive(Debug, Clone, Copy)]
pub enum HybridBackend { Simulation, FPGA, Physical }

/// Configuration
#[derive(Debug, Clone, Copy)]
pub struct HybridConfig {
    pub backend: HybridBackend,
    pub adc_resolution: usize,
}

impl Default for HybridConfig {
    fn default() -> Self {
        HybridConfig { backend: HybridBackend::Simulation, adc_resolution: 12 }
    }
}

/// Bridge between hybrid and quasicrystalline
pub struct HybridGeometricBridge {
    pub num_analog_nodes: usize,
}

impl HybridGeometricBridge {
    pub fn new() -> Self {
        HybridGeometricBridge { num_analog_nodes: 16 }
    }
}
