//! QPU Registers - MMIO interface

pub const QPU_CTRL_REG: u64 = 0x4000_0000;
pub const QPU_STATUS_REG: u64 = 0x4000_0004;
pub const QPU_CMD_REG: u64 = 0x4000_0008;
pub const QPU_DMA_ADDR_REG: u64 = 0x4000_0010;
pub const QPU_RESULT_REG: u64 = 0x4000_0018;

pub const QPU_CMD_RESET: u32 = 0x01;
pub const QPU_CMD_START: u32 = 0x02;
pub const QPU_CMD_LOAD_QASM: u32 = 0x04;
pub const QPU_CMD_EXECUTE: u32 = 0x05;

pub const QPU_STATUS_IDLE: u32 = 0x00;
pub const QPU_STATUS_RUNNING: u32 = 0x01;
pub const QPU_STATUS_DONE: u32 = 0x02;

pub struct QPURegisters { pub base: u64 }

impl QPURegisters {
    pub fn new(base: u64) -> Self { QPURegisters { base } }
    pub unsafe fn write(&self, offset: u64, val: u32) {
        core::ptr::write_volatile((self.base + offset) as *mut u32, val);
    }
    pub unsafe fn read(&self, offset: u64) -> u32 {
        core::ptr::read_volatile((self.base + offset) as *const u32)
    }
    pub unsafe fn read64(&self, offset: u64) -> u64 {
        core::ptr::read_volatile((self.base + offset) as *const u64)
    }
    pub unsafe fn write64(&self, offset: u64, val: u64) {
        core::ptr::write_volatile((self.base + offset) as *mut u64, val);
    }
}

pub struct QPUDriver { pub regs: QPURegisters }

impl QPUDriver {
    pub fn new() -> Self { QPUDriver { regs: QPURegisters::new(QPU_CTRL_REG) } }
    pub unsafe fn reset(&mut self) { self.regs.write(8, QPU_CMD_RESET); }
    pub unsafe fn load_qasm(&mut self, addr: u64) {
        self.regs.write64(QPU_DMA_ADDR_REG - QPU_CTRL_REG, addr);
        self.regs.write(8, QPU_CMD_LOAD_QASM);
    }
    pub unsafe fn execute(&mut self) { self.regs.write(8, QPU_CMD_EXECUTE); }
    pub unsafe fn wait_done(&self) { while self.regs.read(4) == QPU_STATUS_RUNNING {} }
    pub unsafe fn get_result(&self) -> u64 { self.regs.read64(QPU_RESULT_REG - QPU_CTRL_REG) }
}
