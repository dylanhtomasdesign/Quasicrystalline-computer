//! Φ-Lang Syscall Bindings for the Kernel
//! 
//! Provides safe wrappers for system calls that can be invoked
//! from Φ-Lang user space applications

use crate::geometry::projection::Point6D;

/// Standard Φ-Lang syscall numbers
pub const SYS_ALLOC_NODE: usize = 0x100;
pub const SYS_GET_RESONANCE: usize = 0x101;
pub const SYS_POLL_INPUT_WAVE: usize = 0x105;
pub const SYS_YIELD_CPU: usize = 0x106;

/// SNFS Dictionary syscalls (0x200 range)
pub const SYS_SNFS_REGISTER_NODE: usize = 0x200;
pub const SYS_SNFS_RESOLVE_NODE: usize = 0x201;
pub const SYS_SNFS_LIST_NODES: usize = 0x202;
pub const SYS_SNFS_DELETE_NODE: usize = 0x203;
pub const SYS_SNFS_UPDATE_FREQUENCY: usize = 0x204;

/// Wave IPC syscalls (0x300 range)
pub const SYS_WAVE_IPC_SEND: usize = 0x300;
pub const SYS_WAVE_IPC_RECEIVE: usize = 0x301;
pub const SYS_WAVE_IPC_CREATE_CHANNEL: usize = 0x302;

/// Maximum key length
const MAX_KEY_LEN: usize = 64;

/// Registered node in the spatial dictionary
#[derive(Debug, Clone)]
pub struct RegisteredNode {
    pub key: [u8; MAX_KEY_LEN],
    pub coords: Point6D,
    pub frequency: f64,
    pub active: bool,
}

impl RegisteredNode {
    pub fn new(key: &[u8], coords: [f64; 6], frequency: f64) -> Self {
        let mut key_bytes = [0u8; MAX_KEY_LEN];
        let len = key.len().min(MAX_KEY_LEN);
        key_bytes[..len].copy_from_slice(&key[..len]);

        RegisteredNode {
            key: key_bytes,
            coords: Point6D::new(coords),
            frequency,
            active: true,
        }
    }

    /// Compare key bytes with a byte slice
    pub fn key_matches(&self, other: &[u8]) -> bool {
        let self_len = self.key_len();
        if self_len != other.len() {
            return false;
        }
        for i in 0..self_len {
            if self.key[i] != other[i] {
                return false;
            }
        }
        true
    }

    /// Get key length (up to null terminator)
    pub fn key_len(&self) -> usize {
        let mut len = 0;
        for &b in &self.key {
            if b == 0 { break; }
            len += 1;
        }
        len
    }
}

/// Spatial Dictionary Manager - no_std compatible
pub struct SpatialDictionary {
    pub nodes: [Option<RegisteredNode>; 256],
}

impl SpatialDictionary {
    pub fn new() -> Self {
        const NONE: Option<RegisteredNode> = None;
        SpatialDictionary {
            nodes: [NONE; 256],
        }
    }

    /// Register a new node in the dictionary
    pub fn register_node(&mut self, key: &[u8], coords: [f64; 6], frequency: f64) -> Result<usize, &'static str> {
        // Find free slot
        let mut slot = None;
        for i in 0..256 {
            if self.nodes[i].is_none() {
                slot = Some(i);
                break;
            }
        }

        let idx = slot.ok_or("Dictionary full")?;

        // Check for duplicate key
        for i in 0..256 {
            if let Some(ref node) = self.nodes[i] {
                if node.key_matches(key) {
                    return Err("Key already exists");
                }
            }
        }

        // Create and store node
        self.nodes[idx] = Some(RegisteredNode::new(key, coords, frequency));

        Ok(idx)
    }

    /// Resolve a key to 6D coordinates
    pub fn resolve(&self, key: &[u8]) -> Option<[f64; 6]> {
        for i in 0..256 {
            if let Some(ref node) = self.nodes[i] {
                if node.active && node.key_matches(key) {
                    return Some(node.coords.coords);
                }
            }
        }
        None
    }

    /// Deactivate a node
    pub fn delete(&mut self, key: &[u8]) -> bool {
        for i in 0..256 {
            if let Some(ref mut node) = self.nodes[i] {
                if node.key_matches(key) {
                    node.active = false;
                    return true;
                }
            }
        }
        false
    }

    /// Get count of active nodes
    pub fn active_count(&self) -> usize {
        let mut count = 0;
        for i in 0..256 {
            if let Some(ref node) = self.nodes[i] {
                if node.active {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Default for SpatialDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate node frequency based on coordinates and φ
pub fn calculate_node_frequency(coords: &[f64; 6]) -> f64 {
    use crate::geometry::PHI;
    
    let mut sum = 0.0;
    for &c in coords {
        sum += c.abs();
    }
    let mean = sum / 6.0;
    mean * PHI
}

/// Handle SNFS dictionary syscalls - no_std compatible
pub unsafe fn handle_dictionary_syscall(
    syscall_num: usize,
    arg1: *const u8,  // Key string pointer
    arg2: *const f64, // Coordinates array
    arg3: f64,        // Frequency or extra
    dict: &mut SpatialDictionary,
) -> usize {
    match syscall_num {
        SYS_SNFS_REGISTER_NODE => {
            if arg1.is_null() || arg2.is_null() {
                return 0; // Error
            }

            // Read key string
            let mut key_len = 0usize;
            while key_len < MAX_KEY_LEN {
                let byte = core::ptr::read_volatile(arg1.add(key_len));
                if byte == 0 { break; }
                key_len += 1;
            }
            let key_bytes = core::slice::from_raw_parts(arg1, key_len);

            // Read coordinates
            let mut coords = [0.0f64; 6];
            core::ptr::copy_nonoverlapping(arg2, coords.as_mut_ptr(), 6);

            // Calculate frequency if not provided
            let freq = if arg3 > 0.0 { arg3 } else { calculate_node_frequency(&coords) };

            match dict.register_node(key_bytes, coords, freq) {
                Ok(idx) => idx + 1, // Return 1-based index (0 = error)
                Err(_) => 0,
            }
        }

        SYS_SNFS_RESOLVE_NODE => {
            if arg1.is_null() {
                return 0;
            }

            // Read key string
            let mut key_len = 0usize;
            while key_len < MAX_KEY_LEN {
                let byte = core::ptr::read_volatile(arg1.add(key_len));
                if byte == 0 { break; }
                key_len += 1;
            }
            let key_bytes = core::slice::from_raw_parts(arg1, key_len);

            match dict.resolve(key_bytes) {
                Some(coords) => {
                    // Copy coordinates to user space
                    if !arg2.is_null() {
                        core::ptr::copy_nonoverlapping(coords.as_ptr(), arg2 as *mut f64, 6);
                    }
                    1 // Success
                }
                None => 0, // Not found
            }
        }

        SYS_SNFS_DELETE_NODE => {
            if arg1.is_null() {
                return 0;
            }

            let mut key_len = 0usize;
            while key_len < MAX_KEY_LEN {
                let byte = core::ptr::read_volatile(arg1.add(key_len));
                if byte == 0 { break; }
                key_len += 1;
            }
            let key_bytes = core::slice::from_raw_parts(arg1, key_len);

            if dict.delete(key_bytes) { 1 } else { 0 }
        }

        _ => 0, // Unknown syscall
    }
}
