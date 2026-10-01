//! Engine Pengolahan & Editing Video Non-Linear (NLE) Widya-Lang (video.rs)
//! Mendukung Multi-Track Video/Audio Compositor, Keyframe Animation, Video Transitions (Dissolve, Wipe, Fade), dan Frame Synthesis.

use crate::multimedia::image::{ImageBuffer, RgbaPixel, BlendMode};
use std::collections::HashMap;

/// Kurva Easing / Interpolasi Animasi Keyframe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EasingType {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    Step,
}

impl EasingType {
    pub fn interpolate(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingType::Linear => t,
            EasingType::EaseInQuad => t * t,
            EasingType::EaseOutQuad => t * (2.0 - t),
            EasingType::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            EasingType::Step => if t >= 1.0 { 1.0 } else { 0.0 },
        }
    }
}

/// Titik Kunci Parameter Animasi (Keyframe)
#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe<T> {
    pub timestamp_ms: u64,
    pub value: T,
    pub easing: EasingType,
}

/// Jalur Animasi Keyframe untuk Nilai Numerik
#[derive(Debug, Clone, PartialEq)]
pub struct AnimationTrack {
    pub keyframes: Vec<Keyframe<f32>>,
}

impl AnimationTrack {
    pub fn new() -> Self {
        Self { keyframes: Vec::new() }
    }

    pub fn add_keyframe(&mut self, timestamp_ms: u64, value: f32, easing: EasingType) {
        self.keyframes.push(Keyframe { timestamp_ms, value, easing });
        self.keyframes.sort_by_key(|k| k.timestamp_ms);
    }

    pub fn evaluate(&self, timestamp_ms: u64, default_val: f32) -> f32 {
        if self.keyframes.is_empty() {
            return default_val;
        }

        if timestamp_ms <= self.keyframes.first().unwrap().timestamp_ms {
            return self.keyframes.first().unwrap().value;
        }

        if timestamp_ms >= self.keyframes.last().unwrap().timestamp_ms {
            return self.keyframes.last().unwrap().value;
        }

        // Cari dua keyframe yang mengapit timestamp
        for i in 0..self.keyframes.len() - 1 {
            let k1 = &self.keyframes[i];
            let k2 = &self.keyframes[i + 1];

            if timestamp_ms >= k1.timestamp_ms && timestamp_ms <= k2.timestamp_ms {
                let duration = (k2.timestamp_ms - k1.timestamp_ms) as f32;
                if duration == 0.0 {
                    return k2.value;
                }
                let raw_t = (timestamp_ms - k1.timestamp_ms) as f32 / duration;
                let eased_t = k1.easing.interpolate(raw_t);
                return k1.value + (k2.value - k1.value) * eased_t;
            }
        }

        default_val
    }
}

/// Jenis Transisi Video antar Dua Klip
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Cut,
    CrossDissolve,
    FadeToBlack,
    WipeLeft,
    WipeRight,
}

/// Transisi Efek Video
#[derive(Debug, Clone, PartialEq)]
pub struct VideoTransition {
    pub transition_type: TransitionType,
    pub start_time_ms: u64,
    pub duration_ms: u64,
}

/// Klip Video di dalam Track
#[derive(Debug, Clone)]
pub struct VideoClip {
    pub id: String,
    pub start_time_ms: u64,
    pub duration_ms: u64,
    pub source_frames: Vec<ImageBuffer>, // Buffer Frame Gambar
    pub source_fps: f32,
    pub opacity_track: AnimationTrack,
    pub pos_x_track: AnimationTrack,
    pub pos_y_track: AnimationTrack,
    pub blend_mode: BlendMode,
}

impl VideoClip {
    pub fn new(id: &str, start_time_ms: u64, duration_ms: u64, frames: Vec<ImageBuffer>, fps: f32) -> Self {
        Self {
            id: id.to_string(),
            start_time_ms,
            duration_ms,
            source_frames: frames,
            source_fps: fps,
            opacity_track: AnimationTrack::new(),
            pos_x_track: AnimationTrack::new(),
            pos_y_track: AnimationTrack::new(),
            blend_mode: BlendMode::Normal,
        }
    }

    /// Mengambil frame pada timestamp global timeline
    pub fn get_frame_at(&self, timeline_time_ms: u64) -> Option<(ImageBuffer, i32, i32, f32)> {
        if timeline_time_ms < self.start_time_ms || timeline_time_ms >= self.start_time_ms + self.duration_ms {
            return None;
        }

        let local_time_ms = timeline_time_ms - self.start_time_ms;
        let frame_index = ((local_time_ms as f32 / 1000.0) * self.source_fps) as usize;

        if self.source_frames.is_empty() {
            return None;
        }

        let idx = frame_index.min(self.source_frames.len() - 1);
        let frame = self.source_frames[idx].clone();

        let opacity = self.opacity_track.evaluate(local_time_ms, 1.0);
        let pos_x = self.pos_x_track.evaluate(local_time_ms, 0.0) as i32;
        let pos_y = self.pos_y_track.evaluate(local_time_ms, 0.0) as i32;

        Some((frame, pos_x, pos_y, opacity))
    }
}

/// Track Video yang memuat beberapa klip berurutan / terpisah
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: String,
    pub clips: Vec<VideoClip>,
    pub is_visible: bool,
}

impl VideoTrack {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            clips: Vec::new(),
            is_visible: true,
        }
    }

    pub fn add_clip(&mut self, clip: VideoClip) {
        self.clips.push(clip);
        self.clips.sort_by_key(|c| c.start_time_ms);
    }
}

/// Timeline Utama Engine Video NLE
pub struct VideoTimeline {
    pub width: u32,
    pub height: u32,
    pub fps: f32,
    pub total_duration_ms: u64,
    pub background_color: RgbaPixel,
    pub tracks: Vec<VideoTrack>,
    pub transitions: Vec<VideoTransition>,
}

impl VideoTimeline {
    pub fn new(width: u32, height: u32, fps: f32, duration_ms: u64) -> Self {
        Self {
            width,
            height,
            fps,
            total_duration_ms: duration_ms,
            background_color: RgbaPixel::BLACK,
            tracks: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: VideoTrack) {
        self.tracks.push(track);
    }

    pub fn add_transition(&mut self, transition: VideoTransition) {
        self.transitions.push(transition);
    }

    /// Render satu frame pada timestamp tertentu (Frame Synthesizer)
    pub fn render_frame(&self, timestamp_ms: u64) -> ImageBuffer {
        let mut canvas = ImageBuffer::new(self.width, self.height, self.background_color);

        // Komposisikan semua track dari bawah ke atas (Track 0 -> Track N)
        for track in &self.tracks {
            if !track.is_visible {
                continue;
            }

            for clip in &track.clips {
                if let Some((frame_img, px, py, opacity)) = clip.get_frame_at(timestamp_ms) {
                    canvas.blend_layer(&frame_img, px, py, clip.blend_mode, opacity);
                }
            }
        }

        // Terapkan Transisi Global jika ada pada timestamp ini
        for trans in &self.transitions {
            if timestamp_ms >= trans.start_time_ms && timestamp_ms <= trans.start_time_ms + trans.duration_ms {
                let progress = (timestamp_ms - trans.start_time_ms) as f32 / trans.duration_ms as f32;
                Self::apply_transition(&mut canvas, trans.transition_type, progress);
            }
        }

        canvas
    }

    /// Menerapkan efek visual transisi ke kanvas hasil render
    fn apply_transition(canvas: &mut ImageBuffer, t_type: TransitionType, progress: f32) {
        let p = progress.clamp(0.0, 1.0);
        match t_type {
            TransitionType::Cut => {}
            TransitionType::FadeToBlack => {
                // Di paruh pertama fade out ke hitam (1.0 -> 0.0), di paruh kedua fade in dari hitam (0.0 -> 1.0)
                let factor = if p < 0.5 {
                    1.0 - (p * 2.0)
                } else {
                    (p - 0.5) * 2.0
                };
                for pixel in &mut canvas.pixels {
                    pixel.r = ((pixel.r as f32) * factor) as u8;
                    pixel.g = ((pixel.g as f32) * factor) as u8;
                    pixel.b = ((pixel.b as f32) * factor) as u8;
                }
            }
            TransitionType::CrossDissolve => {
                // Opacity modifier
                for pixel in &mut canvas.pixels {
                    pixel.a = ((pixel.a as f32) * p) as u8;
                }
            }
            TransitionType::WipeLeft => {
                let wipe_x = (canvas.width as f32 * p) as u32;
                for y in 0..canvas.height {
                    for x in 0..wipe_x.min(canvas.width) {
                        canvas.set_pixel(x, y, RgbaPixel::BLACK);
                    }
                }
            }
            TransitionType::WipeRight => {
                let wipe_x = (canvas.width as f32 * (1.0 - p)) as u32;
                for y in 0..canvas.height {
                    for x in wipe_x..canvas.width {
                        canvas.set_pixel(x, y, RgbaPixel::BLACK);
                    }
                }
            }
        }
    }

    /// Menghasilkan total jumlah frame yang akan di-export
    pub fn total_frames(&self) -> usize {
        ((self.total_duration_ms as f32 / 1000.0) * self.fps).ceil() as usize
    }
}
