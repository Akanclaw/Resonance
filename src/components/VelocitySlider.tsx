import { useProjectStore } from '../store/projectStore';

export function VelocitySlider() {
  const { project, currentTrackIndex, setProject } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  
  const avgVelocity = track.notes.length > 0
    ? Math.round(track.notes.reduce((sum, n) => sum + n.velocity, 0) / track.notes.length)
    : 100;
  
  const handleSetVelocity = (velocity: number) => {
    const tracks = [...project.tracks];
    const newTrack = { ...tracks[currentTrackIndex] };
    newTrack.notes = newTrack.notes.map(note => ({ ...note, velocity }));
    tracks[currentTrackIndex] = newTrack;
    setProject({ ...project, tracks });
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Vel:</span>
      <input
        type="range"
        min={0}
        max={127}
        value={avgVelocity}
        onChange={(e) => handleSetVelocity(parseInt(e.target.value))}
        className="w-20"
      />
      <span className="text-gray-400 text-xs w-6">{avgVelocity}</span>
    </div>
  );
}
