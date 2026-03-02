use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::audio::buffer::AudioBuffer;
use crate::format::UstxFile;
use crate::format::render::{RenderFormat, RenderConfig};

/// Audio engine for synthesis with advanced playback control
pub struct AudioEngine {
    sample_rate: u32,
    channels: u16,
    buffer: Arc<Mutex<AudioBuffer>>,
    is_playing: bool,
    is_paused: bool,
    current_position: u64,
    playback_rate: f32,
    loop_enabled: bool,
    loop_start: u64,
    loop_end: u64,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(44100, 2))),
            is_playing: false,
            is_paused: false,
            current_position: 0,
            playback_rate: 1.0,
            loop_enabled: false,
            loop_start: 0,
            loop_end: u64::MAX,
        }
    }

    /// Create with custom settings
    pub fn with_settings(sample_rate: u32, channels: u16) -> Self {
        Self {
            sample_rate,
            channels,
            buffer: Arc::new(Mutex::new(AudioBuffer::new(sample_rate, channels))),
            is_playing: false,
            is_paused: false,
            current_position: 0,
            playback_rate: 1.0,
            loop_enabled: false,
            loop_start: 0,
            loop_end: u64::MAX,
        }
    }

    /// Play
    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.is_playing {
            self.is_paused = true;
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.is_paused {
            self.is_paused = false;
        }
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Stop
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        self.current_position = 0;
    }

    /// Seek to specific position (in ticks)
    pub fn seek_to(&mut self, position: u64) {
        self.current_position = position;
        // Clear buffer on seek for clean playback
        if let Ok(mut buf) = self.buffer.lock() {
            buf.clear();
        }
    }

    /// Set playback rate (0.5 - 2.0)
    pub fn set_playback_rate(&mut self, rate: f32) {
        self.playback_rate = rate.clamp(0.5, 2.0);
    }

    /// Get playback rate
    pub fn playback_rate(&self) -> f32 {
        self.playback_rate
    }

    /// Enable/disable loop mode
    pub fn set_loop_enabled(&mut self, enabled: bool) {
        self.loop_enabled = enabled;
    }

    /// Check if loop is enabled
    pub fn is_loop_enabled(&self) -> bool {
        self.loop_enabled
    }

    /// Set loop region
    pub fn set_loop_region(&mut self, start: u64, end: u64) {
        self.loop_start = start;
        self.loop_end = end;
    }

    /// Get loop start position
    pub fn loop_start(&self) -> u64 {
        self.loop_start
    }

    /// Get loop end position
    pub fn loop_end(&self) -> u64 {
        self.loop_end
    }

    /// Check if playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Get channels
    pub fn channels(&self) -> u16 {
        self.channels
    }

    /// Get buffer
    pub fn buffer(&self) -> Arc<Mutex<AudioBuffer>> {
        Arc::clone(&self.buffer)
    }

    /// Add stereo samples
    pub fn add_samples(&self, left: f32, right: f32) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.push_stereo(left, right);
        }
    }

    /// Get current position
    pub fn position(&self) -> u64 {
        self.current_position
    }

    /// Set position
    pub fn set_position(&mut self, pos: u64) {
        self.current_position = pos;
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = AudioEngine::new();
        assert_eq!(engine.sample_rate(), 44100);
    }
}
