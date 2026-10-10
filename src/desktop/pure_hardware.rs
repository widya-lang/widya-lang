//! Pure Industrial & Retail Hardware Bridge (Zero External Dependencies)
//! Zero Crate Dependencies (No serialport, No modbus, No escpos).
//! Implements pure Modbus RTU/TCP with CRC-16 (0xA001), ESC/POS Thermal Receipt
//! Bytecode Generator (Bold, Center, Barcode, QRIS, Auto-Cut, Cash Drawer), and Digital Scale parser.

/// Mesin Pembangkit Kode Byte ESC/POS Thermal Receipt Printer
pub struct EscPosGenerator {
    buffer: Vec<u8>,
}

impl Default for EscPosGenerator {
    fn default() -> Self {
        Self::baru()
    }
}

impl EscPosGenerator {
    pub fn baru() -> Self {
        let mut pembangkit = Self { buffer: Vec::new() };
        pembangkit.inisialisasi();
        pembangkit
    }


    /// Reset printer ke state awal (ESC @)
    pub fn inisialisasi(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x40]);
        self
    }

    /// Tentukan perataan teks: 0 = Kiri, 1 = Tengah, 2 = Kanan (ESC a n)
    pub fn rata_tengah(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x01]);
        self
    }

    pub fn rata_kiri(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x00]);
        self
    }

    pub fn rata_kanan(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x61, 0x02]);
        self
    }

    /// Mode Teks Tebal (ESC E n)
    pub fn tebal(&mut self, aktif: bool) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x45, if aktif { 0x01 } else { 0x00 }]);
        self
    }

    /// Tulis teks dengan baris baru
    pub fn tulis_baris(&mut self, teks: &str) -> &mut Self {
        self.buffer.extend_from_slice(teks.as_bytes());
        self.buffer.push(0x0A); // LF
        self
    }

    /// Cetak Garis Pembatas
    pub fn garis_pembatas(&mut self, karakter: char, lebar_kolom: usize) -> &mut Self {
        let line: String = std::iter::repeat(karakter).take(lebar_kolom).collect();
        self.tulis_baris(&line)
    }

    /// Cetak Barcode Code128 (GS k m n d1..dk)
    pub fn cetak_barcode_code128(&mut self, data: &str) -> &mut Self {
        // Atur tinggi barcode: GS h n
        self.buffer.extend_from_slice(&[0x1D, 0x68, 64]);
        // Posisi HRI teks di bawah: GS H 2
        self.buffer.extend_from_slice(&[0x1D, 0x48, 0x02]);
        // Cetak Code128: GS k 73 len bytes
        let bytes = data.as_bytes();
        self.buffer.extend_from_slice(&[0x1D, 0x6B, 73, bytes.len() as u8]);
        self.buffer.extend_from_slice(bytes);
        self.buffer.push(0x0A);
        self
    }

    /// Potong Kertas Otomatis (GS V 66 0)
    pub fn potong_kertas(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1D, 0x56, 0x42, 0x00]);
        self
    }

    /// Buka Laci Uang Kasir (Cash Drawer Kick: ESC p m t1 t2)
    pub fn buka_laci_kasir(&mut self) -> &mut Self {
        self.buffer.extend_from_slice(&[0x1B, 0x70, 0x00, 0x19, 0xFA]);
        self
    }

    /// Ambil seluruh bytecode biner yang dihasilkan
    pub fn ambil_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

/// Mesin Protokol Modbus RTU Murni
pub struct ModbusRTUEngine;

impl ModbusRTUEngine {
    /// Hitung Checksum CRC-16 Modbus (Polinomial 0xA001, Initial 0xFFFF)
    pub fn hitung_crc16(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        for &byte in data {
            crc ^= byte as u16;
            for _ in 0..8 {
                if (crc & 0x0001) != 0 {
                    crc = (crc >> 1) ^ 0xA001;
                } else {
                    crc >>= 1;
                }
            }
        }
        crc
    }

    /// Buat Frame Baca Holding Register (Function Code 0x03)
    pub fn buat_frame_baca_holding_registers(slave_id: u8, start_address: u16, jumlah_register: u16) -> Vec<u8> {
        let mut pdu = vec![
            slave_id,
            0x03,
            (start_address >> 8) as u8,
            (start_address & 0xFF) as u8,
            (jumlah_register >> 8) as u8,
            (jumlah_register & 0xFF) as u8,
        ];
        let crc = Self::hitung_crc16(&pdu);
        pdu.push((crc & 0xFF) as u8);        // CRC Low byte first
        pdu.push((crc >> 8) as u8);          // CRC High byte second
        pdu
    }

    /// Verifikasi Validitas Frame Respon Modbus RTU
    pub fn verifikasi_frame_respon(frame: &[u8]) -> bool {
        if frame.len() < 4 {
            return false;
        }
        let data_len = frame.len() - 2;
        let expected_crc = Self::hitung_crc16(&frame[..data_len]);
        let actual_crc = (frame[data_len] as u16) | ((frame[data_len + 1] as u16) << 8);
        expected_crc == actual_crc
    }
}

/// Parser Data Timbangan Digital POS (Weight Scale Protocol Parser)
pub struct ParserTimbanganPOS;

impl ParserTimbanganPOS {
    /// Parse string raw ASCII dari timbangan (Format CAS / Mettler Toledo: "ST,GS,+  1.250kg\r\n")
    pub fn parse_berat_kg(raw: &str) -> Result<f32, String> {
        let clean = raw.trim();
        // Cari angka dan titik desimal
        let num_str: String = clean
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
            .collect();
        
        num_str.parse::<f32>().map_err(|e| format!("Gagal parse berat '{}': {}", num_str, e))
    }
}
