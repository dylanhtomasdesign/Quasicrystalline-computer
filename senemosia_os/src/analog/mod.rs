//! Analog Computing Module
//! Interfaces with analog circuits for continuous computation.

pub mod scheduler;

pub use scheduler::{
    AnalogSpectralSolver,
    AnalogDAC,
    AnalogADC,
    DAC_LAPLACIAN_BASE,
    ADC_FIEDLER_BASE,
};
