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
use format::render::{RenderFormat, RenderConfig, AudioRenderer, start_render, cancel_render as cancel_render_impl, get_render_progress as get_render_progress_impl};
use plugin::resampler::{Resampler, builtin::WorldlineResampler};
use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;

static AUDIO_ENGINE: Lazy<Mutex<AudioEngine>> = Lazy::new(|| Mutex::new(AudioEngine::new()));

// Global clipboard for note operations
static mut NOTE_CLIPBOARD: Lazy<Mutex<NoteClipboard>> = Lazy::new(|| Mutex::new(NoteClipboard::default()));

/// Initialize the audio engine
#[tauri::command]
fn create_audio_engine(sample_rate: u32, channels: u16) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    *engine = AudioEngine::with_settings(sample_rate, channels);
    Ok(format!("Audio engine created: {}Hz, {} channels", sample_rate, channels))
}

/// Play audio (generates test tone)
#[tauri::command]
fn play_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 960);
    
    let samples: Vec<f32> = buffer.to_vec();
    for chunk in samples.chunks(2) {
        if chunk.len() == 2 {
            engine.add_samples(chunk[0], chunk[1]);
        }
    }
    
    engine.play();
    Ok("Playing".to_string())
}

/// Stop audio
#[tauri::command]
fn stop_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.stop();
    Ok("Stopped".to_string())
}

/// Get audio engine status
#[tauri::command]
fn get_audio_status() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "Playing: {}, Paused: {}, Sample Rate: {}Hz, Rate: {:.1}x, Loop: {}",
        engine.is_playing(),
        engine.is_paused(),
        engine.sample_rate(),
        engine.playback_rate(),
        engine.is_loop_enabled()
    ))
}

// ============================================================================
// Advanced Playback Control Commands
// ============================================================================

/// Pause audio playback
#[tauri::command]
fn pause_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.pause();
    Ok("Paused".to_string())
}

/// Resume audio playback
#[tauri::command]
fn resume_audio() -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.resume();
    Ok("Resumed".to_string())
}

/// Seek to specific position (in ticks)
#[tauri::command]
fn seek_audio(position: u64) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.seek_to(position);
    Ok(format!("Seeked to position {}", position))
}

/// Set playback rate (0.5 - 2.0)
#[tauri::command]
fn set_playback_rate(rate: f32) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.set_playback_rate(rate);
    Ok(format!("Playback rate set to {:.1}x", rate))
}

/// Get current playback position (in ticks)
#[tauri::command]
fn get_current_position() -> Result<u64, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(engine.position())
}

/// Set loop mode
#[tauri::command]
fn set_loop_mode(enabled: bool, start: Option<u64>, end: Option<u64>) -> Result<String, String> {
    let mut engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    engine.set_loop_enabled(enabled);
    if let (Some(s), Some(e)) = (start, end) {
        engine.set_loop_region(s, e);
    }
    Ok(format!("Loop mode: {}, region: {}-{}", enabled, engine.loop_start(), engine.loop_end()))
}

/// Get playback info
#[tauri::command]
fn get_playback_info() -> Result<String, String> {
    let engine = AUDIO_ENGINE.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "{{\"playing\": {}, \"paused\": {}, \"position\": {}, \"rate\": {}, \"loop\": {}, \"loopStart\": {}, \"loopEnd\": {}}}",
        engine.is_playing(),
        engine.is_paused(),
        engine.position(),
        engine.playback_rate(),
        engine.is_loop_enabled(),
        engine.loop_start(),
        engine.loop_end()
    ))
}

// ============================================================================
// Audio Rendering Commands
// ============================================================================

/// Start rendering project to audio file
#[tauri::command]
fn start_render(
    project: UstxFile,
    output_path: String,
    format: String,
    sample_rate: u32,
    bit_depth: u16,
) -> Result<String, String> {
    let fmt = match format.to_lowercase().as_str() {
        "wav16" => RenderFormat::Wav16,
        "wav24" => RenderFormat::Wav24,
        "wav32" => RenderFormat::Wav32,
        "mp3" => RenderFormat::Mp3,
        "flac" => RenderFormat::Flac,
        _ => return Err(format!("Unsupported format: {}", format)),
    };
    
    let path = std::path::Path::new(&output_path);
    
    start_render(&project, path, fmt, sample_rate, bit_depth)
        .map_err(|e| e.to_string())?;
    
    Ok(format!("Rendered to {}", output_path))
}

/// Cancel ongoing render
#[tauri::command]
fn cancel_render() -> Result<String, String> {
    cancel_render_impl();
    Ok("Render cancelled".to_string())
}

/// Get render progress (0.0 - 100.0)
#[tauri::command]
fn get_render_progress() -> Result<f32, String> {
    Ok(get_render_progress_impl())
}

/// Get supported render formats
#[tauri::command]
fn get_render_formats() -> Result<Vec<String>, String> {
    Ok(vec![
        "wav16".to_string(),
        "wav24".to_string(),
        "wav32".to_string(),
        "mp3".to_string(),
        "flac".to_string(),
    ])
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
    Ok(format!("Created note: {} at {} for {} ticks", note.name(), start, duration))
}

/// Test resampler
#[tauri::command]
fn test_resampler() -> Result<String, String> {
    let resampler = WorldlineResampler::new(44100);
    let buffer = resampler.resample("a", 60, 100, 480);
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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            // Audio commands
            create_audio_engine,
            play_audio,
            stop_audio,
            get_audio_status,
            // Advanced playback control
            pause_audio,
            resume_audio,
            seek_audio,
            set_playback_rate,
            get_current_position,
            set_loop_mode,
            get_playback_info,
            // Render commands
            start_render,
            cancel_render,
            get_render_progress,
            get_render_formats,
            // Project commands
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
