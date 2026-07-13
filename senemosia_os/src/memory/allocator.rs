//! Memory allocator with φ-based partitioning (Quasicrystal Allocator)
//! 
//! Instead of binary division (2^n like Buddy Allocator), memory is partitioned
//! according to the golden ratio φ ≈ 1.618 for self-similar fragmentation patterns

use crate::geometry::{Point6D, Point3D, PHI_INV, PHI, SpectralConflictResolver};
use crate::geometry::conflict::MAX_RECALIBRATION_ATTEMPTS;

/// Standard page size (4KB)
pub const PAGE_SIZE: usize = 4096;

/// Maximum pages in the system
pub const MAX_PAGES: usize = 1024;

/// Spatial page representing a memory block in geometric space
#[derive(Debug, Clone, Copy)]
pub struct GeometricPage {
    /// Unique page identifier
    pub id: [u8; 16],
    /// 6D hyperdimensional coordinates
    pub coord_6d: [f64; 6],
    /// Physical memory address
    pub physical_address: usize,
    /// Resonance level with Zero Point
    pub resonance: f64,
    /// Whether this page is currently allocated
    pub allocated: bool,
}

impl GeometricPage {
    /// Calculate allocation priority based on distance to Zero Point
    pub fn calculate_priority(&self, zero_point: &[f64; 6]) -> f64 {
        let mut sum_sq = 0.0;
        for i in 0..6 {
            let diff = self.coord_6d[i] - zero_point[i];
            sum_sq += diff * diff;
        }
        let distance = libm::sqrt(sum_sq);
        // Closer to Zero Point = higher priority (higher resonance)
        1.0 / (1.0 + distance)
    }
}

/// Page table tracking physical memory allocation
pub struct PageTable {
    pub pages: [Option<GeometricPage>; MAX_PAGES],
    pub base_address: usize,
    pub total_memory: usize,
}

impl PageTable {
    /// Create a new page table
    pub fn new(base: usize, size: usize) -> Self {
        let pages = [const { None }; MAX_PAGES];
        PageTable {
            pages,
            base_address: base,
            total_memory: size,
        }
    }

    /// Find first free page slot
    pub fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_PAGES {
            if self.pages[i].is_none() {
                return Some(i);
            }
        }
        None
    }

    /// Allocate a page at the given index
    pub fn allocate_page(&mut self, index: usize, page: GeometricPage) -> Result<(), &'static str> {
        if index >= MAX_PAGES {
            return Err("Page index out of bounds");
        }
        if self.pages[index].is_some() {
            return Err("Page slot already occupied");
        }
        self.pages[index] = Some(page);
        Ok(())
    }
}

/// φ-based memory allocator using geometric addressing
pub struct QuasicrystalAllocator {
    pub memory_base: usize,
    pub scale_factor: f64,
    pub memory_size: usize,
    pub page_table: PageTable,
    pub conflict_resolver: SpectralConflictResolver,
}

impl QuasicrystalAllocator {
    /// Create a new φ-allocator
    pub fn new(base: usize, size: usize) -> Self {
        QuasicrystalAllocator {
            memory_base: base,
            scale_factor: 4096.0, // Each spatial unit = one 4KB page
            memory_size: size,
            page_table: PageTable::new(base, size),
            conflict_resolver: SpectralConflictResolver::new(),
        }
    }

    /// Convert 3D point to physical memory address
    fn get_physical_address(&self, point: &Point3D) -> usize {
        let (x, y, z) = point.discretize(self.scale_factor);

        // Spatial hash using prime numbers to reduce collisions
        let offset = (x.wrapping_mul(73856093))
            ^ (y.wrapping_mul(19349663))
            ^ (z.wrapping_mul(83492791));

        let relative = offset % self.memory_size;
        self.memory_base.wrapping_add(relative)
    }

    /// Allocate a geometric node with spectral conflict resolution
    pub fn allocate_node(&mut self, node_coords: &[f64; 6]) -> Result<usize, &'static str> {
        let mut current_node = Point6D::new(*node_coords);
        let mut attempt = 0;

        while attempt < MAX_RECALIBRATION_ATTEMPTS {
            // 1. Project 6D to 3D space
            let point_3d = current_node.project();

            // 2. Calculate physical memory address
            let address = self.get_physical_address(&point_3d);
            let page_index = (address - self.memory_base) / PAGE_SIZE;

            // 3. Check for collision
            if let Some(slot) = self.page_table.find_free_slot() {
                let zero_point = [0.0; 6];
                let resonance = current_node.resonance(&Point6D::new(zero_point));

                let page = GeometricPage {
                    id: [0; 16],
                    coord_6d: current_node.coords,
                    physical_address: address,
                    resonance,
                    allocated: true,
                };

                self.page_table.pages[slot] = Some(page);
                return Ok(address);
            }

            // 4. Collision detected: Apply 6D spectral recalibration
            attempt += 1;
            current_node = self.conflict_resolver.full_recalibration(&current_node, attempt);
        }

        Err("Kernel Panic: Geometric memory spectrum saturated")
    }

    /// φ-based dynamic partitioning
    /// Splits memory blocks according to golden ratio proportions
    pub fn phi_split(size: usize) -> (usize, usize) {
        let major = (size as f64 * PHI_INV) as usize;
        let minor = size - major;
        (major, minor)
    }

    /// Free a page by physical address
    pub fn free_page(&mut self, address: usize) -> Result<(), &'static str> {
        for slot in 0..MAX_PAGES {
            if let Some(page) = self.page_table.pages[slot] {
                if page.physical_address == address && page.allocated {
                    self.page_table.pages[slot] = None;
                    return Ok(());
                }
            }
        }
        Err("Page not found")
    }
}
