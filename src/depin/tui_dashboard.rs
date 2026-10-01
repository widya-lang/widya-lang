//! Interactive Terminal Dashboard (TUI) for WidyaOS DePIN Nodes (tui_dashboard.rs)
//! Menyediakan visualisasi real-time berbasis teks (ANSI / Unicode) untuk memantau performa GPU,
//! saldo komisi Rupiah Digital, status antrean tugas komputasi, dan konektivitas peer P2P.

use serde::{Deserialize, Serialize};

/// Snapshot Status Node untuk Ditampilkan di TUI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TuiNodeMetricsSnapshot {
    pub node_id: String,
    pub device_name: String,
    pub gpu_utilization_percent: u8,
    pub gpu_temp_celsius: f32,
    pub vram_used_mb: usize,
    pub vram_total_mb: usize,
    pub current_tflops: f64,
    pub wallet_balance_idr: u64,
    pub active_peers_count: usize,
    pub active_compute_job: Option<String>,
}

/// Renderer Dashboard Terminal TUI
pub struct DepinTuiDashboard;

impl DepinTuiDashboard {
    /// Render dashboard lengkap ke format teks string ANSI
    pub fn render_dashboard(metrics: &TuiNodeMetricsSnapshot) -> String {
        let vram_percent = ((metrics.vram_used_mb as f64) / (metrics.vram_total_mb.max(1) as f64) * 100.0) as usize;
        let job_str = metrics.active_compute_job.as_deref().unwrap_or("IDLE (Menunggu Tugas)");

        // Buat bar kemajuan visual [||||||....]
        let bar_len = 20;
        let filled = (metrics.gpu_utilization_percent as usize * bar_len) / 100;
        let gpu_bar: String = "█".repeat(filled) + &"░".repeat(bar_len.saturating_sub(filled));

        let formatted_balance = Self::format_number_with_commas(metrics.wallet_balance_idr);

        format!(
            r#"╔════════════════════════════════════════════════════════════════════════╗
║             ⚡ WIDYA-OS SOVEREIGN DEPIN COMPUTE NODE ⚡                ║
╠════════════════════════════════════════════════════════════════════════╣
║ Node ID     : {node_id:<25} Status  : ONLINE (PoUW Active)║
║ Perangkat   : {device:<53}║
║ Peer P2P    : {peers:<25} TFLOPS  : {tflops:<12.1} TFLOPS║
╠════════════════════════════════════════════════════════════════════════╣
║ [GPU BEBAN] : [{gpu_bar}] {gpu_util:>3}%  Suhu : {temp:>4.1}°C ║
║ [VRAM MEM]  : {vram_used:>5} MB / {vram_total:>5} MB ({vram_p:>3}%)                           ║
╠════════════════════════════════════════════════════════════════════════╣
║ Tugas Aktif : {job:<53}║
║ Saldo Dompet: Rp {balance:<50}║
╚════════════════════════════════════════════════════════════════════════╝"#,
            node_id = metrics.node_id,
            device = metrics.device_name,
            peers = format!("{} Nodes Terhubung", metrics.active_peers_count),
            tflops = metrics.current_tflops,
            gpu_bar = gpu_bar,
            gpu_util = metrics.gpu_utilization_percent,
            temp = metrics.gpu_temp_celsius,
            vram_used = metrics.vram_used_mb,
            vram_total = metrics.vram_total_mb,
            vram_p = vram_percent,
            job = job_str,
            balance = formatted_balance
        )
    }

    fn format_number_with_commas(n: u64) -> String {
        let s = n.to_string();
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        for (i, &ch) in chars.iter().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(',');
            }
            result.push(ch);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_dashboard_rendering() {
        let snapshot = TuiNodeMetricsSnapshot {
            node_id: "node_surabaya_laptop".to_string(),
            device_name: "Widya Laptop RTX 4060 (Mobile NPU)".to_string(),
            gpu_utilization_percent: 75,
            gpu_temp_celsius: 42.5,
            vram_used_mb: 6144,
            vram_total_mb: 8192,
            current_tflops: 25.0,
            wallet_balance_idr: 1_250_000,
            active_peers_count: 48,
            active_compute_job: Some("DeepSeek LLM Shard #12".to_string()),
        };

        let rendered = DepinTuiDashboard::render_dashboard(&snapshot);
        assert!(rendered.contains("WIDYA-OS SOVEREIGN DEPIN COMPUTE NODE"));
        assert!(rendered.contains("node_surabaya_laptop"));
        assert!(rendered.contains("42.5°C"));
        assert!(rendered.contains("DeepSeek LLM Shard #12"));
        assert!(rendered.contains("Rp 1,250,000"));
    }
}
