//! Φ-Lang Module - The Quasicrystalline Programming Language
//! 
//! Φ-Lang (Phi-Lang) is a domain-specific language for the quasicrystalline
//! computing architecture, featuring:
//! - Native 6D hyperdimensional coordinate support
//! - Spatial dictionary for dynamic node registration
//! - Wave-based IPC syntax
//! - φ (golden ratio) as a first-class constant

pub mod syscall_bindings;

pub use syscall_bindings::{
    SpatialDictionary,
    RegisteredNode,
    SYS_SNFS_REGISTER_NODE,
    SYS_SNFS_RESOLVE_NODE,
    SYS_SNFS_DELETE_NODE,
    SYS_WAVE_IPC_SEND,
    calculate_node_frequency,
    handle_dictionary_syscall,
};
