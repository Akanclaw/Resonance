export function ScaleLibrary() {
  const scales = [
    { name: 'Major', intervals: [0, 2, 4, 5, 7, 9, 11] },
    { name: 'Minor', intervals: [0, 2, 3, 5, 7, 8, 10] },
    { name: 'Pentatonic', intervals: [0, 2, 4, 7, 9] },
    { name: 'Blues', intervals: [0, 3, 5, 6, 7, 10] },
    { name: 'Dorian', intervals: [0, 2, 3, 5, 7, 9, 10] },
    { name: 'Mixolydian', intervals: [0, 2, 4, 5, 7, 9, 10] },
  ];
  
  const playScale = (intervals: number[]) => {
    const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
    intervals.forEach((interval, i) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.connect(gain);
      gain.connect(ctx.destination);
      const freq = 440 * Math.pow(2, interval / 12);
      osc.frequency.value = freq;
      osc.type = 'sine';
      gain.gain.setValueAtTime(0.2, ctx.currentTime + i * 0.2);
      gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + i * 0.2 + 0.3);
      osc.start(ctx.currentTime + i * 0.2);
      osc.stop(ctx.currentTime + i * 0.2 + 0.3);
    });
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Scales:</span>
      {scales.map(s => (
        <button
          key={s.name}
          onClick={() => playScale(s.intervals)}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {s.name}
        </button>
      ))}
    </div>
  );
}
