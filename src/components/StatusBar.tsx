import { useProjectStore } from '../store/projectStore';

export function StatusBar() {
  const { project, currentTrackIndex, selectedNotes, isPlaying } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  
  const totalNotes = track.notes.length;
  const selectedCount = selectedNotes.length;
  const duration = track.notes.length > 0 
    ? Math.max(...track.notes.map(n => n.start + n.duration))
    : 0;
  const durationSec = Math.round(duration / 480 / project.bpm * 60);
  
  return (
    <div className="flex items-center justify-between px-4 py-1 bg-gray-800 border-t border-gray-700 text-xs text-gray-400">
      <div className="flex items-center gap-4">
        <span>
          {isPlaying ? '🔊 Playing' : '⏸ Ready'}
        </span>
        <span>Track: {track.name}</span>
        <span>Notes: {totalNotes}</span>
        {selectedCount > 0 && <span className="text-blue-400">Selected: {selectedCount}</span>}
        <span>Duration: {durationSec}s</span>
      </div>
      <div className="flex items-center gap-4">
        <span>Time: {project.beatPerBar}/{project.beatUnit}</span>
        <span>© 2026 Resonance Team</span>
      </div>
    </div>
  );
}
