//! Resonance - Open Singing Synthesis Platform
//! 
//! A Rust + React rewrite of OpenUtau

pub mod audio;
pub mod midi;
pub mod format;
pub mod plugin;

use audio::AudioEngine;
use midi::{Note, NoteTrack, NoteClipboard, NoteClipboardItem};
use midi::note_track;
use format::UstxFile;
use format::ustx::{TrackData, NoteData};
use plugin::resampler::{Resampler, builtin::WorldlineResampler};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use tracing_appender::rolling::{RollingFileAppender, Rotation};

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

// Global clipboard for note operations
static mut NOTE_CLIPBOARD: Lazy<Mutex<NoteClipboard>> = Lazy::new(|| Mutex::new(NoteClipboard::default()));

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

// ============================================================================
// Note Track Management Commands
// ============================================================================

/// Create a new note track
#[tauri::command]
fn create_note_track(name: String, color: Option<u32>) -> NoteTrack {
    match color {
        Some(c) => NoteTrack::new(name).with_color(c),
        None => NoteTrack::new(name),
    }
}

/// Add a note to a track
#[tauri::command]
fn add_note_to_track(
    track: &mut NoteTrack,
    pitch: u8,
    velocity: u8,
    start: u64,
    duration: u64,
) -> Result<usize, String> {
    let note = Note::new(pitch, velocity, start, duration);
    let idx = track.add_note(note);
    Ok(idx)
}

/// Remove a note from a track by index
#[tauri::command]
fn remove_note(track: &mut NoteTrack, index: usize) -> Result<Note, String> {
    track.remove_note(index)
        .ok_or_else(|| format!("Note at index {} not found", index))
}

/// Move a note to a new position
#[tauri::command]
fn move_note(
    track: &mut NoteTrack,
    index: usize,
    new_start: u64,
    new_pitch: Option<u8>,
) -> Result<usize, String> {
    track.move_note(index, new_start, new_pitch)
        .ok_or_else(|| format!("Failed to move note at index {}", index))
}

/// Duplicate notes by indices
#[tauri::command]
fn duplicate_notes(track: &mut NoteTrack, indices: Vec<usize>) -> Result<Vec<usize>, String> {
    Ok(track.duplicate_notes(&indices))
}

// ============================================================================
// Note Clipboard Commands (Cut/Copy/Paste)
// ============================================================================

/// Copy notes to clipboard
#[tauri::command]
fn copy_notes(track: &NoteTrack, indices: Vec<usize>, track_index: usize) -> Result<NoteClipboard, String> {
    let items: Vec<NoteClipboardItem> = indices
        .iter()
        .filter_map(|&i| track.get_note(i))
        .map(NoteClipboardItem::from)
        .collect();
    
    Ok(NoteClipboard {
        notes: items,
        source_track: Some(track_index),
    })
}

/// Cut notes (copy and remove)
#[tauri::command]
fn cut_notes(track: &mut NoteTrack, indices: Vec<usize>, track_index: usize) -> Result<NoteClipboard, String> {
    // First copy
    let items: Vec<NoteClipboardItem> = indices
        .iter()
        .filter_map(|&i| track.get_note(i).cloned())
        .map(|n| NoteClipboardItem {
            pitch: n.pitch,
            velocity: n.velocity,
            start: n.start,
            duration: n.duration,
        })
        .collect();
    
    // Then remove in reverse order to maintain indices
    let mut sorted_indices = indices.clone();
    sorted_indices.sort_by(|a, b| b.cmp(a)); // descending
    for &idx in &sorted_indices {
        track.remove_note(idx);
    }
    
    Ok(NoteClipboard {
        notes: items,
        source_track: Some(track_index),
    })
}

/// Paste notes from clipboard to track
#[tauri::command]
fn paste_notes(track: &mut NoteTrack, clipboard: NoteClipboard, offset: u64) -> Vec<usize> {
    let mut new_indices = Vec::new();
    
    for item in &clipboard.notes {
        let note = Note::new(
            item.pitch,
            item.velocity,
            item.start + offset,
            item.duration,
        );
        let idx = track.add_note(note);
        new_indices.push(idx);
    }
    
    new_indices
}

// ============================================================================
// USTX Project Commands
// ============================================================================

/// Add a note to USTX track
#[tauri::command]
fn add_note_to_ustx_track(
    project: &mut UstxFile,
    track_index: usize,
    position: u64,
    duration: u64,
    pitch: i32,
    velocity: u32,
) -> Result<usize, String> {
    let track = project.get_track_mut(track_index)
        .ok_or_else(|| format!("Track {} not found", track_index))?;
    
    let note = NoteData::new(position, duration, pitch, velocity);
    Ok(track.add_note(note))
}

/// Remove a note from USTX track
#[tauri::command]
fn remove_ustx_note(
    project: &mut UstxFile,
    track_index: usize,
    note_index: usize,
) -> Result<(), String> {
    let track = project.get_track_mut(track_index)
        .ok_or_else(|| format!("Track {} not found", track_index))?;
    
    track.remove_note(note_index)
        .map(|_| ())
        .ok_or_else(|| format!("Note {} not found in track", note_index))
}

/// Move a note in USTX project
#[tauri::command]
fn move_ustx_note(
    project: &mut UstxFile,
    track_index: usize,
    note_index: usize,
    new_position: u64,
    new_pitch: Option<i32>,
) -> Result<(), String> {
    let track = project.get_track_mut(track_index)
        .ok_or_else(|| format!("Track {} not found", track_index))?;
    
    if note_index >= track.notes.len() {
        return Err(format!("Note {} not found in track", note_index));
    }
    
    let note = &mut track.notes[note_index];
    note.position = new_position;
    if let Some(pitch) = new_pitch {
        note.pitch = pitch.clamp(0, 127);
    }
    
    // Re-sort
    track.notes.sort_by_key(|n| n.position);
    
    Ok(())
}

/// Add a new track to USTX project
#[tauri::command]
fn add_ustx_track(project: &mut UstxFile, name: String, color: Option<u32>) -> usize {
    let track = TrackData {
        name,
        color,
        notes: vec![],
    };
    project.add_track(track)
}

/// Remove a track from USTX project
#[tauri::command]
fn remove_ustx_track(project: &mut UstxFile, index: usize) -> Result<(), String> {
    project.remove_track(index)
        .map(|_| ())
        .ok_or_else(|| {
            if project.track_count() <= 1 {
                "Cannot remove the last track".to_string()
            } else {
                format!("Track {} not found", index)
            }
        })
}

/// Validate USTX project
#[tauri::command]
fn validate_project(project: &mut UstxFile) -> Result<(), String> {
    project.validate()
        .map_err(|errors| errors.join("; "))
}

/// Get track info
#[tauri::command]
fn get_track_info(project: &UstxFile, track_index: usize) -> Result<String, String> {
    let track = project.get_track(track_index)
        .ok_or_else(|| format!("Track {} not found", track_index))?;
    
    Ok(format!(
        "Track: {} | Notes: {} | Duration: {} ticks",
        track.name,
        track.note_count(),
        track.duration()
    ))
}

/// Transpose notes in track
#[tauri::command]
fn transpose_track_notes(track: &mut NoteTrack, semitones: i32) {
    track.transpose(semitones);
}

/// Quantize note positions
#[tauri::command]
fn quantize_track_notes(track: &mut NoteTrack, grid: u64) {
    track.quantize(grid);
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
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Audio commands
            create_audio_engine,
            play_audio,
            stop_audio,
            get_audio_status,
            get_project_info,
            create_note,
            test_resampler,
            get_version,
            // File I/O commands
            format::io::load_ustx_file,
            format::io::save_ustx_file,
            format::io::create_new_project,
            format::io::get_default,
            format::midi_io::import_midi,
            format::midi_io::export_midi,
            // NoteTrack commands
            create_note_track,
            add_note_to_track,
            remove_note,
            move_note,
            duplicate_notes,
            copy_notes,
            cut_notes,
            paste_notes,
            transpose_track_notes,
            quantize_track_notes,
            // USTX project commands
            add_note_to_ustx_track,
            remove_ustx_note,
            move_ustx_note,
            add_ustx_track,
            remove_ustx_track,
            validate_project,
            get_track_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
