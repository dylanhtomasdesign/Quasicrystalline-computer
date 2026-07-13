//! Analog Spectral Scheduler
//! Uses analog circuits to solve Laplacian eigenvalue problems via Kirchhoff's laws.

pub const DAC_LAPLACIAN_BASE: u64 = 0x6000_0000;
pub const ADC_FIEDLER_BASE: u64 = 0x6000_1000;
pub const ANALOG_MATRIX_SIZE: usize = 32;

pub struct AnalogDAC { pub base_address: u64 }

impl AnalogDAC {
    pub fn new(base: u64) -> Self { AnalogDAC { base_address: base } }
    pub unsafe fn write_channel(&mut self, channel: usize, value: u16) {
        if channel < ANALOG_MATRIX_SIZE {
            core::ptr::write_volatile((self.base_address + (channel as u64 * 2)) as *mut u16, value);
        }
    }
    pub unsafe fn ricalibra_scheduler_analogico(&mut self, matrice: &[u16; 32]) {
        for i in 0..32.min(matrice.len()) { self.write_channel(i, matrice[i]); }
    }
}

pub struct AnalogADC { pub base_address: u64 }

impl AnalogADC {
    pub fn new(base: u64) -> Self { AnalogADC { base_address: base } }
    pub unsafe fn read_channel(&self, channel: usize) -> u16 {
        if channel < ANALOG_MATRIX_SIZE {
            core::ptr::read_volatile((self.base_address + (channel as u64 * 2)) as *const u16)
        } else { 0 }
    }
    pub unsafe fn leggi_vettore_fiedler(&self) -> [f64; 32] {
        let mut result = [0.0f64; 32];
        let v_ref = 3.3;
        let max_val = 4095.0;
        for i in 0..32 {
            let raw = self.read_channel(i);
            result[i] = ((raw as f64) / max_val * v_ref) - (v_ref / 2.0);
        }
        result
    }
}

pub struct AnalogSpectralSolver { pub dac: AnalogDAC, pub adc: AnalogADC }

impl AnalogSpectralSolver {
    pub fn new() -> Self {
        AnalogSpectralSolver {
            dac: AnalogDAC::new(DAC_LAPLACIAN_BASE),
            adc: AnalogADC::new(ADC_FIEDLER_BASE),
        }
    }
    pub unsafe fn risolvi_eigenvalue_analogico(&mut self, laplacian: &[f64; 32]) -> [f64; 32] {
        let mut conductances = [0u16; 32];
        let max_g = 4095.0;
        for i in 0..32 {
            let g = ((laplacian[i] * max_g) / 10.0).max(0.0) as u16;
            conductances[i] = g.min(max_g as u16);
        }
        self.dac.ricalibra_scheduler_analogico(&conductances);
        self.adc.leggi_vettore_fiedler()
    }
    pub fn estrai_priorita_scheduler(&self, fiedler: &[f64; 32]) -> [f64; 32] {
        let mut priorities = [0.0f64; 32];
        let max_val = fiedler.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
        for i in 0..32 {
            priorities[i] = if max_val > 0.0 { fiedler[i].abs() / max_val } else { 0.0 };
        }
        priorities
    }
}
