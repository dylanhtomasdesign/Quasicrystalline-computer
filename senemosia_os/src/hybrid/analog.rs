//! Analog Circuit Simulation
//! 
//! Models analog electronic components for hybrid computing.

use crate::geometry::PHI;

/// Electronic component types
#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    Resistor,
    Capacitor,
    Inductor,
    VoltageSource,
    CurrentSource,
    OpAmp,
}

/// Base electronic component
#[derive(Debug, Clone)]
pub struct Component {
    pub id: usize,
    pub component_type: ComponentType,
    pub value: f64,  // Ohms, Farads, Henries, Volts, Amperes
    pub nodes: [usize; 2],  // Node indices
}

impl Component {
    pub fn resistor(id: usize, resistance: f64, n1: usize, n2: usize) -> Self {
        Component {
            id,
            component_type: ComponentType::Resistor,
            value: resistance,
            nodes: [n1, n2],
        }
    }

    pub fn capacitor(id: usize, capacitance: f64, n1: usize, n2: usize) -> Self {
        Component {
            id,
            component_type: ComponentType::Capacitor,
            value: capacitance,
            nodes: [n1, n2],
        }
    }

    pub fn inductor(id: usize, inductance: f64, n1: usize, n2: usize) -> Self {
        Component {
            id,
            component_type: ComponentType::Inductor,
            value: inductance,
            nodes: [n1, n2],
        }
    }

    pub fn voltage_source(id: usize, voltage: f64, n1: usize, n2: usize) -> Self {
        Component {
            id,
            component_type: ComponentType::VoltageSource,
            value: voltage,
            nodes: [n1, n2],
        }
    }
}

/// Operational Amplifier model
#[derive(Debug, Clone)]
pub struct OperationalAmplifier {
    pub id: usize,
    pub gain: f64,          // Open-loop gain
    pub input_resistance: f64,  // Ohms
    pub output_resistance: f64, // Ohms
    pub bandwidth: f64,     // Hz
    pub v_plus: f64,        // Non-inverting input voltage
    pub v_minus: f64,       // Inverting input voltage
    pub v_out: f64,         // Output voltage
}

impl OperationalAmplifier {
    pub fn new(id: usize) -> Self {
        OperationalAmplifier {
            id,
            gain: 100000.0,  // 100k open-loop gain
            input_resistance: 1e6,  // 1MΩ
            output_resistance: 100.0, // 100Ω
            bandwidth: 1e6,  // 1MHz
            v_plus: 0.0,
            v_minus: 0.0,
            v_out: 0.0,
        }
    }

    /// Update op-amp output
    pub fn update(&mut self, v_plus: f64, v_minus: f64) {
        self.v_plus = v_plus;
        self.v_minus = v_minus;
        
        let diff = v_plus - v_minus;
        self.v_out = (diff * self.gain).max(-15.0).min(15.0);  // Rail to ±15V
    }

    /// Get frequency response at given frequency
    pub fn frequency_response(&self, freq: f64) -> f64 {
        let f0 = self.bandwidth;
        self.gain / libm::sqrt(1.0 + (freq / f0).powi(2))
    }
}

/// Capacitor with state
#[derive(Debug, Clone)]
pub struct Capacitor {
    pub id: usize,
    pub capacitance: f64,  // Farads
    pub voltage: f64,      // Current voltage
    pub charge: f64,       // Stored charge
}

impl Capacitor {
    pub fn new(id: usize, capacitance: f64) -> Self {
        Capacitor {
            id,
            capacitance,
            voltage: 0.0,
            charge: 0.0,
        }
    }

    /// Update capacitor (charge/discharge)
    pub fn update(&mut self, current: f64, dt: f64) {
        self.charge += current * dt;
        self.voltage = self.charge / self.capacitance;
    }

    /// Get energy stored
    pub fn energy(&self) -> f64 {
        0.5 * self.capacitance * self.voltage.powi(2)
    }
}

/// Inductor with state
#[derive(Debug, Clone)]
pub struct Inductor {
    pub id: usize,
    pub inductance: f64,  // Henries
    pub current: f64,      // Current through inductor
    pub flux: f64,         // Magnetic flux
}

impl Inductor {
    pub fn new(id: usize, inductance: f64) -> Self {
        Inductor {
            id,
            inductance,
            current: 0.0,
            flux: 0.0,
        }
    }

    /// Update inductor (voltage drives current)
    pub fn update(&mut self, voltage: f64, dt: f64) {
        self.flux += voltage * dt;
        self.current = self.flux / self.inductance;
    }

    /// Get energy stored
    pub fn energy(&self) -> f64 {
        0.5 * self.inductance * self.current.powi(2)
    }
}

/// Analog circuit simulator
#[derive(Debug, Clone)]
pub struct AnalogCircuit {
    pub components: Vec<Component>,
    pub nodes: Vec<f64>,  // Node voltages
    pub ground: usize,    // Ground node index
}

impl AnalogCircuit {
    pub fn new(num_nodes: usize) -> Self {
        AnalogCircuit {
            components: Vec::new(),
            nodes: vec![0.0; num_nodes],
            ground: 0,
        }
    }

    pub fn add_resistor(&mut self, resistance: f64, n1: usize, n2: usize) {
        let id = self.components.len();
        self.components.push(Component::resistor(id, resistance, n1, n2));
    }

    pub fn add_capacitor(&mut self, capacitance: f64, n1: usize, n2: usize) {
        let id = self.components.len();
        self.components.push(Component::capacitor(id, capacitance, n1, n2));
    }

    pub fn add_voltage_source(&mut self, voltage: f64, n1: usize, n2: usize) {
        let id = self.components.len();
        self.components.push(Component::voltage_source(id, voltage, n1, n2));
    }

    /// Solve circuit using modified nodal analysis (simplified)
    pub fn solve(&mut self) {
        // Simplified: just set node voltages based on voltage sources
        for comp in &self.components {
            match comp.component_type {
                ComponentType::VoltageSource => {
                    self.nodes[comp.nodes[0]] = comp.value;
                    self.nodes[comp.nodes[1]] = 0.0;  // Ground
                }
                _ => {}
            }
        }
    }

    /// Get 6D representation of circuit state
    pub fn to_6d_coordinates(&self) -> [f64; 6] {
        // Sample node voltages as coordinates
        let v0 = self.nodes.get(0).unwrap_or(&0.0);
        let v1 = self.nodes.get(1).unwrap_or(&0.0);
        let v2 = self.nodes.get(2).unwrap_or(&0.0);
        
        [
            v0 * PHI,
            v1 * PHI_INV,
            v2 * (PHI - 1.0),
            self.components.len() as f64 * 0.1,
            self.nodes.iter().sum::<f64>() / self.nodes.len().max(1) as f64,
            self.energy_total(),
        ]
    }

    /// Total energy in circuit
    fn energy_total(&self) -> f64 {
        // Sum of voltage * current for all components
        let mut energy = 0.0;
        for comp in &self.components {
            match comp.component_type {
                ComponentType::Resistor => {
                    let v = self.nodes[comp.nodes[0]] - self.nodes[comp.nodes[1]];
                    let i = v / comp.value;
                    energy += v * i;  // Power
                }
                _ => {}
            }
        }
        energy
    }
}
