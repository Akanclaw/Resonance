export function MIDILearn() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">MIDI:</span>
      <button className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
        Learn
      </button>
      <span className="text-gray-500 text-xs">No device</span>
    </div>
  );
}
