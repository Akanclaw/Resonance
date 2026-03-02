//! Resonance - MIDI Processing
//! 
//! This module provides MIDI parsing and processing capabilities.

pub mod note;
pub mod track;
pub mod event;
pub mod note_track;

pub use note::Note;
pub use track::Track;
pub use event::{MidiEvent, MidiEventType};
pub use note_track::{NoteTrack, NoteClipboard, NoteClipboardItem};
