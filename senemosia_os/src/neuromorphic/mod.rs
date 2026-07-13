//! Neuromorphic Module - Memristor-based in-memory computing

pub mod memristor;

pub use memristor::{
    MemristorArray,
    MemristorState,
    STDPSpikeGenerator,
    MEMRISTOR_CTRL_REG,
    STDP_TIMING_REG,
};
