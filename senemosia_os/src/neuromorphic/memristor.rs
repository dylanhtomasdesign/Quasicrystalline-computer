//! Memristor Crossbar - no_std compatible

pub const MEMRISTOR_CTRL_REG: u64 = 0x5000_0000;
pub const STDP_TIMING_REG: u64 = 0x5000_0004;
pub const MAX_ARRAY_SIZE: usize = 1024;
pub const STDP_TAU: f64 = 20.0;
pub const STDP_A: f64 = 0.01;

#[derive(Debug, Clone, Copy)]
pub struct MemristorState {
    pub g: f64,
    pub max_g: f64,
    pub min_g: f64,
}

impl MemristorState {
    pub fn new(max: f64) -> Self {
        MemristorState { g: max * 0.5, max_g: max, min_g: 0.0 }
    }
    
    pub fn ltp(&mut self, dt: i32) {
        let w = libm::exp((dt as f64) / STDP_TAU) * STDP_A;
        self.g = (self.g + w * self.max_g).min(self.max_g);
    }
    
    pub fn ltd(&mut self, dt: i32) {
        let w = ((dt.abs() as f64) / STDP_TAU) * STDP_A;
        self.g = (self.g - w * self.max_g).max(self.min_g);
    }
}

pub struct MemristorArray {
    pub rows: usize,
    pub cols: usize,
    pub mem: [Option<MemristorState>; MAX_ARRAY_SIZE],
    pub count: usize,
}

impl MemristorArray {
    pub fn new(r: usize, c: usize, max_g: f64) -> Self {
        let mut arr = MemristorArray {
            rows: r, cols: c, mem: [None; MAX_ARRAY_SIZE], count: 0,
        };
        let total = (r * c).min(MAX_ARRAY_SIZE);
        for i in 0..total {
            arr.mem[i] = Some(MemristorState::new(max_g));
            arr.count = i + 1;
        }
        arr
    }

    pub unsafe fn applica_impulso_stdp(&mut self, id: usize, dt: i32) {
        if id < self.count {
            if let Some(ref mut m) = self.mem[id] {
                if dt > 0 { m.ltp(dt) } else { m.ltd(dt) }
            }
        }
    }

    pub fn compute_vmm(&self, input: &[f64]) -> [f64; 64] {
        let mut out = [0.0f64; 64];
        for j in 0..self.cols.min(64) {
            let mut sum = 0.0f64;
            for i in 0..self.rows.min(input.len()) {
                let idx = i * self.cols + j;
                if idx < self.count {
                    if let Some(ref m) = self.mem[idx] {
                        sum += input[i] * m.g;
                    }
                }
            }
            out[j] = sum;
        }
        out
    }
}

pub struct STDPSpikeGenerator { pub last: i64 }

impl STDPSpikeGenerator {
    pub fn new() -> Self { STDPSpikeGenerator { last: 0 } }
    pub fn update(&mut self, t: i64) -> i32 {
        let dt = t - self.last;
        self.last = t;
        dt as i32
    }
}
