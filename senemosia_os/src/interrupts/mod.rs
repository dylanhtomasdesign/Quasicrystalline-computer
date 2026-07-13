//! Interrupts module - hardware interrupt handling
//! 
//! Contains APIC timer configuration and interrupt handlers
//! for real-time spectral scheduling

pub mod apic;
pub mod handlers;

pub use apic::{init_apic_timer, signal_eoi};
pub use handlers::{CpuContext, apic_timer_handler};
