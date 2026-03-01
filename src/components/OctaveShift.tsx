import { useProjectStore } from '../store/projectStore';

export function OctaveShift() {
  const handleShift = (semitones: number) => {
    const { project, currentTrackIndex, setProject } = useProjectStore.getState();
    const tracks = [...project.tracks];
    const track = { ...tracks[currentTrackIndex] };
    track.notes = track.notes.map(note => ({
      ...note,
      pitch: Math.max(0, Math.min(127, note.pitch + semitones))
    }));
    tracks[currentTrackIndex] = track;
    setProject({ ...project, tracks });
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Octave:</span>
      <button onClick={() => handleShift(-12)} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">-1</button>
      <button onClick={() => handleShift(-1)} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">-</button>
      <button onClick={() => handleShift(1)} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">+</button>
      <button onClick={() => handleShift(12)} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm">+1</button>
    </div>
  );
}
