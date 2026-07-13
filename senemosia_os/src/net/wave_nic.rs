//! Network driver for wave-based inter-node communication
//! 
//! Sends/receives wave data over Ethernet using Layer 2 raw frames
//! EtherType 0x7000 for Senemosìa protocol

use crate::ipc::wave_channel::WAVE_BUFFER_SIZE;

/// NIC register offsets (example Intel e1000 MMIO)
pub const NIC_REG_CTRL: usize = 0x0000;
pub const NIC_REG_TBAL: usize = 0x3800;
pub const NIC_REG_TBAH: usize = 0x3804;
pub const NIC_REG_TDLEN: usize = 0x3808;
pub const NIC_REG_TDH: usize = 0x3810;
pub const NIC_REG_TDT: usize = 0x3818;

/// TX descriptor structure
#[repr(C, align(16))]
pub struct TxDescriptor {
    pub buffer_address: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

/// Wave network driver for Layer 2 Ethernet communication
pub struct WaveNetworkDriver {
    pub io_base: usize,
    pub tx_descriptors: *mut TxDescriptor,
    pub tx_tail: usize,
}

impl WaveNetworkDriver {
    /// Initialize the network driver
    /// 
    /// # Safety
    /// Requires valid MMIO base address
    pub unsafe fn init(&mut self, base_address: usize, desc_buffer_phys: u64) {
        self.io_base = base_address;
        self.tx_descriptors = desc_buffer_phys as *mut TxDescriptor;
        self.tx_tail = 0;

        // Tell NIC where TX descriptor ring is
        write_volatile((self.io_base + NIC_REG_TBAL) as *mut u32, desc_buffer_phys as u32);
        write_volatile((self.io_base + NIC_REG_TBAH) as *mut u32, (desc_buffer_phys >> 32) as u32);

        // Set ring length (64 descriptors * 16 bytes)
        write_volatile((self.io_base + NIC_REG_TDLEN) as *mut u32, 64 * 16);

        // Reset head and tail
        write_volatile((self.io_base + NIC_REG_TDH) as *mut u32, 0);
        write_volatile((self.io_base + NIC_REG_TDT) as *mut u32, 0);

        // Enable TX (Bit 1)
        let ctrl = read_volatile((self.io_base + NIC_REG_CTRL) as *const u32);
        write_volatile((self.io_base + NIC_REG_CTRL) as *mut u32, ctrl | (1 << 1));
    }

    /// Transmit wave data as raw Ethernet frame
    /// 
    /// # Safety
    /// Requires initialized NIC and valid buffers
    pub unsafe fn transmit_wave(&mut self, wave_data: &[f64; WAVE_BUFFER_SIZE], dest_mac: [u8; 6]) {
        let mut frame = [0u8; 2048];

        // Ethernet header (14 bytes)
        // [0..6] Destination MAC
        // [6..12] Source MAC
        // [12..14] EtherType (0x7000 = Senemosìa)
        frame[0..6].copy_from_slice(&dest_mac);
        frame[6..12].copy_from_slice(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        frame[12] = 0x70;
        frame[13] = 0x00;

        // Copy wave data as raw f64 bytes
        let payload_start = 14;
        for i in 0..WAVE_BUFFER_SIZE {
            let bytes = wave_data[i].to_ne_bytes();
            let idx = payload_start + (i * 8);
            if idx + 8 < frame.len() {
                frame[idx..idx + 8].copy_from_slice(&bytes);
            }
        }

        let frame_len = payload_start + (WAVE_BUFFER_SIZE * 8);

        // Get current TX descriptor
        let desc = &mut *self.tx_descriptors.add(self.tx_tail);
        desc.buffer_address = frame.as_ptr() as u64;
        desc.length = frame_len as u16;
        desc.cmd = (1 << 3) | (1 << 0); // RS + EOP
        desc.status = 0;

        // Advance tail - triggers DMA
        self.tx_tail = (self.tx_tail + 1) % 64;
        write_volatile((self.io_base + NIC_REG_TDT) as *mut u32, self.tx_tail as u32);
    }
}

#[inline(always)]
unsafe fn read_volatile<T>(addr: *const T) -> T {
    core::ptr::read_volatile(addr)
}

#[inline(always)]
unsafe fn write_volatile<T>(addr: *mut T, value: T) {
    core::ptr::write_volatile(addr, value);
}
