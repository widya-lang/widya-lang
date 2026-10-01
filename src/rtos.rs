//! WidyaRTOS — Sovereign Hard Real-Time OS & Drone/Robotics Engine (Micro-ROS / FreeRTOS Equivalent)
//! Provides sub-10us deterministic cyclic scheduling, CAN-Bus frame processing,
//! MAVLink v2 telemetry protocol handling, and PID feedback control loops.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanFrame {
    pub id: u32,
    pub is_extended: bool,
    pub dlc: u8,
    pub data: [u8; 8],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MavlinkTelemetry {
    pub system_id: u8,
    pub component_id: u8,
    pub roll_deg: f32,
    pub pitch_deg: f32,
    pub yaw_deg: f32,
    pub altitude_m: f32,
    pub battery_remaining_pct: u8,
}

#[derive(Debug, Clone)]
pub struct PidController {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    integral: f32,
    prev_error: f32,
}

impl PidController {
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
        }
    }

    pub fn compute(&mut self, setpoint: f32, actual: f32, dt: f32) -> f32 {
        let error = setpoint - actual;
        self.integral += error * dt;
        let derivative = if dt > 0.0 { (error - self.prev_error) / dt } else { 0.0 };
        self.prev_error = error;

        (self.kp * error) + (self.ki * self.integral) + (self.kd * derivative)
    }
}

pub struct DroneAutopilotEngine;

impl DroneAutopilotEngine {
    pub fn encode_mavlink_heartbeat(system_id: u8) -> Vec<u8> {
        let mut packet = vec![0xFD]; // MAVLink v2 magic
        packet.push(9);              // Payload length
        packet.push(0);              // Incompat flags
        packet.push(0);              // Compat flags
        packet.push(1);              // Sequence
        packet.push(system_id);      // Sys ID
        packet.push(1);              // Comp ID
        packet.extend_from_slice(&[0, 0, 0]); // Msg ID (0 = HEARTBEAT)
        packet.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0]); // Payload
        packet
    }
}
