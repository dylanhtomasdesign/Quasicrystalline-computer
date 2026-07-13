//! Video driver for framebuffer visualization
//! 
//! Renders the spatial allocation matrix and system state
//! directly to VGA/linear framebuffer

use crate::fs::snfs::HarmonicAllocationTable;
use crate::fs::snfs::MAX_NODES;

/// Color constants (ARGB format)
const COLOR_BLACK: u32 = 0x00000000;
const COLOR_CYAN: u32 = 0x0000FFFF;
const COLOR_GREEN: u32 = 0x0000FF00;
const COLOR_RED: u32 = 0x00FF0000;
const COLOR_WHITE: u32 = 0x00FFFFFF;

/// Framebuffer driver for direct pixel manipulation
pub struct Framebuffer {
    /// Base address of video memory
    pub address: *mut u32,
    /// Horizontal resolution
    pub width: usize,
    /// Vertical resolution
    pub height: usize,
    /// Current X cursor position
    pub cursor_x: usize,
    /// Current Y cursor position
    pub cursor_y: usize,
}

impl Framebuffer {
    /// Create a new framebuffer
    pub fn new(address: *mut u32, width: usize, height: usize) -> Self {
        Framebuffer {
            address,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Draw a single pixel
    #[inline(always)]
    pub unsafe fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let offset = y * self.width + x;
            core::ptr::write_volatile(self.address.add(offset), color);
        }
    }

    /// Clear screen to a solid color
    pub unsafe fn clear(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
    }

    /// Clear screen to black (Punto Zero)
    pub unsafe fn clear_zero(&self) {
        self.clear(COLOR_BLACK);
    }

    /// Draw a filled rectangle
    pub unsafe fn draw_rect(&self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for dy in 0..h {
            for dx in 0..w {
                self.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    /// Render the spatial allocation matrix from HAT
    pub unsafe fn render_spatial_matrix(&self, hat: &HarmonicAllocationTable) {
        // Clear to black (void)
        self.clear_zero();

        for i in 0..MAX_NODES {
            let inode = &hat.nodes[i];
            if inode.active == 1 {
                // Project 6D coordinates to 2D screen space
                let x_pos = (((inode.coord_6d[0] + inode.coord_6d[3]) * 100.0).abs() as usize) % self.width;
                let y_pos = (((inode.coord_6d[1] + inode.coord_6d[4]) * 100.0).abs() as usize) % self.height;

                // Color based on resonance
                let intensity = ((inode.resonance_score * 255.0) as u32).min(255);
                let color = if inode.resonance_score >= 0.5 {
                    // Stable: Cyan/Green spectrum
                    (intensity << 8) | (intensity << 16) | 0x00000080
                } else {
                    // Unstable: Red spectrum
                    COLOR_RED | (intensity << 8)
                };

                // Draw as 4x4 pixel cluster for visibility
                for dx in 0..4 {
                    for dy in 0..4 {
                        self.draw_pixel(x_pos + dx, y_pos + dy, color);
                    }
                }
            }
        }
    }

    /// Print a character (basic 8x16 font approximation)
    pub unsafe fn put_char(&mut self, _c: u8, color: u32) {
        // Simple 8x8 bitmap for ASCII printable characters
        // This is a simplified version - full implementation would use a font bitmap
        if self.cursor_x + 8 > self.width {
            self.cursor_x = 0;
            self.cursor_y += 16;
        }
        if self.cursor_y + 16 > self.height {
            self.cursor_y = 0;
        }

        // For now, just draw a rectangle as placeholder
        self.draw_rect(self.cursor_x, self.cursor_y, 8, 16, color);
        self.cursor_x += 9;
    }

    /// Print a string
    pub unsafe fn print(&mut self, s: &str, color: u32) {
        for byte in s.bytes() {
            self.put_char(byte, color);
        }
    }

    /// Newline
    pub fn newline(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 20;
        if self.cursor_y >= self.height {
            self.cursor_y = 0;
        }
    }

    /// Move cursor to position
    pub fn move_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}
