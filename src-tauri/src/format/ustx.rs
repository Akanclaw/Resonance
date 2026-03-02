use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// USTX File - OpenUtau's main file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UstxFile {
    /// File version
    #[serde(rename = "Version")]
    pub version: String,
    /// Project name
    #[serde(rename = "Name")]
    pub name: String,
    /// BPM (beats per minute)
    #[serde(rename = "BPM")]
    pub bpm: f64,
    /// Time signature numerator
    #[serde(rename = "BeatPerBar")]
    pub beat_per_bar: u32,
    /// Time signature denominator
    #[serde(rename = "BeatUnit")]
    pub beat_unit: u32,
    /// Tempo map
    #[serde(rename = "Tempo")]
    pub tempo: Vec<Tempo>,
    /// Tracks
    #[serde(rename = "Tracks")]
    pub tracks: Vec<TrackData>,
    /// Project settings
    #[serde(rename = "Project")]
    pub project: ProjectSettings,
}

/// Tempo point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tempo {
    #[serde(rename = "Position")]
    pub position: u64,
    #[serde(rename = "BPM")]
    pub bpm: f64,
}

/// Track data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackData {
    /// Track name
    #[serde(rename = "Name")]
    pub name: String,
    /// Track color
    #[serde(rename = "Color")]
    pub color: Option<u32>,
    /// Notes in the track
    #[serde(rename = "Notes")]
    pub notes: Vec<NoteData>,
}

/// Note data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteData {
    /// Note position in ticks
    #[serde(rename = "Position")]
    pub position: u64,
    /// Note duration in ticks
    #[serde(rename = "Duration")]
    pub duration: u64,
    /// Note pitch (MIDI number)
    #[serde(rename = "Pitch")]
    pub pitch: i32,
    /// Note velocity
    #[serde(rename = "Velocity")]
    pub velocity: u32,
    /// Vibrato settings
    #[serde(rename = "Vibrato")]
    pub vibrato: Option<VibratoData>,
}

/// Vibrato data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibratoData {
    /// Start time relative to note start
    #[serde(rename = "Start")]
    pub start: u64,
    /// Vibrato duration
    #[serde(rename = "Length")]
    pub length: u64,
    /// Vibrato period in points
    #[serde(rename = "Period")]
    pub period: u64,
    /// Vibrato depth
    #[serde(rename = "Depth")]
    pub depth: i32,
    /// Fade in length
    #[serde(rename = "Fade")]
    pub fade: u64,
}

/// Project settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// vocals directory path
    #[serde(rename = "VoiceDir")]
    pub voice_dir: Option<String>,
    /// Singer name
    #[serde(rename = "Singer")]
    pub singer: Option<String>,
    /// Expression presets
    #[serde(rename = "Expressions")]
    pub expressions: HashMap<String, ExpressionDef>,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            voice_dir: None,
            singer: None,
            expressions: HashMap::new(),
        }
    }
}

/// Expression definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressionDef {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Abbreviation")]
    pub abbreviation: Option<String>,
    #[serde(rename = "Type")]
    pub expr_type: u32,
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde(rename = "DefaultValue")]
    pub default_value: f64,
}

impl Default for UstxFile {
    fn default() -> Self {
        Self {
            version: "OpenUtau".to_string(),
            name: "Untitled".to_string(),
            bpm: 120.0,
            beat_per_bar: 4,
            beat_unit: 4,
            tempo: vec![Tempo { position: 0, bpm: 120.0 }],
            tracks: vec![TrackData::default()],
            project: ProjectSettings::default(),
        }
    }
}

impl UstxFile {
    /// Create a new USTX file with default settings
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Create a new USTX with full configuration
    pub fn with_config(
        name: impl Into<String>,
        bpm: f64,
        beat_per_bar: u32,
        beat_unit: u32,
    ) -> Self {
        Self {
            name: name.into(),
            bpm,
            beat_per_bar,
            beat_unit,
            tempo: vec![Tempo { position: 0, bpm }],
            ..Default::default()
        }
    }

    /// Get project name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set project name
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    /// Get BPM
    pub fn bpm(&self) -> f64 {
        self.bpm
    }

    /// Set BPM
    pub fn set_bpm(&mut self, bpm: f64) {
        self.bpm = bpm;
        if let Some(tempo) = self.tempo.first_mut() {
            tempo.bpm = bpm;
        }
    }

    /// Get beat per bar
    pub fn beat_per_bar(&self) -> u32 {
        self.beat_per_bar
    }

    /// Set beat per bar
    pub fn set_beat_per_bar(&mut self, beat: u32) {
        self.beat_per_bar = beat;
    }

    /// Get beat unit
    pub fn beat_unit(&self) -> u32 {
        self.beat_unit
    }

    /// Set beat unit
    pub fn set_beat_unit(&mut self, unit: u32) {
        self.beat_unit = unit;
    }

    /// Get number of tracks
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Get track at index
    pub fn get_track(&self, index: usize) -> Option<&TrackData> {
        self.tracks.get(index)
    }

    /// Get track at index (mutable)
    pub fn get_track_mut(&mut self, index: usize) -> Option<&mut TrackData> {
        self.tracks.get_mut(index)
    }

    /// Add a new track
    pub fn add_track(&mut self, track: TrackData) -> usize {
        let idx = self.tracks.len();
        self.tracks.push(track);
        idx
    }

    /// Add a new track with name
    pub fn add_track_with_name(&mut self, name: impl Into<String>) -> usize {
        self.add_track(TrackData {
            name: name.into(),
            ..Default::default()
        })
    }

    /// Remove a track at index
    pub fn remove_track(&mut self, index: usize) -> Option<TrackData> {
        if index < self.tracks.len() && self.tracks.len() > 1 {
            Some(self.tracks.remove(index))
        } else {
            None
        }
    }

    /// Get total note count
    pub fn total_notes(&self) -> usize {
        self.tracks.iter().map(|t| t.notes.len()).sum()
    }

    /// Get project duration in ticks
    pub fn duration(&self) -> u64 {
        self.tracks.iter()
            .flat_map(|t| t.notes.iter())
            .map(|n| n.position + n.duration)
            .max()
            .unwrap_or(0)
    }

    /// Get singer name
    pub fn singer(&self) -> Option<&str> {
        self.project.singer.as_deref()
    }

    /// Set singer name
    pub fn set_singer(&mut self, singer: impl Into<String>) {
        self.project.singer = Some(singer.into());
    }

    /// Get voice directory
    pub fn voice_dir(&self) -> Option<&str> {
        self.project.voice_dir.as_deref()
    }

    /// Set voice directory
    pub fn set_voice_dir(&mut self, dir: impl Into<String>) {
        self.project.voice_dir = Some(dir.into());
    }

    /// Validate the project
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.name.is_empty() {
            errors.push("Project name is empty".to_string());
        }
        
        if self.bpm <= 0.0 || self.bpm > 1000.0 {
            errors.push(format!("Invalid BPM: {}", self.bpm));
        }
        
        if self.tracks.is_empty() {
            errors.push("No tracks in project".to_string());
        }
        
        for (i, track) in self.tracks.iter().enumerate() {
            for (j, note) in track.notes.iter().enumerate() {
                if note.pitch < 0 || note.pitch > 127 {
                    errors.push(format!("Track {} note {} has invalid pitch: {}", i, j, note.pitch));
                }
                if note.duration == 0 {
                    errors.push(format!("Track {} note {} has zero duration", i, j));
                }
                if note.velocity == 0 {
                    errors.push(format!("Track {} note {} has zero velocity", i, j));
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl TrackData {
    /// Create a new track
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Create with color
    pub fn with_color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Get note at index
    pub fn get_note(&self, index: usize) -> Option<&NoteData> {
        self.notes.get(index)
    }

    /// Add a note
    pub fn add_note(&mut self, note: NoteData) -> usize {
        let idx = self.notes.len();
        self.notes.push(note);
        self.notes.sort_by_key(|n| n.position);
        idx
    }

    /// Remove a note at index
    pub fn remove_note(&mut self, index: usize) -> Option<NoteData> {
        if index < self.notes.len() {
            Some(self.notes.remove(index))
        } else {
            None
        }
    }

    /// Get note count
    pub fn note_count(&self) -> usize {
        self.notes.len()
    }

    /// Get track duration
    pub fn duration(&self) -> u64 {
        self.notes.iter()
            .map(|n| n.position + n.duration)
            .max()
            .unwrap_or(0)
    }
}

impl NoteData {
    /// Create a new note
    pub fn new(position: u64, duration: u64, pitch: i32, velocity: u32) -> Self {
        Self {
            position,
            duration,
            pitch: pitch.clamp(0, 127),
            velocity: velocity.min(127),
            vibrato: None,
        }
    }

    /// Get note end position
    pub fn end(&self) -> u64 {
        self.position + self.duration
    }

    /// Get pitch name (e.g., "C4")
    pub fn pitch_name(&self) -> String {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let octave = (self.pitch / 12) - 1;
        let note = note_names[(self.pitch % 12) as usize];
        format!("{}{}", note, octave)
    }

    /// Set vibrato
    pub fn set_vibrato(&mut self, vibrato: VibratoData) {
        self.vibrato = Some(vibrato);
    }

    /// Clear vibrato
    pub fn clear_vibrato(&mut self) {
        self.vibrato = None;
    }
}

impl VibratoData {
    /// Create new vibrato
    pub fn new(start: u64, length: u64, period: u64, depth: i32, fade: u64) -> Self {
        Self {
            start,
            length,
            period,
            depth,
            fade,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ustx_default() {
        let file = UstxFile::default();
        assert_eq!(file.bpm, 120.0);
        assert_eq!(file.tracks.len(), 1);
    }

    #[test]
    fn test_ustx_serialization() {
        let file = UstxFile::new("Test Project");
        let json = serde_json::to_string(&file).unwrap();
        assert!(json.contains("Test Project"));
    }
}
