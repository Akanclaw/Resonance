import { useProjectStore } from '../store/projectStore';

interface Note {
  pitch: number;
  velocity: number;
  start: number;
  duration: number;
}

export function NoteProperties() {
  const { project, currentTrackIndex, selectedNotes, updateNote, deleteNote } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  const selectedNote = selectedNotes.length === 1 ? track.notes[selectedNotes[0]] : null;
  
  if (!selectedNote) {
    return (
      <div className="w-64 bg-gray-900 border-l border-gray-700 p-4">
        <h3 className="text-gray-300 font-medium mb-4">Properties</h3>
        <p className="text-gray-500 text-sm">Select a note to edit</p>
      </div>
    );
  }
  
  const noteIndex = selectedNotes[0];
  
  const handleChange = (field: keyof Note, value: number) => {
    updateNote(currentTrackIndex, noteIndex, { [field]: value });
  };
  
  return (
    <div className="w-64 bg-gray-900 border-l border-gray-700 p-4">
      <h3 className="text-gray-300 font-medium mb-4">Note Properties</h3>
      
      <div className="space-y-3">
        <div>
          <label className="text-gray-400 text-xs">Pitch (MIDI)</label>
          <input
            type="number"
            value={selectedNote.pitch}
            onChange={(e) => handleChange('pitch', parseInt(e.target.value) || 60)}
            className="w-full mt-1 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
            min={0}
            max={127}
          />
        </div>
        
        <div>
          <label className="text-gray-400 text-xs">Velocity</label>
          <input
            type="number"
            value={selectedNote.velocity}
            onChange={(e) => handleChange('velocity', parseInt(e.target.value) || 100)}
            className="w-full mt-1 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
            min={0}
            max={127}
          />
        </div>
        
        <div>
          <label className="text-gray-400 text-xs">Start (ticks)</label>
          <input
            type="number"
            value={selectedNote.start}
            onChange={(e) => handleChange('start', parseInt(e.target.value) || 0)}
            className="w-full mt-1 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
            min={0}
          />
        </div>
        
        <div>
          <label className="text-gray-400 text-xs">Duration (ticks)</label>
          <input
            type="number"
            value={selectedNote.duration}
            onChange={(e) => handleChange('duration', parseInt(e.target.value) || 480)}
            className="w-full mt-1 px-2 py-1 bg-gray-800 border border-gray-600 rounded text-white text-sm"
            min={1}
          />
        </div>
        
        <button
          onClick={() => deleteNote(currentTrackIndex, noteIndex)}
          className="w-full mt-4 px-3 py-2 bg-red-600 hover:bg-red-700 rounded text-white text-sm"
        >
          Delete Note
        </button>
      </div>
    </div>
  );
}
