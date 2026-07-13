//! APIC (Advanced Programmable Interrupt Controller) driver
//! 
//! Configures the local APIC timer for periodic interrupts
//! used by the spectral scheduler

/// LAPIC base address (standard location)
pub const LAPIC_BASE: usize = 0xFEE00000;

/// Timer Divide Configuration Register offset
pub const LAPIC_TDCR: usize = 0x3E0;
/// LVT Timer Register offset
pub const LAPIC_TIMER: usize = 0x320;
/// Initial Count Register offset
pub const LAPIC_TICR: usize = 0x380;
/// End of Interrupt Register offset
pub const LAPIC_EOI: usize = 0x0B0;

/// Initialize the APIC timer for periodic interrupts
/// 
/// # Safety
/// Must be called in kernel mode with valid LAPIC access
pub unsafe fn init_apic_timer(ticks: u32) {
    // Set divide configuration to 16
    write_volatile((LAPIC_BASE + LAPIC_TDCR) as *mut u32, 0x03);

    // Configure LVT Timer:
    // Bit 17 = 1 (Periodic mode)
    // Bits 0-7 = 32 (Interrupt vector)
    let timer_config: u32 = (1 << 17) | 32;
    write_volatile((LAPIC_BASE + LAPIC_TIMER) as *mut u32, timer_config);

    // Set initial count - timer starts counting down
    write_volatile((LAPIC_BASE + LAPIC_TICR) as *mut u32, ticks);
}

/// Send End of Interrupt signal to APIC
pub unsafe fn signal_eoi() {
    write_volatile((LAPIC_BASE + LAPIC_EOI) as *mut u32, 0);
}

/// Read current timer count (for calibration)
pub unsafe fn read_timer_count() -> u32 {
    read_volatile((LAPIC_BASE + LAPIC_TICR) as *const u32)
}

/// Volatile read for MMIO registers
#[inline(always)]
unsafe fn read_volatile<T>(addr: *const T) -> T {
    core::ptr::read_volatile(addr)
}

/// Volatile write for MMIO registers
#[inline(always)]
unsafe fn write_volatile<T>(addr: *mut T, value: T) {
    core::ptr::write_volatile(addr, value);
}
