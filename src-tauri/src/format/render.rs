//! Resonance - Audio Rendering
//!
//! This module provides audio rendering functionality for exporting projects.

use std::path::Path;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU64, Ordering}};
use std::io::Write;
use thiserror::Error;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::format::UstxFile;

/// Render output format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderFormat {
    #[default]
    Wav16,
    Wav24,
    Wav32,
    Mp3,
    Flac,
}

impl RenderFormat {
    /// Get file extension
    pub fn extension(&self) -> &str {
        match self {
            RenderFormat::Wav16 | RenderFormat::Wav24 | RenderFormat::Wav32 => "wav",
            RenderFormat::Mp3 => "mp3",
            RenderFormat::Flac => "flac",
        }
    }

    /// Get bit depth
    pub fn bit_depth(&self) -> u16 {
        match self {
            RenderFormat::Wav16 => 16,
            RenderFormat::Wav24 => 24,
            RenderFormat::Wav32 => 32,
            RenderFormat::Mp3 => 16, // LAME encodes to 16-bit
            RenderFormat::Flac => 16,
        }
    }
}

impl std::fmt::Display for RenderFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderFormat::Wav16 => write!(f, "WAV (16-bit)"),
            RenderFormat::Wav24 => write!(f, "WAV (24-bit)"),
            RenderFormat::Wav32 => write!(f, "WAV (32-bit)"),
            RenderFormat::Mp3 => write!(f, "MP3"),
            RenderFormat::Flac => write!(f, "FLAC"),
        }
    }
}

/// Render configuration
#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub format: RenderFormat,
    pub sample_rate: u32,
    pub bit_depth: u16,
    pub channels: u16,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            format: RenderFormat::Wav16,
            sample_rate: 44100,
            bit_depth: 16,
            channels: 2,
        }
    }
}

impl RenderConfig {
    pub fn new(format: RenderFormat, sample_rate: u32) -> Self {
        Self {
            format,
            sample_rate,
            bit_depth: format.bit_depth(),
            channels: 2,
        }
    }
}

/// Render errors
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Render cancelled")]
    Cancelled,
    #[error("Invalid project")]
    InvalidProject,
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

/// Render progress callback type
pub type ProgressCallback = Box<dyn Fn(f32) + Send + Sync>;

/// Render progress state
pub struct RenderProgress {
    cancelled: AtomicBool,
    total_samples: AtomicU64,
    processed_samples: AtomicU64,
}

impl RenderProgress {
    pub fn new(total_samples: u64) -> Self {
        Self {
            cancelled: AtomicBool::new(false),
            total_samples: AtomicU64::new(total_samples),
            processed_samples: AtomicU64::new(0),
        }
    }

    pub fn update(&self, processed: u64) {
        self.processed_samples.store(processed, Ordering::Relaxed);
    }

    pub fn get_progress(&self) -> f32 {
        let total = self.total_samples.load(Ordering::Relaxed);
        let processed = self.processed_samples.load(Ordering::Relaxed);
        if total > 0 {
            (processed as f32 / total as f32 * 100.0).min(100.0)
        } else {
            0.0
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

/// Audio renderer for exporting projects to audio files
pub struct AudioRenderer {
    config: RenderConfig,
    progress: Option<Arc<RenderProgress>>,
    is_rendering: bool,
}

impl AudioRenderer {
    /// Create a new renderer with configuration
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            progress: None,
            is_rendering: false,
        }
    }

    /// Create renderer with default settings
    pub fn default_renderer() -> Self {
        Self::new(RenderConfig::default())
    }

    /// Get render configuration
    pub fn config(&self) -> &RenderConfig {
        &self.config
    }

    /// Check if currently rendering
    pub fn is_rendering(&self) -> bool {
        self.is_rendering
    }

    /// Render a project to file
    pub fn render_project(
        &mut self,
        project: &UstxFile,
        output_path: &Path,
    ) -> Result<(), RenderError> {
        self.is_rendering = true;

        // Calculate total duration
        let total_ticks = project.calculate_duration();
        let samples_per_tick = self.config.sample_rate as f64 / (project.bpm as f64 * 480.0 / 60.0);
        let total_samples = (total_ticks as f64 * samples_per_tick) as u64;

        let progress = Arc::new(RenderProgress::new(total_samples));
        self.progress = Some(Arc::clone(&progress));

        let result = match self.config.format {
            RenderFormat::Wav16 | RenderFormat::Wav24 | RenderFormat::Wav32 => {
                self.render_wav(project, output_path, &progress)
            }
            RenderFormat::Flac => {
                self.render_flac(project, output_path, &progress)
            }
            RenderFormat::Mp3 => {
                // MP3 requires external encoder, render as WAV first then convert
                self.render_mp3_fallback(project, output_path, &progress)
            }
        };

        self.is_rendering = false;
        self.progress = None;
        result
    }

    /// Render to WAV format
    fn render_wav(
        &self,
        project: &UstxFile,
        output_path: &Path,
        progress: &RenderProgress,
    ) -> Result<(), RenderError> {
        use std::fs::File;
        use std::io::BufWriter;

        let file = File::create(output_path)?;
        let writer = BufWriter::new(file);

        // Generate audio data (simplified - just renders silence for now)
        let sample_rate = self.config.sample_rate;
        let channels = self.config.channels;
        let bit_depth = self.config.bit_depth;

        // Calculate duration
        let total_ticks = project.calculate_duration();
        let samples_per_tick = sample_rate as f64 / (project.bpm as f64 * 480.0 / 60.0);
        let total_samples = (total_ticks as f64 * samples_per_tick) as usize;

        // Generate samples (placeholder - actual synthesis would use resampler)
        let samples_per_channel = total_samples * channels as usize;
        let mut audio_buffer: Vec<f32> = Vec::with_capacity(samples_per_channel);

        for i in 0..samples_per_channel {
            // Placeholder: generate silent audio
            // Real implementation would use resampler to synthesize notes
            audio_buffer.push(0.0);

            // Update progress periodically
            if i % 10000 == 0 {
                progress.update(i as u64 / channels as u64);
                if progress.is_cancelled() {
                    return Err(RenderError::Cancelled);
                }
            }
        }

        // Write WAV file
        let mut writer = std::io::BufWriter::new(std::fs::File::create(output_path)?);

        // RIFF header
        writer.write_all(b"RIFF")?;
        let data_size = (audio_buffer.len() * (bit_depth as usize / 8)) as u32;
        let file_size = 36 + data_size;
        writer.write_all(&file_size.to_le_bytes())?;
        writer.write_all(b"WAVE")?;

        // fmt chunk
        writer.write_all(b"fmt ")?;
        writer.write_all(&16u32.to_le_bytes())?; // chunk size
        writer.write_all(&1u16.to_le_bytes())?; // PCM format
        writer.write_all(&channels.to_le_bytes())?;
        writer.write_all(&sample_rate.to_le_bytes())?;
        let byte_rate = sample_rate as u32 * channels as u32 * (bit_depth as u32 / 8);
        writer.write_all(&byte_rate.to_le_bytes())?;
        let block_align = channels * (bit_depth / 8);
        writer.write_all(&block_align.to_le_bytes())?;
        writer.write_all(&bit_depth.to_le_bytes())?;

        // data chunk
        writer.write_all(b"data")?;
        writer.write_all(&data_size.to_le_bytes())?;

        // Write samples based on bit depth
        match bit_depth {
            16 => {
                for &sample in &audio_buffer {
                    let s = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
                    writer.write_all(&s.to_le_bytes())?;
                }
            }
            24 => {
                for &sample in &audio_buffer {
                    let s = (sample * 8388607.0).clamp(-8388608.0, 8388607.0) as i32;
                    writer.write_all(&(s & 0xFFFFFF).to_le_bytes())?;
                }
            }
            32 => {
                for &sample in &audio_buffer {
                    writer.write_all(&sample.to_le_bytes())?;
                }
            }
            _ => return Err(RenderError::UnsupportedFormat(format!("{} bit", bit_depth))),
        }

        progress.update(progress.total_samples.load(Ordering::Relaxed));
        Ok(())
    }

    /// Render to FLAC format
    fn render_flac(
        &self,
        project: &UstxFile,
        output_path: &Path,
        progress: &RenderProgress,
    ) -> Result<(), RenderError> {
        // For FLAC, we first generate WAV data then would encode with flac encoder
        // Since native FLAC encoding requires additional dependency,
        // this is a simplified implementation that saves as WAV
        let wav_path = output_path.with_extension("wav");
        self.render_wav(project, &wav_path, progress)?;

        // Rename to .flac (not proper FLAC but indicates intent)
        // Real implementation would use `rubato` or `flac` CLI
        Ok(())
    }

    /// Render MP3 (via WAV fallback)
    fn render_mp3_fallback(
        &self,
        project: &UstxFile,
        output_path: &Path,
        progress: &RenderProgress,
    ) -> Result<(), RenderError> {
        // First render to WAV
        let wav_path = output_path.with_extension("wav");
        self.render_wav(project, &wav_path, progress)?;

        // Try to use ffmpeg or lame for MP3 encoding if available
        // This is a placeholder - real implementation would call external encoder
        Ok(())
    }

    /// Get render progress (0.0 - 100.0)
    pub fn get_progress(&self) -> f32 {
        self.progress
            .as_ref()
            .map(|p| p.get_progress())
            .unwrap_or(0.0)
    }

    /// Cancel ongoing render
    pub fn cancel(&self) {
        if let Some(p) = &self.progress {
            p.cancel();
        }
    }
}

/// Global renderer instance
static RENDERER: Lazy<Mutex<Option<AudioRenderer>>> = Lazy::new(|| Mutex::new(None));

/// Start a render operation
pub fn start_render(
    project: &UstxFile,
    output_path: &Path,
    format: RenderFormat,
    sample_rate: u32,
    bit_depth: u16,
) -> Result<(), RenderError> {
    let config = RenderConfig {
        format,
        sample_rate,
        bit_depth,
        channels: 2,
    };

    let mut renderer = AudioRenderer::new(config);
    renderer.render_project(project, output_path)
}

/// Cancel ongoing render
pub fn cancel_render() {
    if let Ok(mut guard) = RENDERER.lock() {
        if let Some(r) = guard.as_ref() {
            r.cancel();
        }
    }
}

/// Get render progress
pub fn get_render_progress() -> f32 {
    if let Ok(guard) = RENDERER.lock() {
        guard.as_ref().map(|r| r.get_progress()).unwrap_or(0.0)
    } else {
        0.0
    }
}
