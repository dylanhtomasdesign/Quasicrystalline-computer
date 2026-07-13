//! Geometry module for 6D→3D projection and spatial transformations
//! Based on icosahedral symmetry and golden ratio (φ)

use core::f64::consts::PI;

/// Golden Ratio φ ≈ 1.618033988749895
pub const PHI: f64 = 1.618033988749895;

/// Inverse Golden Ratio 1/φ ≈ 0.618033988749895
pub const PHI_INV: f64 = 0.618033988749895;

/// φ² ≈ 2.618033988749895
pub const PHI_SQUARED: f64 = 2.618033988749895;

/// Approximate sqrt for no_std
fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

/// 6D to 3D projection matrix based on icosahedral geometry
/// Rows are derived from golden ratio vectors pointing to icosahedron vertices
pub const PROJECTION_MATRIX: [[f64; 6]; 3] = [
    // X axis: harmonic combination of 6D components
    [1.0, PHI_INV, 0.0, -1.0, PHI_INV, 0.0],
    // Y axis
    [PHI_INV, 0.0, 1.0, PHI_INV, 0.0, -1.0],
    // Z axis
    [0.0, 1.0, PHI_INV, 0.0, -1.0, PHI_INV],
];

/// 6D hyperdimensional point structure
#[derive(Debug, Clone, Copy)]
pub struct Point6D {
    pub coords: [f64; 6],
}

impl Point6D {
    /// Create a new 6D point from coordinates
    pub fn new(coords: [f64; 6]) -> Self {
        Point6D { coords }
    }

    /// Project 6D coordinates to 3D space
    pub fn project(&self) -> Point3D {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;

        for i in 0..6 {
            x += PROJECTION_MATRIX[0][i] * self.coords[i];
            y += PROJECTION_MATRIX[1][i] * self.coords[i];
            z += PROJECTION_MATRIX[2][i] * self.coords[i];
        }

        Point3D { x, y, z }
    }

    /// Calculate Euclidean distance to another 6D point
    pub fn distance(&self, other: &Point6D) -> f64 {
        let mut sum = 0.0;
        for i in 0..6 {
            let diff = self.coords[i] - other.coords[i];
            sum += diff * diff;
        }
        sqrt(sum)
    }

    /// Calculate resonance with a reference point (closer = higher resonance)
    pub fn resonance(&self, zero_point: &Point6D) -> f64 {
        let dist = self.distance(zero_point);
        1.0 / (1.0 + dist)
    }

    /// Normalize the 6D vector
    pub fn normalize(&mut self) {
        let mut norm_sq = 0.0;
        for i in 0..6 {
            norm_sq += self.coords[i] * self.coords[i];
        }
        let norm = sqrt(norm_sq);
        if norm > 1e-12 {
            for i in 0..6 {
                self.coords[i] /= norm;
            }
        }
    }
}

/// Zero point (origin) in 6D space - the anchor of the quasicrystalline system
pub fn zero_point() -> Point6D {
    Point6D::new([0.0; 6])
}

/// Anchor point based on golden ratio proportions
pub fn anchor_point() -> Point6D {
    Point6D::new([
        PHI_INV,
        1.0,
        PHI_INV,
        PHI_INV,
        1.0,
        PHI_INV,
    ])
}

/// 3D projected point result
#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    /// Calculate distance to another 3D point
    pub fn distance(&self, other: &Point3D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        sqrt(dx * dx + dy * dy + dz * dz)
    }

    /// Discretize to memory address space (for allocator)
    pub fn discretize(&self, scale: f64) -> (usize, usize, usize) {
        (
            (self.x.abs() * scale) as usize,
            (self.y.abs() * scale) as usize,
            (self.z.abs() * scale) as usize,
        )
    }
}
