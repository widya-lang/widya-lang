//! Thermal & Battery-Aware Adaptive Compute Guard for WidyaOS (thermal_battery_guard.rs)
//! Melindungi perangkat keras pengguna (Laptop, Smartphone, Mini PC) dari overheating,
//! baterai cepat drop, dan penurunan kenyamanan saat dipakai aktif.

use serde::{Deserialize, Serialize};

/// Status Daya & Termal Perangkat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceHardwareState {
    pub device_id: String,
    pub battery_percentage: u8, // 0 - 100
    pub is_charging: bool,
    pub temperature_celsius: f32,
    pub is_user_interacting: bool, // Mouse/Keyboard active
}

/// Keputusan Kebijakan Throttling Komputasi
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeThrottlePolicy {
    FullCapacity100,   // Berjalan 100% beban komputasi
    Throttled50,       // Turunkan ke 50% daya
    Throttled25,       // Turunkan ke 25% daya
    EmergencyPause,    // Jeda komputasi segera (Cooldown / Save Battery)
}

/// Engine Pelindung Fisik & Termal Perangkat WidyaOS
pub struct ThermalBatteryGuard {
    max_safe_temp_celsius: f32,
    min_battery_threshold: u8,
}

impl ThermalBatteryGuard {
    pub fn new(max_temp: f32, min_battery: u8) -> Self {
        Self {
            max_safe_temp_celsius: max_temp,
            min_battery_threshold: min_battery,
        }
    }

    /// Evaluasi status perangkat dan tentukan batas beban komputasi AI
    pub fn evaluate_throttle_policy(&self, state: &DeviceHardwareState) -> ComputeThrottlePolicy {
        // 1. Prioritas Utama: Emergency Pause jika suhu terlalu panas
        if state.temperature_celsius >= self.max_safe_temp_celsius {
            return ComputeThrottlePolicy::EmergencyPause;
        }

        // 2. Jika baterai rendah dan tidak sedang dicas -> Segera jeda
        if !state.is_charging && state.battery_percentage <= self.min_battery_threshold {
            return ComputeThrottlePolicy::EmergencyPause;
        }

        // 3. Jika pengguna sedang memakai perangkat secara aktif -> Turunkan ke 25% agar tidak lag
        if state.is_user_interacting {
            return ComputeThrottlePolicy::Throttled25;
        }

        // 4. Jika suhu mendekati batas (dalam jarak 5 derajat) -> Throttle 50%
        if state.temperature_celsius >= (self.max_safe_temp_celsius - 5.0) {
            return ComputeThrottlePolicy::Throttled50;
        }

        // 5. Kondisi ideal: Colok daya + Suhu dingin + User idle -> 100% Full Capacity
        ComputeThrottlePolicy::FullCapacity100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_battery_guard_policies() {
        let guard = ThermalBatteryGuard::new(50.0, 20);

        // Kondisi 1: Suhu normal, dicas, user idle -> 100%
        let normal_state = DeviceHardwareState {
            device_id: "laptop_01".to_string(),
            battery_percentage: 85,
            is_charging: true,
            temperature_celsius: 38.0,
            is_user_interacting: false,
        };
        assert_eq!(guard.evaluate_throttle_policy(&normal_state), ComputeThrottlePolicy::FullCapacity100);

        // Kondisi 2: Overheating (52°C >= 50°C) -> EmergencyPause
        let hot_state = DeviceHardwareState {
            temperature_celsius: 52.0,
            ..normal_state.clone()
        };
        assert_eq!(guard.evaluate_throttle_policy(&hot_state), ComputeThrottlePolicy::EmergencyPause);

        // Kondisi 3: Baterai rendah (15% <= 20%) & tidak dicas -> EmergencyPause
        let low_battery_state = DeviceHardwareState {
            battery_percentage: 15,
            is_charging: false,
            temperature_celsius: 35.0,
            ..normal_state.clone()
        };
        assert_eq!(guard.evaluate_throttle_policy(&low_battery_state), ComputeThrottlePolicy::EmergencyPause);

        // Kondisi 4: User sedang mengetik -> Throttled25
        let user_active_state = DeviceHardwareState {
            is_user_interacting: true,
            ..normal_state
        };
        assert_eq!(guard.evaluate_throttle_policy(&user_active_state), ComputeThrottlePolicy::Throttled25);
    }
}
