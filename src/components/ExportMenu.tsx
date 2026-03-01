export function ExportMenu() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Export:</span>
      {['WAV', 'MP3', 'OGG', 'FLAC'].map(f => (
        <button key={f} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {f}
        </button>
      ))}
    </div>
  );
}
