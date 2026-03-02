import { useProjectStore } from '../store/projectStore';

export function Timeline() {
  const { project, currentTick, setCurrentTick } = useProjectStore();
  
  const totalTicks = 10000;
  const width = 1200;
  
  const ticksPerBeat = 480;
  const ticksPerBar = ticksPerBeat * project.beatPerBar;
  
  // Calculate bar positions
  const bars = Math.ceil(totalTicks / ticksPerBar);
  
  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    const rect = e.currentTarget.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const tick = Math.floor((x / width) * totalTicks);
    setCurrentTick(Math.max(0, tick));
  };
  
  // Playhead position as percentage
  const playheadPercent = Math.min(100, Math.max(0, (currentTick / totalTicks) * 100));
  
  // Generate time markers and bar numbers
  const markers = [];
  for (let i = 0; i <= bars; i++) {
    const tickPosition = i * ticksPerBar;
    const percent = (tickPosition / totalTicks) * 100;
    markers.push({
      bar: i + 1,
      percent,
      tick: tickPosition
    });
  }
  
  return (
    <div 
      className="h-8 bg-gray-800 border-b border-gray-700 flex items-end relative select-none"
      style={{ width, position: 'relative' }}
    >
      {/* Background grid */}
      <div className="absolute inset-0 flex">
        {markers.map((marker, i) => (
          <div
            key={i}
            className="absolute h-full border-l border-gray-600"
            style={{ left: `${marker.percent}%` }}
          />
        ))}
      </div>
      
      {/* Time/Bar markers */}
      <div className="absolute inset-0 flex items-end pointer-events-none">
        {markers.map((marker, i) => (
          <div
            key={i}
            className="absolute flex flex-col items-center"
            style={{ left: `${marker.percent}%`, transform: 'translateX(-50%)' }}
          >
            {/* Bar number */}
            <span className="text-[10px] text-blue-400 font-medium mb-0.5">
              {marker.bar}
            </span>
            {/* Tick mark */}
            <div className="w-px h-2 bg-gray-500" />
          </div>
        ))}
      </div>
      
      {/* Playhead - red vertical line */}
      <div
        className="absolute top-0 w-0.5 h-full bg-red-500 z-20 pointer-events-none transition-none"
        style={{ left: `${playheadPercent}%` }}
      >
        {/* Playhead triangle at top */}
        <div 
          className="absolute -top-0 left-1/2 transform -translate-x-1/2"
          style={{
            width: 0,
            height: 0,
            borderLeft: '6px solid transparent',
            borderRight: '6px solid transparent',
            borderTop: '8px solid #ef4444'
          }}
        />
      </div>
      
      {/* Clickable seek area */}
      <div
        className="absolute inset-0 cursor-pointer z-10"
        onClick={handleClick}
        title="Click to seek"
      />
    </div>
  );
}
