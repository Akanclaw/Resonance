export function PianoRange() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Range:</span>
      {['C3-C5', 'C4-C6', 'C2-C4', 'Full'].map(r => (
        <button key={r} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {r}
        </button>
      ))}
    </div>
  );
}
