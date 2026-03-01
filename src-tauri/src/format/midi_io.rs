use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use crate::format::ustx::{UstxFile, TrackData, NoteData};

/// Import MIDI file and convert to USTX project (basic implementation)
#[tauri::command]
pub fn import_midi(path: String) -> Result<UstxFile, String> {
    // Basic MIDI parsing - read as binary and extract note events
    let data = std::fs::read(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Check MIDI magic number
    if data.len() < 14 || &data[0..4] != b"MThd" {
        return Err("Not a valid MIDI file".to_string());
    }
    
    // Parse header
    let format = u16::from_be_bytes([data[8], data[9]]);
    let num_tracks = u16::from_be_bytes([data[10], data[11]]);
    let division = u16::from_be_bytes([data[12], data[13]]);
    
    println!("MIDI: format={}, tracks={}, division={}", format, num_tracks, division);
    
    let mut notes: Vec<NoteData> = Vec::new();
    let mut pos = 14;
    
    // Process tracks
    for _track in 0..num_tracks {
        if pos + 8 > data.len() { break; }
        
        if &data[pos..pos+4] != b"MTrk" {
            break;
        }
        
        let track_len = u32::from_be_bytes([data[pos+4], data[pos+5], data[pos+6], data[pos+7]]) as usize;
        pos += 8;
        
        if pos + track_len > data.len() { break; }
        
        // Simple note event parsing
        let track_data = &data[pos..pos+track_len];
        let mut current_tick: u64 = 0;
        let mut i = 0;
        
        while i < track_data.len() - 1 {
            // Read delta time (variable length)
            let mut delta: u64 = 0;
            let mut byte;
            loop {
                byte = track_data[i];
                i += 1;
                delta = (delta << 7) | (byte & 0x7F) as u64;
                if byte & 0x80 == 0 { break; }
            }
            
            current_tick += delta;
            
            if i >= track_data.len() { break; }
            
            let event = track_data[i];
            i += 1;
            
            // Note On (9x) or Note Off (8x)
            if (event & 0xF0) == 0x90 || (event & 0xF0) == 0x80 {
                if i + 2 <= track_data.len() {
                    let pitch = track_data[i] as i32;
                    let velocity = track_data[i + 1] as u32;
                    i += 2;
                    
                    if velocity > 0 && (event & 0xF0) == 0x90 {
                        // Note On
                        let note = NoteData {
                            position: current_tick,
                            duration: 240, // Default duration
                            pitch,
                            velocity: velocity as u32,
                            vibrato: None,
                        };
                        notes.push(note);
                    }
                }
            }
        }
        
        pos += track_len;
    }
    
    // Sort by position
    notes.sort_by_key(|n| n.position);
    
    let track_data = TrackData {
        name: "Imported Track".to_string(),
        color: None,
        notes,
    };
    
    let mut project = UstxFile::default();
    project.tracks = vec![track_data];
    project.name = Path::new(&path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported")
        .to_string();
    project.bpm = 120.0;
    
    Ok(project)
}

/// Export USTX project to MIDI
#[tauri::command]
pub fn export_midi(path: String, project: UstxFile) -> Result<(), String> {
    let file = File::create(&path)
        .map_err(|e| format!("Failed to create file: {}", e))?;
    
    let mut writer = BufWriter::new(file);
    
    // MIDI header "MThd"
    writer.write_all(b"MThd").map_err(|e| e.to_string())?;
    writer.write_all(&6u32.to_be_bytes()).map_err(|e| e.to_string())?; // Chunk size
    writer.write_all(&0u16.to_be_bytes()).map_err(|e| e.to_string())?; // Format 0
    writer.write_all(&1u16.to_be_bytes()).map_err(|e| e.to_string())?; // 1 track
    writer.write_all(&480u16.to_be_bytes()).map_err(|e| e.to_string())?; // Ticks per quarter
    
    // Build track data first
    let mut track_data: Vec<u8> = Vec::new();
    
    // Tempo event (meta 0x51)
    track_data.push(0x00);
    track_data.push(0xFF);
    track_data.push(0x51);
    track_data.push(0x03);
    let tempo = (60000000.0 / project.bpm) as u32;
    track_data.extend_from_slice(&tempo.to_be_bytes());
    
    // Notes
    if let Some(track) = project.tracks.first() {
        for note in &track.notes {
            // Delta time
            write_var_len(&mut track_data, note.position as u32);
            // Note On
            track_data.push(0x90);
            track_data.push(note.pitch as u8);
            track_data.push(note.velocity as u8);
            
            // Delta time for Note Off
            write_var_len(&mut track_data, note.duration as u32);
            // Note Off
            track_data.push(0x80);
            track_data.push(note.pitch as u8);
            track_data.push(0);
        }
    }
    
    // End of track
    track_data.push(0x00);
    track_data.push(0xFF);
    track_data.push(0x2F);
    track_data.push(0x00);
    
    // Track chunk "MTrk"
    writer.write_all(b"MTrk").map_err(|e| e.to_string())?;
    writer.write_all(&(track_data.len() as u32).to_be_bytes()).map_err(|e| e.to_string())?;
    writer.write_all(&track_data).map_err(|e| e.to_string())?;
    
    writer.flush().map_err(|e| e.to_string())?;
    
    Ok(())
}

fn write_var_len(data: &mut Vec<u8>, value: u32) {
    let mut buffer = Vec::new();
    let mut v = value;
    buffer.push((v & 0x7F) as u8);
    v >>= 7;
    while v > 0 {
        buffer.push(((v & 0x7F) | 0x80) as u8);
        v >>= 7;
    }
    buffer.reverse();
    data.extend_from_slice(&buffer);
}
