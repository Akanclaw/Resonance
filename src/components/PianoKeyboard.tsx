export function PianoKeyboard() {
  const keys = [
    { note: 'C', black: false },
    { note: 'C#', black: true },
    { note: 'D', black: false },
    { note: 'D#', black: true },
    { note: 'E', black: false },
    { note: 'F', black: false },
    { note: 'F#', black: true },
    { note: 'G', black: false },
    { note: 'G#', black: true },
    { note: 'A', black: false },
    { note: 'A#', black: true },
    { note: 'B', black: false },
  ];
  
  const playNote = (note: string, octave: number) => {
    const ctx = new (window.AudioContext || (window as any).webkitAudioContext)();
    const osc = ctx.createOscillator();
    const gain = ctx.createGain();
    osc.connect(gain);
    gain.connect(ctx.destination);
    const freq = 440 * Math.pow(2, (note.indexOf(note) + (octave - 4) * 12 - 9) / 12);
    osc.frequency.value = freq;
    gain.gain.setValueAtTime(0.3, ctx.currentTime);
    gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.5);
    osc.start(ctx.currentTime);
    osc.stop(ctx.currentTime + 0.5);
  };
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Piano:</span>
      {[3, 4, 5].map(octave => (
        <div key={octave} className="flex">
          {keys.map(k => (
            <button
              key={`${k.note}${octave}`}
              onClick={() => playNote(k.note, octave)}
              className={`w-6 h-8 ${k.black ? 'bg-gray-900 -mx-1 z-10' : 'bg-white'} rounded-b-sm border border-gray-600`}
            />
          ))}
        </div>
      ))}
    </div>
  );
}
