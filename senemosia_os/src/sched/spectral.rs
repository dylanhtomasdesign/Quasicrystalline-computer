//! Spectral scheduler based on Laplacian matrix eigenvalues
//! 
//! Process scheduling is determined by the Fiedler vector (second smallest
//! eigenvalue of the Laplacian matrix), maximizing system resonance

use crate::geometry::{Point6D, PHI, PHI_INV};

/// Maximum number of processes
pub const MAX_PROCESSES: usize = 32;

/// Process execution state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
}

/// Process structure with geometric identity
#[derive(Debug, Clone)]
pub struct Process {
    pub id: usize,
    pub state: ProcessState,
    /// 6D geometric coordinates (node identity)
    pub coord_6d: Point6D,
    /// Connection weights to other processes (adjacency)
    pub connections: [f64; MAX_PROCESSES],
}

impl Process {
    pub fn new(id: usize, coords: [f64; 6]) -> Self {
        Process {
            id,
            state: ProcessState::Ready,
            coord_6d: Point6D::new(coords),
            connections: [0.0; MAX_PROCESSES],
        }
    }
}

/// Spectral scheduler using Laplacian-based process ordering
pub struct SpectralScheduler {
    pub processes: [Option<Process>; MAX_PROCESSES],
    pub laplacian: [[f64; MAX_PROCESSES]; MAX_PROCESSES],
    pub current_running: Option<usize>,
    pub fiedler_vector: [f64; MAX_PROCESSES],
}

impl SpectralScheduler {
    /// Create a new spectral scheduler
    pub fn new() -> Self {
        const NONE_PROCESS: Option<Process> = None;
        let init_val = 1.0 / libm::sqrt(MAX_PROCESSES as f64);
        SpectralScheduler {
            processes: [NONE_PROCESS; MAX_PROCESSES],
            laplacian: [[0.0; MAX_PROCESSES]; MAX_PROCESSES],
            current_running: None,
            fiedler_vector: [init_val; MAX_PROCESSES],
        }
    }

    /// Add a process to the scheduler
    pub fn add_process(&mut self, coords: [f64; 6]) -> Option<usize> {
        for i in 0..MAX_PROCESSES {
            if self.processes[i].is_none() {
                self.processes[i] = Some(Process::new(i, coords));
                return Some(i);
            }
        }
        None
    }

    /// Update process connections (adjacency weights)
    pub fn update_connection(&mut self, from: usize, to: usize, weight: f64) {
        if from < MAX_PROCESSES && to < MAX_PROCESSES {
            if let Some(ref mut p) = self.processes[from] {
                p.connections[to] = weight;
            }
        }
    }

    /// Build Laplacian matrix L = D - A (Degree - Adjacency)
    fn build_laplacian(&mut self) {
        // Reset Laplacian
        for i in 0..MAX_PROCESSES {
            for j in 0..MAX_PROCESSES {
                self.laplacian[i][j] = 0.0;
            }
        }

        // Build from process connections
        for i in 0..MAX_PROCESSES {
            if let Some(ref p) = self.processes[i] {
                let mut degree = 0.0;
                for j in 0..MAX_PROCESSES {
                    if i != j && self.processes[j].is_some() {
                        let weight = p.connections[j];
                        self.laplacian[i][j] = -weight;
                        degree += weight;
                    }
                }
                self.laplacian[i][i] = degree; // Diagonal = degree
            }
        }
    }

    /// Compute Fiedler vector using power iteration with Tikhonov regularization
    pub fn compute_fiedler_vector(&mut self) -> [f64; MAX_PROCESSES] {
        const EPSILON: f64 = 1e-6;
        const ITERATIONS: usize = 6;

        self.build_laplacian();

        // Apply Tikhonov regularization for numerical stability
        for i in 0..MAX_PROCESSES {
            self.laplacian[i][i] += EPSILON / PHI;
        }

        // Warm start from previous state
        let mut v = self.fiedler_vector;

        for _ in 0..ITERATIONS {
            let mut v_next = [0.0; MAX_PROCESSES];

            // Matrix-vector multiplication: L * v
            for i in 0..MAX_PROCESSES {
                let mut sum = 0.0;
                for j in 0..MAX_PROCESSES {
                    sum += self.laplacian[i][j] * v[j];
                }
                v_next[i] = sum;
            }

            // Orthogonalize against first eigenvector [1,1,...,1]
            let mut mean = 0.0;
            for i in 0..MAX_PROCESSES {
                mean += v_next[i];
            }
            mean /= MAX_PROCESSES as f64;
            for i in 0..MAX_PROCESSES {
                v_next[i] -= mean;
            }

            // L2 normalization
            let mut norm_sq = 0.0;
            for i in 0..MAX_PROCESSES {
                norm_sq += v_next[i] * v_next[i];
            }
            let norm = libm::sqrt(norm_sq);
            if norm > 1e-12 {
                for i in 0..MAX_PROCESSES {
                    v_next[i] /= norm;
                }
            }

            v = v_next;
        }

        // Apply temporal hysteresis (EMA filter) with α = 1/φ
        let alpha = PHI_INV;
        for i in 0..MAX_PROCESSES {
            self.fiedler_vector[i] = alpha * v[i] + (1.0 - alpha) * self.fiedler_vector[i];
        }

        self.fiedler_vector
    }

    /// Schedule next process based on spectral ordering
    pub fn schedule_next(&mut self) -> Option<usize> {
        let fiedler = self.compute_fiedler_vector();
        let mut target_index = None;
        let mut max_tension = -1.0_f64;

        // Find ready process with highest spectral tension
        for i in 0..MAX_PROCESSES {
            if let Some(ref p) = self.processes[i] {
                if p.state == ProcessState::Ready {
                    let tension = fiedler[i].abs();
                    if tension > max_tension {
                        max_tension = tension;
                        target_index = Some(i);
                    }
                }
            }
        }

        // Perform context switch
        if let Some(idx) = target_index {
            if let Some(curr) = self.current_running {
                if let Some(ref mut p) = self.processes[curr] {
                    p.state = ProcessState::Ready;
                }
            }
            if let Some(ref mut p) = self.processes[idx] {
                p.state = ProcessState::Running;
            }
            self.current_running = Some(idx);
        }

        target_index
    }

    /// Block a process (e.g., waiting for I/O)
    pub fn block_process(&mut self, id: usize) {
        if let Some(ref mut p) = self.processes[id] {
            p.state = ProcessState::Blocked;
        }
    }

    /// Wake a blocked process
    pub fn wake_process(&mut self, id: usize) {
        if let Some(ref mut p) = self.processes[id] {
            if p.state == ProcessState::Blocked {
                p.state = ProcessState::Ready;
            }
        }
    }
}

impl Default for SpectralScheduler {
    fn default() -> Self {
        Self::new()
    }
}
