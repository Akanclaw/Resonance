export function RenderQuality() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Quality:</span>
      {['Draft', 'Normal', 'High', 'Ultra'].map(q => (
        <button key={q} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {q}
        </button>
      ))}
    </div>
  );
}
