import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { useProjectStore } from '../store/projectStore';
import { Presets } from './Presets';

export function Toolbar() {
  const { isPlaying, setPlaying, project, undo, redo, canUndo, canRedo } = useProjectStore();
  const [loading, setLoading] = useState(false);
  
  const handlePlay = async () => {
    try {
      setLoading(true);
      if (isPlaying) {
        await invoke('stop_audio');
      } else {
        await invoke('play_audio');
      }
      setPlaying(!isPlaying);
    } catch (e) {
      console.error('Playback error:', e);
    } finally {
      setLoading(false);
    }
  };
  
  const handleStop = async () => {
    try {
      await invoke('stop_audio');
      setPlaying(false);
    } catch (e) {
      console.error('Stop error:', e);
    }
  };
  
  const handleExportMidi = async () => {
    try {
      const path = await save({
        filters: [{ name: 'MIDI', extensions: ['mid', 'midi'] }],
        defaultPath: `${project.name}.mid`
      });
      
      if (path) {
        // Convert project to USTX format for export
        const ustxProject = {
          name: project.name,
          bpm: project.bpm,
          beatPerBar: project.beatPerBar,
          beatUnit: project.beatUnit,
          tempo: project.tempo,
          tracks: project.tracks,
          project: {
            voiceDir: null,
            singer: null,
            expressions: {}
          },
          Version: 'Resonance'
        };
        await invoke('export_midi', { path, project: ustxProject });
      }
    } catch (e) {
      console.error('Export error:', e);
    }
  };
  
  return (
    <div className="flex items-center gap-2 px-4 py-2 bg-gray-800 border-b border-gray-700">
      <div className="flex items-center gap-1">
        <button
          onClick={undo}
          disabled={!canUndo()}
          className="px-2 py-1.5 bg-gray-700 hover:bg-gray-600 disabled:opacity-30 rounded text-white text-sm"
          title="Ctrl+Z"
        >
          ↩
        </button>
        <button
          onClick={redo}
          disabled={!canRedo()}
          className="px-2 py-1.5 bg-gray-700 hover:bg-gray-600 disabled:opacity-30 rounded text-white text-sm"
          title="Ctrl+Shift+Z"
        >
          ↪
        </button>
      </div>
      
      <div className="h-6 w-px bg-gray-600 mx-1" />
      
      <div className="flex items-center gap-1">
        <button
          onClick={handlePlay}
          disabled={loading}
          className="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 rounded text-white text-sm"
        >
          {isPlaying ? '⏹ Stop' : '▶ Play'}
        </button>
        
        <button
          onClick={handleStop}
          className="px-3 py-1.5 bg-gray-700 hover:bg-gray-600 rounded text-white text-sm"
        >
          ⏹
        </button>
      </div>
      
      <div className="h-6 w-px bg-gray-600 mx-2" />
      
      <button
        onClick={handleExportMidi}
        className="px-3 py-1.5 bg-green-700 hover:bg-green-600 rounded text-white text-sm"
      >
        Export MIDI
      </button>
      
      <div className="flex items-center gap-2">
        <span className="text-gray-400 text-sm">BPM:</span>
        <input
          type="number"
          value={project.bpm}
          className="w-16 px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
          readOnly
        />
      </div>
      
      <Presets />
      
      <div className="flex-1" />
      <span className="text-gray-400 text-sm">{project.name}</span>
    </div>
  );
}
