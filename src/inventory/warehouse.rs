// ============================================================================
// WIDYA INVENTORY & SUPPLY CHAIN - WAREHOUSE MANAGEMENT (src/inventory/warehouse.rs)
// Otomasi Pelacakan Stok Gudang, Manajemen Lokasi Rak/Bin, dan Rekonsiliasi RFID
// ============================================================================

use super::barcode::Ean13Barcode;
use super::rfid::{EpcFilterValue, EpcSgtin96, GateDirection, RfidGateEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Definisi Lokasi Fisik Penyimpanan di Gudang (Zona-Lorong-Rak-Bin)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarehouseLocation {
    pub location_id: String, // Contoh: "ZONA-A-L02-R04-B01"
    pub zone: String,        // Misal: "ZONA-A" (Bahan Baku / Produk Jadi)
    pub aisle: String,       // Lorong
    pub rack: String,        // Rak
    pub bin: String,         // Kotak/Slot Bin
    pub max_capacity_units: u32,
    pub current_units: u32,
}

/// Entitas Barang Inventaris Terpadu (Barcode + RFID)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub sku_barcode: String,       // EAN-13 / Code-128
    pub rfid_epc_hex: String,      // EPC Gen2 SGTIN-96
    pub name: String,
    pub category: String,
    pub unit_of_measure: String,   // Pcs, Box, Karton, Kg
    pub current_stock: u32,
    pub min_reorder_threshold: u32,
    pub unit_cost: u64,            // Rupiah
    pub location_id: String,
}

/// Catatan Riwayat Mutasi Stok Gudang (Audit Trail Mutasi)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovementRecord {
    pub movement_id: String,
    pub sku_barcode: String,
    pub rfid_epc_hex: String,
    pub direction: GateDirection,
    pub quantity: u32,
    pub from_location: Option<String>,
    pub to_location: Option<String>,
    pub timestamp_ms: u64,
    pub operator_id: String,
}

/// Laporan Rekonsiliasi Stok Opname Fisik vs Sistem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockOpnameReport {
    pub total_system_stock: u32,
    pub total_scanned_rfid: u32,
    pub matched_tags: u32,
    pub missing_tags: Vec<String>,
    pub unexpected_tags: Vec<String>,
    pub discrepancy_count: i32,
    pub is_balanced: bool,
}

/// Mesin Orkestrasi Inventaris Gudang Mandiri
pub struct WarehouseManager {
    items: HashMap<String, InventoryItem>, // Key: SKU Barcode
    epc_to_sku: HashMap<String, String>,   // Key: RFID EPC Hex -> SKU Barcode
    locations: HashMap<String, WarehouseLocation>,
    movements: Vec<StockMovementRecord>,
}

impl WarehouseManager {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            epc_to_sku: HashMap::new(),
            locations: HashMap::new(),
            movements: Vec::new(),
        }
    }

    /// Mendaftarkan lokasi rak baru ke gudang
    pub fn register_location(&mut self, loc: WarehouseLocation) {
        self.locations.insert(loc.location_id.clone(), loc);
    }

    /// Mendaftarkan item baru dengan validasi format Barcode EAN-13 dan RFID EPC
    pub fn register_item(
        &mut self,
        barcode: &str,
        name: &str,
        category: &str,
        unit: &str,
        initial_stock: u32,
        min_threshold: u32,
        cost: u64,
        loc_id: &str,
    ) -> Result<InventoryItem, String> {
        // Validasi format barcode EAN-13 jika tepat 13 digit numerik
        if barcode.len() == 13 && barcode.chars().all(|c| c.is_ascii_digit()) {
            let _ = Ean13Barcode::parse(barcode)?;
        }

        // Generate RFID EPC SGTIN-96 otomatis untuk produk ini
        let company_prefix = 8991234; // GS1 Indonesia Demo Prefix
        let item_ref = (self.items.len() + 1) as u32;
        let serial = 1001u64;
        let epc = EpcSgtin96::new(EpcFilterValue::PointOfSale, company_prefix, item_ref, serial);

        let item = InventoryItem {
            sku_barcode: barcode.to_string(),
            rfid_epc_hex: epc.raw_hex.clone(),
            name: name.to_string(),
            category: category.to_string(),
            unit_of_measure: unit.to_string(),
            current_stock: initial_stock,
            min_reorder_threshold: min_threshold,
            unit_cost: cost,
            location_id: loc_id.to_string(),
        };

        self.epc_to_sku.insert(epc.raw_hex.clone(), barcode.to_string());
        self.items.insert(barcode.to_string(), item.clone());

        // Update kapasitas lokasi jika ada
        if let Some(loc) = self.locations.get_mut(loc_id) {
            loc.current_units += initial_stock;
        }

        Ok(item)
    }

    /// Memproses event pemindaian Gate RFID otomatis (Gate-In atau Gate-Out)
    pub fn process_gate_event(&mut self, event: RfidGateEvent) -> Result<StockMovementRecord, String> {
        let epc_clean = event.tag_epc_hex.trim().to_uppercase();
        let sku = self
            .epc_to_sku
            .get(&epc_clean)
            .cloned()
            .ok_or_else(|| format!("Tag RFID EPC '{}' belum terdaftar di inventaris", epc_clean))?;

        let item = self.items.get_mut(&sku).unwrap();

        let (from_loc, to_loc) = match event.direction {
            GateDirection::MasukGudang => {
                item.current_stock += 1;
                (None, Some(item.location_id.clone()))
            }
            GateDirection::KeluarGudang => {
                if item.current_stock == 0 {
                    return Err(format!("Stok untuk item '{}' sudah kosong (0)", item.name));
                }
                item.current_stock -= 1;
                (Some(item.location_id.clone()), None)
            }
            GateDirection::PindahLokasi => (Some(item.location_id.clone()), Some("ZONA-TRANSIT".to_string())),
        };

        let movement = StockMovementRecord {
            movement_id: format!("MOV-{}", self.movements.len() + 1),
            sku_barcode: sku,
            rfid_epc_hex: epc_clean,
            direction: event.direction,
            quantity: 1,
            from_location: from_loc,
            to_location: to_loc,
            timestamp_ms: event.timestamp_ms,
            operator_id: format!("PORTAL_{}", event.gate_id),
        };

        self.movements.push(movement.clone());
        Ok(movement)
    }

    /// Menjalankan rekonsiliasi Stok Opname RFID secara otomatis
    pub fn reconcile_stock_opname(&self, scanned_epcs: &[String]) -> StockOpnameReport {
        let total_system: u32 = self.items.values().map(|i| i.current_stock).sum();
        let total_scanned = scanned_epcs.len() as u32;

        let mut matched = 0;
        let mut missing = Vec::new();
        let mut unexpected = Vec::new();

        let mut system_epcs: HashMap<String, bool> = HashMap::new();
        for item in self.items.values() {
            system_epcs.insert(item.rfid_epc_hex.clone(), false);
        }

        for scan in scanned_epcs {
            let clean = scan.trim().to_uppercase();
            if let Some(found) = system_epcs.get_mut(&clean) {
                *found = true;
                matched += 1;
            } else {
                unexpected.push(clean);
            }
        }

        for (epc, found) in system_epcs {
            if !found {
                missing.push(epc);
            }
        }

        let discrepancy = total_scanned as i32 - total_system as i32;

        StockOpnameReport {
            total_system_stock: total_system,
            total_scanned_rfid: total_scanned,
            matched_tags: matched,
            missing_tags: missing,
            unexpected_tags: unexpected,
            discrepancy_count: discrepancy,
            is_balanced: discrepancy == 0,
        }
    }

    /// Mengambil seluruh item yang berada di bawah ambang batas stok minimum
    pub fn get_low_stock_alerts(&self) -> Vec<&InventoryItem> {
        self.items
            .values()
            .filter(|i| i.current_stock <= i.min_reorder_threshold)
            .collect()
    }
}
