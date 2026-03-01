export function AutomationLane() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Automation:</span>
      {['Volume', 'Pan', 'Pitch', 'Mod'].map(a => (
        <button key={a} className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
          {a}
        </button>
      ))}
    </div>
  );
}
