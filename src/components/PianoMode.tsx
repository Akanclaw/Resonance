export function PianoMode() {
  const modes = ['Melody', 'Drum', 'Bass', 'Chord'];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Mode:</span>
      {modes.map(mode => (
        <button
          key={mode}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {mode}
        </button>
      ))}
    </div>
  );
}
