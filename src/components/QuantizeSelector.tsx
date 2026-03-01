import { useProjectStore } from '../store/projectStore';

export function QuantizeSelector() {
  const quantizeOptions = [
    { name: '1/1', value: 480 },
    { name: '1/2', value: 240 },
    { name: '1/4', value: 120 },
    { name: '1/8', value: 60 },
    { name: '1/16', value: 30 },
    { name: '1/32', value: 15 },
  ];
  
  const handleQuantize = (value: number) => {
    const { project, currentTrackIndex, setProject } = useProjectStore.getState();
    const tracks = [...project.tracks];
    const track = { ...tracks[currentTrackIndex] };
    track.notes = track.notes.map(note => ({
      ...note,
      start: Math.round(note.start / value) * value,
      duration: Math.round(note.duration / value) * value
    }));
    tracks[currentTrackIndex] = track;
    setProject({ ...project, tracks });
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Quantize:</span>
      <select
        onChange={(e) => handleQuantize(parseInt(e.target.value))}
        className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
        defaultValue="0"
      >
        <option value="0" disabled>Select...</option>
        {quantizeOptions.map(opt => (
          <option key={opt.value} value={opt.value}>{opt.name}</option>
        ))}
      </select>
    </div>
  );
}
