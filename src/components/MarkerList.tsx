export function MarkerList() {
  const markers = [
    { time: 0, name: 'Intro' },
    { time: 1920, name: 'Verse 1' },
    { time: 3840, name: 'Chorus' },
  ];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Markers:</span>
      {markers.map((m, i) => (
        <button
          key={i}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {m.name}
        </button>
      ))}
      <button className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs">
        +
      </button>
    </div>
  );
}
