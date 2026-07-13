//! System call interface
//! 
//! Provides syscalls for user space applications to access
//! kernel services like memory allocation, scheduling, and IPC

use crate::geometry::projection::Point6D;
use crate::memory::allocator::QuasicrystalAllocator;

/// Syscall numbers
pub const SYS_ALLOC_NODE: usize = 0x100;
pub const SYS_GET_RESONANCE: usize = 0x101;
pub const SYS_POLL_INPUT_WAVE: usize = 0x105;
pub const SYS_YIELD_CPU: usize = 0x106;

/// System call handler
/// 
/// # Safety
/// Called from user space via syscall instruction
pub unsafe extern "C" fn sys_call_handler(
    syscall_number: usize,
    arg1: *const f64,
    _arg2: usize,
    allocator: &mut QuasicrystalAllocator,
) -> usize {
    match syscall_number {
        SYS_ALLOC_NODE => {
            if arg1.is_null() {
                return 0;
            }

            // Copy 6D coordinates from user space
            let mut coords = [0.0; 6];
            core::ptr::copy_nonoverlapping(arg1, coords.as_mut_ptr(), 6);

            let node = Point6D::new(coords);

            // Allocate geometric node
            match allocator.allocate_node(&coords) {
                Ok(address) => address,
                Err(_) => 0,
            }
        }

        SYS_GET_RESONANCE => {
            // Calculate resonance for a given address
            0
        }

        SYS_YIELD_CPU => {
            // Yield to scheduler - would trigger context switch
            1
        }

        _ => 0, // Unknown syscall
    }
}
