//! Memory management module with φ-based allocation
//! 
//! Features:
//! - Geometric page addressing (6D→3D→RAM)
//! - Golden ratio (φ) based memory partitioning
//! - Spectral conflict resolution for memory collisions

pub mod allocator;

pub use allocator::{QuasicrystalAllocator, GeometricPage, PageTable, PAGE_SIZE, MAX_PAGES};
