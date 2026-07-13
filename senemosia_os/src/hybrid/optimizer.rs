//! Optimization Networks Module
//! 
//! Implements Hopfield networks and other analog optimization circuits.

use crate::geometry::PHI;
use crate::geometry::PHI_INV;

/// Hopfield Network for combinatorial optimization
/// Implemented as analog circuit simulation
#[derive(Debug, Clone)]
pub struct HopfieldNetwork {
    pub num_neurons: usize,
    pub weights: Vec<Vec<f64>>,  // Weight matrix
    pub thresholds: Vec<f64>,     // Neuron thresholds
    pub states: Vec<f64>,        // Current states (-1 or 1)
    pub energy: f64,             // Network energy
}

impl HopfieldNetwork {
    pub fn new(num_neurons: usize) -> Self {
        let weights = vec![vec![0.0; num_neurons]; num_neurons];
        let thresholds = vec![0.0; num_neurons];
        let states = vec![0.0; num_neurons];
        
        HopfieldNetwork {
            num_neurons,
            weights,
            thresholds,
            states,
            energy: 0.0,
        }
    }

    /// Set weight between two neurons
    pub fn set_weight(&mut self, i: usize, j: usize, weight: f64) {
        if i < self.num_neurons && j < self.num_neurons {
            self.weights[i][j] = weight;
        }
    }

    /// Set all weights from a pattern (Hebbian learning)
    pub fn learn_pattern(&mut self, pattern: &[f64]) {
        if pattern.len() != self.num_neurons {
            return;
        }
        
        for i in 0..self.num_neurons {
            for j in 0..self.num_neurons {
                if i != j {
                    self.weights[i][j] += PHI_INV * pattern[i] * pattern[j] / self.num_neurons as f64;
                }
            }
        }
    }

    /// Update single neuron (async)
    pub fn update_neuron(&mut self, i: usize) {
        let mut sum = 0.0;
        for j in 0..self.num_neurons {
            sum += self.weights[i][j] * self.states[j];
        }
        sum -= self.thresholds[i];
        
        // Activation function (sign)
        self.states[i] = if sum > 0.0 { 1.0 } else { -1.0 };
    }

    /// Update all neurons (sync)
    pub fn update_all(&mut self) {
        let mut new_states = vec![0.0; self.num_neurons];
        
        for i in 0..self.num_neurons {
            let mut sum = 0.0;
            for j in 0..self.num_neurons {
                sum += self.weights[i][j] * self.states[j];
            }
            sum -= self.thresholds[i];
            new_states[i] = if sum > 0.0 { 1.0 } else { -1.0 };
        }
        
        self.states = new_states;
    }

    /// Compute network energy
    pub fn compute_energy(&mut self) -> f64 {
        let mut e = 0.0;
        
        // E = -0.5 * sum(w_ij * s_i * s_j) + sum(theta_i * s_i)
        for i in 0..self.num_neurons {
            for j in 0..self.num_neurons {
                e -= 0.5 * self.weights[i][j] * self.states[i] * self.states[j];
            }
            e += self.thresholds[i] * self.states[i];
        }
        
        self.energy = e;
        e
    }

    /// Run until convergence
    pub fn converge(&mut self, max_iterations: usize) -> bool {
        for _ in 0..max_iterations {
            let old_energy = self.energy;
            self.update_all();
            self.compute_energy();
            
            // Converged when energy stops changing
            if (self.energy - old_energy).abs() < 1e-6 {
                return true;
            }
        }
        false
    }

    /// Get state as 6D coordinate
    pub fn to_6d_coordinate(&self) -> [f64; 6] {
        // Use first 6 neurons as coordinates
        let mut coord = [0.0; 6];
        for i in 0..6.min(self.num_neurons) {
            coord[i] = self.states[i] * PHI.powf(i as f64 * PHI_INV);
        }
        
        [
            coord[0],
            coord[1],
            coord[2],
            self.energy,
            self.states.iter().sum::<f64>() / self.num_neurons as f64,
            (self.num_neurons as f64) * PHI_INV,
        ]
    }
}

/// Simulated Annealing optimizer
pub struct SimulatedAnnealing {
    pub temperature: f64,
    pub cooling_rate: f64,
    pub min_temperature: f64,
}

impl SimulatedAnnealing {
    pub fn new(initial_temp: f64, cooling_rate: f64) -> Self {
        SimulatedAnnealing {
            temperature: initial_temp,
            cooling_rate,
            min_temperature: 0.001,
        }
    }

    /// Accept worse solution with probability
    pub fn should_accept(&self, delta_energy: f64) -> bool {
        if delta_energy < 0.0 {
            true
        } else {
            let prob = libm::exp(-delta_energy / self.temperature);
            let random = PHI.fract();  // Simplified random
            random < prob
        }
    }

    /// Cool down
    pub fn cool(&mut self) {
        self.temperature *= self.cooling_rate;
        if self.temperature < self.min_temperature {
            self.temperature = self.min_temperature;
        }
    }
}

/// Gradient descent optimizer (for continuous optimization)
pub struct GradientDescent {
    pub learning_rate: f64,
    pub momentum: f64,
    pub velocity: f64,
}

impl GradientDescent {
    pub fn new(learning_rate: f64) -> Self {
        GradientDescent {
            learning_rate,
            momentum: 0.9,
            velocity: 0.0,
        }
    }

    /// Update position using gradient
    pub fn update(&mut self, position: &mut f64, gradient: f64) {
        self.velocity = self.momentum * self.velocity - self.learning_rate * gradient;
        *position += self.velocity;
    }
}
