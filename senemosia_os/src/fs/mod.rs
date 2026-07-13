//! File System module - Spatial Node File System (SNFS)
//! 
//! Geometric addressing where files are located by 6D coordinates
//! rather than traditional hierarchical paths

pub mod snfs;

pub use snfs::{SpatialInode, HarmonicAllocationTable, DiskController, BLOCK_SIZE, MAX_NODES};
