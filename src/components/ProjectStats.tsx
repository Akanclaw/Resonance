import { useProjectStore } from '../store/projectStore';

export function ProjectStats() {
  const { project, currentTrackIndex } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  
  // Calculate stats
  const totalNotes = project.tracks.reduce((sum, t) => sum + t.notes.length, 0);
  const trackNotes = track.notes.length;
  
  // Calculate total duration
  const maxTick = Math.max(0, ...project.tracks.map(t => 
    Math.max(0, ...t.notes.map(n => n.start + n.duration))
  ));
  
  // Convert ticks to time (assuming 480 ticks per beat)
  const ticksPerBeat = 480;
  const beats = maxTick / ticksPerBeat;
  const minutes = Math.floor(beats / project.beatPerBar);
  const seconds = Math.floor((beats % project.beatPerBar) / project.beatPerBar * 60);
  
  // Calculate pitch range
  const allPitches = project.tracks.flatMap(t => t.notes.map(n => n.pitch));
  const minPitch = allPitches.length ? Math.min(...allPitches) : 0;
  const maxPitch = allPitches.length ? Math.max(...allPitches) : 0;
  
  const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
  
  // Calculate average velocity
  const avgVelocity = trackNotes > 0 
    ? Math.round(track.notes.reduce((sum, n) => sum + n.velocity, 0) / trackNotes)
    : 0;
  
  // Calculate note duration stats
  const avgDuration = trackNotes > 0
    ? Math.round(track.notes.reduce((sum, n) => sum + n.duration, 0) / trackNotes)
    : 0;
  
  return (
    <div className="bg-gray-800 rounded-lg p-3 text-xs text-gray-300 space-y-2">
      <div className="font-medium text-gray-400 border-b border-gray-700 pb-1 mb-2">
        Project Statistics
      </div>
      
      <div className="grid grid-cols-2 gap-x-4 gap-y-1">
        <div>
          <span className="text-gray-500">Tracks:</span>
          <span className="ml-1 text-white">{project.tracks.length}</span>
        </div>
        <div>
          <span className="text-gray-500">Total Notes:</span>
          <span className="ml-1 text-white">{totalNotes}</span>
        </div>
        <div>
          <span className="text-gray-500">Current Track:</span>
          <span className="ml-1 text-white">{trackNotes}</span>
        </div>
        <div>
          <span className="text-gray-500">Duration:</span>
          <span className="ml-1 text-white">{minutes}:{seconds.toString().padStart(2, '0')}</span>
        </div>
        <div>
          <span className="text-gray-500">Tempo:</span>
          <span className="ml-1 text-white">{project.tempo[0]?.bpm || 120} BPM</span>
        </div>
        <div>
          <span className="text-gray-500">Time Sig:</span>
          <span className="ml-1 text-white">{project.beatPerBar}/{project.beatUnit}</span>
        </div>
        <div>
          <span className="text-gray-500">Pitch Range:</span>
          <span className="ml-1 text-white">
            {noteNames[minPitch % 12]}{Math.floor(minPitch / 12) - 1} - 
            {noteNames[maxPitch % 12]}{Math.floor(maxPitch / 12) - 1}
          </span>
        </div>
        <div>
          <span className="text-gray-500">Avg Velocity:</span>
          <span className="ml-1 text-white">{avgVelocity}</span>
        </div>
        <div>
          <span className="text-gray-500">Avg Note:</span>
          <span className="ml-1 text-white">{avgDuration} ticks</span>
        </div>
      </div>
    </div>
  );
}
