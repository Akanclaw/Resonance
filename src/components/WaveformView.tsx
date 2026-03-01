export function WaveformView() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">View:</span>
      {['Piano', 'Wave', 'Mix'].map(v => (
        <button key={v} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {v}
        </button>
      ))}
    </div>
  );
}
