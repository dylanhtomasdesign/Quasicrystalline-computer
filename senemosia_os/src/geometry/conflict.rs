//! Spectral conflict resolution using 6D rotations (Givens rotations)
//! Resolves memory allocation conflicts by rotating nodes through higher dimensions

use super::projection::{Point6D, PHI, PHI_INV, PHI_SQUARED};

/// Spectral conflict resolver using hyperdimensional rotations
pub struct SpectralConflictResolver {
    /// Harmonic factor based on golden ratio
    pub harmonic_factor: f64,
}

impl SpectralConflictResolver {
    /// Create a new conflict resolver with φ-based harmonic factor
    pub fn new() -> Self {
        SpectralConflictResolver {
            harmonic_factor: PHI_INV,
        }
    }

    /// Apply 6D Givens rotation on the (i,j) plane by angle θ
    /// This rotates the hyperdimensional vector to resolve spatial conflicts
    pub fn rotate_6d(&self, point: &Point6D, i: usize, j: usize, angle: f64) -> Point6D {
        let mut coords = point.coords;
        let cos_theta = libm::cos(angle);
        let sin_theta = libm::sin(angle);

        // Bounds check
        if i >= 6 || j >= 6 || i == j {
            return *point;
        }

        let ci = coords[i];
        let cj = coords[j];

        coords[i] = ci * cos_theta - cj * sin_theta;
        coords[j] = ci * sin_theta + cj * cos_theta;

        Point6D { coords }
    }

    /// Recalibrate a conflicting node using φ-based angle
    /// Attempts increase with each retry to find harmonic space
    pub fn recalibrate(&self, conflict_node: &Point6D, attempt: u32) -> Point6D {
        // Dynamic angle based on attempt and golden ratio
        let angle = self.harmonic_factor * (attempt as f64) * libm::acos(-1.0) / PHI;
        
        // Rotate on the primary plane (0, 1) first
        self.rotate_6d(conflict_node, 0, 1, angle)
    }

    /// Full spectral recalibration: apply rotations on multiple planes
    /// This ensures the node finds its natural harmonic space
    pub fn full_recalibration(&self, conflict_node: &Point6D, attempt: u32) -> Point6D {
        let mut node = *conflict_node;
        let base_angle = self.harmonic_factor * (attempt as f64);

        // Apply rotations on multiple 6D planes
        // Plane (0,1)
        node = self.rotate_6d(&node, 0, 1, base_angle);
        // Plane (2,3)
        node = self.rotate_6d(&node, 2, 3, base_angle * PHI_INV);
        // Plane (4,5)
        node = self.rotate_6d(&node, 4, 5, base_angle * PHI_SQUARED);

        node
    }

    /// Detect if two nodes are in conflict (too close in 6D space)
    pub fn detect_conflict(&self, node1: &Point6D, node2: &Point6D, threshold: f64) -> bool {
        node1.distance(node2) < threshold
    }
}

impl Default for SpectralConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Maximum number of recalibration attempts before kernel panic
pub const MAX_RECALIBRATION_ATTEMPTS: u32 = 10;
