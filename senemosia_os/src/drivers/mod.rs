//! Drivers module - hardware interface drivers
//! 
//! Contains drivers for:
//! - Video/Framebuffer output
//! - APIC timer
//! - Network (future)

pub mod video;

pub use video::Framebuffer;
