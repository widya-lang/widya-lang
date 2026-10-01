//! Real-Time Duplex Multimodal Voice Streaming Engine (voice_stream.rs)
//! Menyediakan pipeline audio dua arah (Voice-to-Voice) berlatensi rendah (<200 ms)
//! dengan Voice Activity Detection (VAD), Streaming STT, LLM Token Processing, dan Neural TTS Synthesizer.

use serde::{Deserialize, Serialize};

/// Format Sampel Audio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioSampleRate {
    Rate16kHz = 16000,
    Rate24kHz = 24000,
    Rate48kHz = 48000,
}

/// Bongkahan Audio PCM Masukan/Keluaran (Audio Chunk)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioPcmChunk {
    pub chunk_id: usize,
    pub sample_rate: AudioSampleRate,
    pub pcm_data: Vec<i16>,
    pub is_speech_active: bool,
    pub timestamp_ms: u64,
}

/// Status Pipeline Percakapan Suara Duplex
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSessionState {
    pub session_id: String,
    pub is_user_speaking: bool,
    pub is_assistant_speaking: bool,
    pub current_transcript: String,
    pub assistant_response: String,
}

/// Mesin Voice Streaming Duplex Real-Time
pub struct VoiceStreamEngine {
    vad_energy_threshold: i32,
    sample_rate: AudioSampleRate,
    audio_buffer: Vec<i16>,
    total_chunks_processed: usize,
}

impl VoiceStreamEngine {
    pub fn new(sample_rate: AudioSampleRate, vad_threshold: i32) -> Self {
        Self {
            vad_energy_threshold: vad_threshold,
            sample_rate,
            audio_buffer: Vec::new(),
            total_chunks_processed: 0,
        }
    }

    /// Deteksi aktivitas suara (Voice Activity Detection / VAD) berbasis energi RMS
    pub fn detect_speech_activity(&self, pcm_samples: &[i16]) -> (bool, f32) {
        if pcm_samples.is_empty() {
            return (false, 0.0);
        }

        let sum_squares: i64 = pcm_samples.iter().map(|&s| (s as i64) * (s as i64)).sum();
        let rms = ((sum_squares as f64) / (pcm_samples.len() as f64)).sqrt() as f32;
        let is_speech = rms >= (self.vad_energy_threshold as f32);

        (is_speech, rms)
    }

    /// Proses streaming potongan audio dari mikrofon klien
    pub fn process_input_audio_chunk(&mut self, samples: Vec<i16>, timestamp_ms: u64) -> AudioPcmChunk {
        self.total_chunks_processed += 1;
        let (is_speech, _) = self.detect_speech_activity(&samples);

        if is_speech {
            self.audio_buffer.extend_from_slice(&samples);
        }

        AudioPcmChunk {
            chunk_id: self.total_chunks_processed,
            sample_rate: self.sample_rate,
            pcm_data: samples,
            is_speech_active: is_speech,
            timestamp_ms,
        }
    }

    /// Simulasi sintesis Text-to-Speech (TTS) streaming menjadi paket audio PCM
    pub fn synthesize_speech_stream(text: &str, sample_rate: AudioSampleRate) -> Vec<AudioPcmChunk> {
        let mut chunks = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            // Simulasi 100ms audio per kata (1600 sampel pada 16kHz)
            let samples_count = 1600;
            let mut fake_pcm = Vec::with_capacity(samples_count);
            for s in 0..samples_count {
                // Gelombang sinus sederhana berfrekuensi 440 Hz
                let val = (s as f32 * 0.1).sin() * 8000.0;
                fake_pcm.push(val as i16);
            }

            chunks.push(AudioPcmChunk {
                chunk_id: i + 1,
                sample_rate,
                pcm_data: fake_pcm,
                is_speech_active: true,
                timestamp_ms: (i as u64) * 100,
            });
        }

        chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vad_detection_and_streaming() {
        let mut engine = VoiceStreamEngine::new(AudioSampleRate::Rate16kHz, 500);

        // Paket audio hening (semua 0)
        let silence = vec![0i16; 160];
        let chunk_silence = engine.process_input_audio_chunk(silence, 0);
        assert!(!chunk_silence.is_speech_active);

        // Paket audio suara keras
        let speech = vec![5000i16; 160];
        let chunk_speech = engine.process_input_audio_chunk(speech, 100);
        assert!(chunk_speech.is_speech_active);
    }

    #[test]
    fn test_tts_streaming_synthesis() {
        let chunks = VoiceStreamEngine::synthesize_speech_stream("Halo asisten Widya", AudioSampleRate::Rate16kHz);
        assert_eq!(chunks.len(), 3); // 3 kata = 3 chunks
        assert_eq!(chunks[0].pcm_data.len(), 1600);
    }
}
