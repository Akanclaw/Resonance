export function LoopRegion() {
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Loop:</span>
      <button className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
        A
      </button>
      <span className="text-gray-500 text-xs">-</span>
      <button className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
        B
      </button>
      <button className="px-2 py-1 bg-blue-700 hover:bg-blue-600 rounded text-white text-xs">
        🔁
      </button>
    </div>
  );
}
