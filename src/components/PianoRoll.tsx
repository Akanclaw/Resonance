import { useRef, useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';

const NOTE_HEIGHT = 16;
const MIN_PITCH = 36;
const MAX_PITCH = 84;
const HEADER_HEIGHT = 24;
const PIANO_KEY_WIDTH = 40;

interface DragState {
  type: 'move' | 'resize-left' | 'resize-right' | null;
  noteIndex: number;
  startX: number;
  startTick: number;
  originalStart: number;
  originalDuration: number;
}

export function PianoRoll() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { 
    project, 
    currentTrackIndex, 
    selectedNotes, 
    selectNote, 
    addNote, 
    clearSelection,
    updateNote,
    currentTick,
    isPlaying 
  } = useProjectStore();
  const track = project.tracks[currentTrackIndex];
  const [scrollX, setScrollX] = useState(0);
  const [scrollY, setScrollY] = useState(0);
  const [tickWidth, setTickWidth] = useState(0.5);
  const [dragState, setDragState] = useState<DragState>({
    type: null,
    noteIndex: -1,
    startX: 0,
    startTick: 0,
    originalStart: 0,
    originalDuration: 0
  });
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 });

  // Calculate total width and height
  const totalTicks = Math.max(4000, ...track.notes.map(n => n.start + n.duration));
  const totalHeight = (MAX_PITCH - MIN_PITCH + 1) * NOTE_HEIGHT + HEADER_HEIGHT;

  // Load project info from backend
  useEffect(() => {
    invoke('get_project_info')
      .then((info) => {
        console.log('Project info:', info);
      })
      .catch((err) => {
        console.log('Backend not ready or error:', err);
      });
  }, []);

  const render = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    
    const width = canvas.width;
    const height = canvas.height;
    
    // Clear
    ctx.fillStyle = '#1e1e1e';
    ctx.fillRect(0, 0, width, height);
    
    // Pitch lines and piano keys
    const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const blackNotes = [1, 3, 6, 8, 10];
    
    // Draw piano key area
    ctx.fillStyle = '#252525';
    ctx.fillRect(0, HEADER_HEIGHT, PIANO_KEY_WIDTH, height - HEADER_HEIGHT);
    
    for (let p = MIN_PITCH; p <= MAX_PITCH; p++) {
      const y = HEADER_HEIGHT + (MAX_PITCH - p) * NOTE_HEIGHT;
      const noteInPitch = p % 12;
      const isBlack = blackNotes.includes(noteInPitch);
      
      // Grid row background
      ctx.fillStyle = isBlack ? '#2a2a2a' : '#252525';
      ctx.fillRect(PIANO_KEY_WIDTH, y, width - PIANO_KEY_WIDTH, NOTE_HEIGHT);
      
      // Piano key label
      ctx.fillStyle = isBlack ? '#3a3a3a' : '#2a2a2a';
      ctx.fillRect(0, y, PIANO_KEY_WIDTH, NOTE_HEIGHT);
      
      if (noteInPitch === 0 || noteInPitch === 5) {
        ctx.fillStyle = isBlack ? '#888' : '#aaa';
        ctx.font = '10px monospace';
        ctx.fillText(`${noteNames[noteInPitch]}${Math.floor(p / 12) - 1}`, 4, y + NOTE_HEIGHT - 4);
      }
    }
    
    // Beat lines
    const ticksPerBeat = 480;
    const ticksPerBar = ticksPerBeat * project.beatPerBar;
    
    for (let t = 0; t <= totalTicks; t += ticksPerBeat) {
      const x = PIANO_KEY_WIDTH + t * tickWidth - scrollX;
      if (x < PIANO_KEY_WIDTH || x > width) continue;
      
      ctx.strokeStyle = t % ticksPerBar === 0 ? '#444' : '#333';
      ctx.lineWidth = t % ticksPerBar === 0 ? 1 : 0.5;
      ctx.beginPath();
      ctx.moveTo(x, HEADER_HEIGHT);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
    
    // Playhead
    if (isPlaying || currentTick > 0) {
      const playheadX = PIANO_KEY_WIDTH + currentTick * tickWidth - scrollX;
      if (playheadX >= PIANO_KEY_WIDTH && playheadX <= width) {
        ctx.strokeStyle = '#ef4444';
        ctx.lineWidth = 2;
        ctx.beginPath();
        ctx.moveTo(playheadX, 0);
        ctx.lineTo(playheadX, height);
        ctx.stroke();
        
        // Playhead triangle
        ctx.fillStyle = '#ef4444';
        ctx.beginPath();
        ctx.moveTo(playheadX - 6, 0);
        ctx.lineTo(playheadX + 6, 0);
        ctx.lineTo(playheadX, 10);
        ctx.closePath();
        ctx.fill();
      }
    }
    
    // Notes
    track.notes.forEach((note, i) => {
      const x = PIANO_KEY_WIDTH + note.start * tickWidth - scrollX;
      const y = HEADER_HEIGHT + (MAX_PITCH - note.pitch) * NOTE_HEIGHT;
      const w = note.duration * tickWidth;
      
      if (x + w < PIANO_KEY_WIDTH || x > width) return;
      
      const isSelected = selectedNotes.includes(i);
      const color = track.color || '#3b82f6';
      
      // Note body
      ctx.fillStyle = isSelected ? '#60a5fa' : color;
      ctx.beginPath();
      const noteX = Math.max(PIANO_KEY_WIDTH + 1, x);
      const noteW = Math.min(w - 2, width - noteX - 2);
      ctx.roundRect(noteX + 1, y + 1, noteW - 2, NOTE_HEIGHT - 2, 2);
      ctx.fill();
      
      // Selection border
      if (isSelected) {
        ctx.strokeStyle = '#fff';
        ctx.lineWidth = 2;
        ctx.stroke();
        
        // Resize handles
        ctx.fillStyle = '#fff';
        // Left handle
        if (x >= PIANO_KEY_WIDTH) {
          ctx.fillRect(x + 2, y + 2, 4, NOTE_HEIGHT - 4);
        }
        // Right handle
        if (x + w <= width) {
          ctx.fillRect(x + w - 6, y + 2, 4, NOTE_HEIGHT - 4);
        }
      }
    });
    
    // Current mouse position highlight
    const mouseY = mousePos.y;
    if (mouseY >= HEADER_HEIGHT) {
      const hoverPitch = MAX_PITCH - Math.floor((mouseY - HEADER_HEIGHT) / NOTE_HEIGHT);
      if (hoverPitch >= MIN_PITCH && hoverPitch <= MAX_PITCH) {
        const rowY = HEADER_HEIGHT + (MAX_PITCH - hoverPitch) * NOTE_HEIGHT;
        ctx.fillStyle = 'rgba(255, 255, 255, 0.05)';
        ctx.fillRect(PIANO_KEY_WIDTH, rowY, width - PIANO_KEY_WIDTH, NOTE_HEIGHT);
      }
    }
  }, [track, selectedNotes, scrollX, scrollY, tickWidth, project.beatPerBar, currentTick, isPlaying, mousePos, totalTicks]);
  
  useEffect(() => {
    render();
  }, [render]);

  // Convert mouse position to grid position
  const getGridPosition = useCallback((clientX: number, clientY: number) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return null;
    
    const x = clientX - rect.left;
    const y = clientY - rect.top;
    
    if (y < HEADER_HEIGHT) return null;
    
    const pitch = MAX_PITCH - Math.floor((y - HEADER_HEIGHT) / NOTE_HEIGHT);
    const tick = Math.round((x - PIANO_KEY_WIDTH + scrollX) / tickWidth);
    
    if (pitch < MIN_PITCH || pitch > MAX_PITCH || tick < 0) return null;
    
    return { pitch, tick };
  }, [scrollX, tickWidth]);

  // Find note at position with drag type
  const getNoteAtPosition = useCallback((x: number, y: number): { noteIndex: number; type: 'move' | 'resize-left' | 'resize-right' } | null => {
    const gridPos = getGridPosition(x, y);
    if (!gridPos) return null;
    
    for (let i = track.notes.length - 1; i >= 0; i--) {
      const note = track.notes[i];
      const noteX = PIANO_KEY_WIDTH + note.start * tickWidth - scrollX;
      const noteW = note.duration * tickWidth;
      
      // Check if in note area
      if (gridPos.tick >= note.start && gridPos.tick < note.start + note.duration &&
          gridPos.pitch === note.pitch) {
        // Determine drag type based on position within note
        const relX = x - noteX;
        if (relX < 8) {
          return { noteIndex: i, type: 'resize-left' };
        } else if (relX > noteW - 8) {
          return { noteIndex: i, type: 'resize-right' };
        }
        return { noteIndex: i, type: 'move' };
      }
    }
    return null;
  }, [track.notes, scrollX, tickWidth, getGridPosition]);

  const handleMouseDown = (e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const hitResult = getNoteAtPosition(x, y);
    
    if (hitResult) {
      selectNote(hitResult.noteIndex);
      const note = track.notes[hitResult.noteIndex];
      setDragState({
        type: hitResult.type,
        noteIndex: hitResult.noteIndex,
        startX: e.clientX,
        startTick: Math.round((x - PIANO_KEY_WIDTH + scrollX) / tickWidth),
        originalStart: note.start,
        originalDuration: note.duration
      });
    } else if (x > PIANO_KEY_WIDTH) {
      clearSelection();
    }
  };

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    setMousePos({ x, y });
    
    // Update cursor
    const hitResult = getNoteAtPosition(x, y);
    const canvas = canvasRef.current;
    if (canvas) {
      if (hitResult) {
        if (hitResult.type === 'resize-left' || hitResult.type === 'resize-right') {
          canvas.style.cursor = 'ew-resize';
        } else {
          canvas.style.cursor = 'move';
        }
      } else if (x > PIANO_KEY_WIDTH && y > HEADER_HEIGHT) {
        canvas.style.cursor = 'crosshair';
      } else {
        canvas.style.cursor = 'default';
      }
    }
    
    // Handle dragging
    if (dragState.type && dragState.noteIndex >= 0) {
      const deltaX = e.clientX - dragState.startX;
      const deltaTick = Math.round(deltaX / tickWidth);
      
      if (dragState.type === 'move') {
        const newStart = Math.max(0, dragState.originalStart + deltaTick);
        updateNote(currentTrackIndex, dragState.noteIndex, { start: newStart });
      } else if (dragState.type === 'resize-left') {
        const newStart = Math.max(0, dragState.originalStart + deltaTick);
        const newDuration = dragState.originalDuration - deltaTick;
        if (newDuration > 10) {
          updateNote(currentTrackIndex, dragState.noteIndex, { 
            start: newStart, 
            duration: newDuration 
          });
        }
      } else if (dragState.type === 'resize-right') {
        const newDuration = Math.max(10, dragState.originalDuration + deltaTick);
        updateNote(currentTrackIndex, dragState.noteIndex, { duration: newDuration });
      }
    }
  }, [dragState, tickWidth, scrollX, getNoteAtPosition, updateNote, currentTrackIndex]);

  const handleMouseUp = useCallback(() => {
    if (dragState.type && dragState.noteIndex >= 0) {
      // Sync to backend after drag
      const note = track.notes[dragState.noteIndex];
      invoke('create_note', {
        pitch: note.pitch,
        velocity: note.velocity,
        start: note.start,
        duration: note.duration
      }).catch(() => {});
    }
    setDragState({ type: null, noteIndex: -1, startX: 0, startTick: 0, originalStart: 0, originalDuration: 0 });
  }, [dragState, track.notes]);

  const handleDoubleClick = (e: React.MouseEvent) => {
    const gridPos = getGridPosition(e.clientX, e.clientY);
    if (!gridPos) return;
    
    // Snap to grid (16th notes = 120 ticks)
    const snapTicks = 120;
    const start = Math.floor(gridPos.tick / snapTicks) * snapTicks;
    
    addNote(currentTrackIndex, { 
      pitch: gridPos.pitch, 
      velocity: 100, 
      start, 
      duration: 480 
    });
    
    // Sync to backend
    invoke('create_note', {
      pitch: gridPos.pitch,
      velocity: 100,
      start,
      duration: 480
    }).catch(() => {});
  };
  
  const handleWheel = (e: React.WheelEvent) => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;
    
    if (e.ctrlKey) {
      e.preventDefault();
      const delta = e.deltaY > 0 ? -0.05 : 0.05;
      setTickWidth(prev => Math.max(0.1, Math.min(4, prev + delta)));
    } else if (e.shiftKey) {
      setScrollX(prev => Math.max(0, prev - e.deltaY));
    } else {
      setScrollY(prev => Math.max(0, prev - e.deltaY));
    }
  };

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (selectedNotes.length > 0 && (e.key === 'Delete' || e.key === 'Backspace')) {
        const { deleteNote } = useProjectStore.getState();
        selectedNotes.forEach(noteIndex => {
          deleteNote(currentTrackIndex, noteIndex);
        });
      }
    };
    
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [selectedNotes, currentTrackIndex]);

  return (
    <div className="flex-1 overflow-hidden bg-gray-800 flex flex-col">
      <div className="h-6 bg-gray-700 border-b border-gray-600 flex items-center justify-between px-2">
        <span className="text-xs text-gray-400">Piano Roll - {track.name}</span>
        <div className="flex items-center gap-3 text-xs text-gray-500">
          <span>Zoom: {Math.round(tickWidth * 200)}%</span>
          <span>|</span>
          <span>Scroll: Wheel | Shift+Wheel: H</span>
          <span>|</span>
          <span>Del: Delete Note</span>
        </div>
      </div>
      <div className="overflow-auto flex-1 relative">
        <canvas
          ref={canvasRef}
          width={1400}
          height={totalHeight}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          onMouseLeave={handleMouseUp}
          onDoubleClick={handleDoubleClick}
          onWheel={handleWheel}
          className="cursor-crosshair"
        />
      </div>
    </div>
  );
}
