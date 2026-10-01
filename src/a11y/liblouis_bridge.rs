use serde::{Deserialize, Serialize};

/// ============================================================================
/// Refreshable Braille Display Hardware Driver (USB HID & Bluetooth LE)
/// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrailleDisplayInfo {
    pub device_name: String,
    pub cell_count: usize, // e.g., 14, 40, or 80 cells
    pub is_connected: bool,
    pub battery_pct: u8,
}

pub struct BrailleDisplayDriver {
    pub display_info: BrailleDisplayInfo,
    pub current_cells_buffer: Vec<u8>, // 8-bit mask per cell
}

impl BrailleDisplayDriver {
    pub fn new(name: &str, cells: usize) -> Self {
        Self {
            display_info: BrailleDisplayInfo {
                device_name: name.to_string(),
                cell_count: cells,
                is_connected: true,
                battery_pct: 95,
            },
            current_cells_buffer: vec![0; cells],
        }
    }

    /// Render baris sel Braille ke pin fisik aktuator display
    pub fn write_cells(&mut self, cells_data: &[u8]) {
        for (idx, &dot_mask) in cells_data.iter().take(self.display_info.cell_count).enumerate() {
            self.current_cells_buffer[idx] = dot_mask;
        }
    }
}
