import { useProjectStore } from '../store/projectStore';

export function NoteLengthSelector() {
  const lengthOptions = [
    { name: '1/4', value: 120 },
    { name: '1/2', value: 240 },
    { name: '1/1', value: 480 },
    { name: '2/1', value: 960 },
    { name: '3/4', value: 360 },
    { name: '3/8', value: 180 },
  ];
  
  const handleSetLength = (value: number) => {
    const { project, currentTrackIndex, setProject } = useProjectStore.getState();
    const tracks = [...project.tracks];
    const track = { ...tracks[currentTrackIndex] };
    // Set selected note duration or add new note with this length
    const selected = useProjectStore.getState().selectedNotes;
    if (selected.length > 0) {
      const notes = [...track.notes];
      notes[selected[0]] = { ...notes[selected[0]], duration: value };
      track.notes = notes;
    }
    tracks[currentTrackIndex] = track;
    setProject({ ...project, tracks });
  };
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Length:</span>
      <select
        onChange={(e) => handleSetLength(parseInt(e.target.value))}
        className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
        defaultValue="0"
      >
        <option value="0" disabled>Select...</option>
        {lengthOptions.map(opt => (
          <option key={opt.value} value={opt.value}>{opt.name}</option>
        ))}
      </select>
    </div>
  );
}
