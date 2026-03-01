export function VocalMode() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Vocal:</span>
      {['CV', 'VCV', 'VC'].map(m => (
        <button key={m} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {m}
        </button>
      ))}
    </div>
  );
}
