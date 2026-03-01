export function ZoomControls() {
  const handleZoom = (delta: number) => {
    console.log('Zoom:', delta);
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Zoom:</span>
      <button 
        onClick={() => handleZoom(-1)}
        className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm"
      >
        -
      </button>
      <span className="text-gray-400 text-xs w-12 text-center">100%</span>
      <button 
        onClick={() => handleZoom(1)}
        className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm"
      >
        +
      </button>
    </div>
  );
}
