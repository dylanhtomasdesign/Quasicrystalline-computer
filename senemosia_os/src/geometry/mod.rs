//! Geometry module - 6D→3D projection and spatial transformations
//! 
//! Core mathematical primitives for the quasicrystalline computing architecture:
//! - 6D hyperdimensional points based on golden ratio (φ)
//! - Icosahedral projection matrices
//! - Spectral conflict resolution via 6D rotations

pub mod projection;
pub mod conflict;

pub use projection::{Point6D, Point3D, PHI, PHI_INV, PHI_SQUARED, PROJECTION_MATRIX};
pub use conflict::{SpectralConflictResolver, MAX_RECALIBRATION_ATTEMPTS};
