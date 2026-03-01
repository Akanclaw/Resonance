import { useProjectStore } from '../store/projectStore';

export function Toolbar() {
  const { isPlaying, setPlaying, project } = useProjectStore();
  
  return (
    <div className="flex items-center gap-2 px-4 py-2 bg-gray-800 border-b border-gray-700">
      <button
        onClick={() => setPlaying(!isPlaying)}
        className="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 rounded text-white text-sm"
      >
        {isPlaying ? '⏹ Stop' : '▶ Play'}
      </button>
      
      <div className="flex items-center gap-2 ml-4">
        <span className="text-gray-400 text-sm">BPM:</span>
        <input
          type="number"
          value={project.bpm}
          className="w-16 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
          readOnly
        />
      </div>
      
      <div className="flex-1" />
      <span className="text-gray-400 text-sm">{project.name}</span>
    </div>
  );
}
