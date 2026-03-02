//! Resonance - Open Singing Synthesis Platform
//! 
//! A Rust + React rewrite of OpenUtau

pub mod audio;
pub mod midi;
pub mod format;
pub mod plugin;

use audio::AudioEngine;
use midi::Note;
use format::UstxFile;
use plugin::resampler::{Resampler, builtin::WorldlineResampler};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

static AUDIO_ENGINE: Lazy<Mutex<AudioEngine>> = Lazy::new(|| {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    
    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Logger already initialized");
    }
    
    info!("Resonance audio engine initializing...");
    Mutex::new(AudioEngine::new())
});

/// Initialize the audio engine
#[tauri::command]
fn create_audio_engine(sample_rate: u32, channels: u16) -> Result<String, String> {
    info!("Creating audio engine: {}Hz, {} channels", sample_rate, channels);
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| {
        error!("Failed to lock audio engine: {}", e);
        e.to_string()
    })?;
    *engine = AudioEngine::with_settings(sample_rate, channels);
    info!("Audio engine created successfully");
    Ok(format!("Audio engine created: {}Hz, {} channels", sample_rate, channels))
}

/// Play audio (generates test tone)
#[tauri::command]
fn play_audio() -> Result<String, String> {
    info!("Play audio command received");
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    // Use Worldline resampler to generate test tone
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 960);
    
    let samples: Vec<f32> = buffer.to_vec();
    for chunk in samples.chunks(2) {
        if chunk.len() == 2 {
            engine.add_samples(chunk[0], chunk[1]);
        }
    }
    
    engine.play();
    info!("Audio playback started");
    Ok("Playing".to_string())
}

/// Stop audio
#[tauri::command]
fn stop_audio() -> Result<String, String> {
    info!("Stop audio command received");
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.stop();
    info!("Audio playback stopped");
    Ok("Stopped".to_string())
}

/// Get audio buffer as samples (for Web Audio API playback)
#[tauri::command]
fn get_audio_samples() -> Result<Vec<f32>, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.get_samples())
}

/// Get audio buffer length
#[tauri::command]
fn get_audio_buffer_len() -> Result<usize, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.buffer_len())
}

/// Generate and render project to audio buffer
#[tauri::command]
fn render_project(project: UstxFile) -> Result<usize, String> {
    info!("Rendering project: {}", project.name);
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    // Clear existing buffer
    engine.clear_buffer();
    
    // Generate audio from project notes
    let sample_rate = engine.sample_rate() as f32;
    let resampler = WorldlineResampler::new(sample_rate as u32);
    
    for track in &project.tracks {
        for note in &track.notes {
            // Convert pitch to note name (simplified)
            let note_names = ["c", "d", "e", "f", "g", "a", "b"];
            let octave = (note.pitch / 12) - 1;
            let note_idx = note.pitch % 12;
            let note_name = format!("{}{}", note_names[(note_idx as usize) % 7], octave);
            
            // Resample note
            let buffer = resampler.resample(&note_name, note.pitch as u8, note.velocity as u8, note.duration);
            
            // Add to audio buffer
            for chunk in buffer.to_vec().chunks(2) {
                if chunk.len() == 2 {
                    engine.add_samples(chunk[0], chunk[1]);
                } else if chunk.len() == 1 {
                    engine.add_sample(chunk[0]);
                }
            }
        }
    }
    
    let len = engine.buffer_len();
    info!("Rendered {} samples", len);
    Ok(len)
}

/// Get audio engine status
#[tauri::command]
fn get_audio_status() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!("Playing: {}, Sample Rate: {}Hz", engine.is_playing(), engine.sample_rate()))
}

/// Get project info
#[tauri::command]
fn get_project_info(project: UstxFile) -> Result<String, String> {
    Ok(format!(
        "Project: {} | BPM: {} | Tracks: {}",
        project.name,
        project.bpm,
        project.tracks.len()
    ))
}

/// Create a new note
#[tauri::command]
fn create_note(pitch: u8, velocity: u8, start: u64, duration: u64) -> Result<String, String> {
    let note = Note::new(pitch, velocity, start, duration);
    info!("Created note: {} at {} for {} ticks", note.name(), start, duration);
    Ok(format!("Created note: {} at {} for {} ticks", note.name(), start, duration))
}

/// Test resampler
#[tauri::command]
fn test_resampler() -> Result<String, String> {
    info!("Testing resampler");
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 480);
    info!("Resampler generated {} samples", buffer.len());
    Ok(format!("Resampler generated {} samples", buffer.len()))
}

/// Get app version
#[tauri::command]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    info!("Resonance v{} starting...", env!("CARGO_PKG_VERSION"));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_audio_engine,
            play_audio,
            stop_audio,
            get_audio_samples,
            get_audio_buffer_len,
            render_project,
            get_audio_status,
            get_project_info,
            create_note,
            test_resampler,
            get_version,
            format::io::load_ustx_file,
            format::io::save_ustx_file,
            format::io::create_new_project,
            format::io::get_default,
            format::midi_io::import_midi,
            format::midi_io::export_midi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}