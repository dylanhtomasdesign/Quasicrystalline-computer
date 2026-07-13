//! Interrupt handlers
//! 
//! Handles timer interrupts for spectral scheduling

use crate::interrupts::apic::signal_eoi;

/// Saved CPU context on interrupt
#[repr(C)]
pub struct CpuContext {
    pub r15: usize,
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rbp: usize,
    pub rdi: usize,
    pub rsi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
    pub rsp: usize,
    pub ss: usize,
}

/// Timer interrupt handler called from assembly stub
/// 
/// # Safety
/// Called from assembly interrupt stub with valid context
#[no_mangle]
pub unsafe extern "C" fn apic_timer_handler(_context: *mut CpuContext) {
    // 1. Run spectral scheduler to pick next process
    // 2. Refresh video display if enabled
    // 3. Send EOI to acknowledge interrupt
    signal_eoi();
}

// Assembly stub would be in interrupts.asm:
// global timer_interrupt_stub
// extern apic_timer_handler
//
// timer_interrupt_stub:
//     push rax, rcx, rdx, rsi, rdi, rbx
//     push rbp, r12, r13, r14, r15
//     mov rdi, rsp
//     call apic_timer_handler
//     pop r15, r14, r13, r12, rbp
//     pop rbx, rdi, rsi, rdx, rcx, rax
//     iretq
