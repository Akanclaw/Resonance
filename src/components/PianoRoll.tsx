import { useRef, useEffect, useState } from 'react';
import { useProjectStore } from '../store/projectStore';

const NOTE_HEIGHT = 16;
const TICK_WIDTH = 0.5;
const MIN_PITCH = 36;
const MAX_PITCH = 84;

export function PianoRoll() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { project, currentTrackIndex, selectedNotes, addNote } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  const [scrollX] = useState(0);
  
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = 1200;
    const height = (MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT;
    
    ctx.fillStyle = '#1e1e1e';
    ctx.fillRect(0, 0, width, height);
    
    // Grid
    for (let p = MIN_PITCH; p <= MAX_PITCH; p++) {
      const y = (MAX_PITCH - p) * NOTE_HEIGHT;
      const isBlack = [1, 3, 6, 8, 10].includes(p % 12);
      ctx.fillStyle = isBlack ? '#2a2a2a' : '#252525';
      ctx.fillRect(0, y, width, NOTE_HEIGHT);
    }
    
    // Notes
    track.notes.forEach((note, i) => {
      const x = note.start * TICK_WIDTH - scrollX;
      const y = (MAX_PITCH - note.pitch) * NOTE_HEIGHT;
      const w = note.duration * TICK_WIDTH;
      
      if (x + w < 0 || x > width) return;
      
      ctx.fillStyle = selectedNotes.includes(i) ? '#60a5fa' : '#3b82f6';
      ctx.fillRect(Math.max(0, x) + 1, y + 1, Math.min(w - 2, width - x - 2), NOTE_HEIGHT - 2);
    });
  }, [track, selectedNotes, scrollX]);
  
  const handleClick = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    const x = e.clientX - rect.left + scrollX;
    const y = e.clientY - rect.top;
    const pitch = MAX_PITCH - Math.floor(y / NOTE_HEIGHT);
    const start = Math.floor(x / TICK_WIDTH / 10) * 10;
    
    if (pitch >= MIN_PITCH && pitch <= MAX_PITCH && start >= 0) {
      addNote(currentTrackIndex, { pitch, velocity: 100, start, duration: 480 });
    }
  };
  
  return (
    <div className="flex-1 overflow-auto bg-gray-800">
      <canvas
        ref={canvasRef}
        width={1200}
        height={(MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT}
        onClick={handleClick}
        className="cursor-crosshair"
      />
    </div>
  );
}
