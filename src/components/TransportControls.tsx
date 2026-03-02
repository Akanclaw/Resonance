import { invoke } from '@tauri-apps/api/core';
import { useProjectStore } from '../store/projectStore';
import { useState, useEffect } from 'react';

export function TransportControls() {
  const { isPlaying, setPlaying, currentTick, setCurrentTick, project } = useProjectStore();
  const [playbackRate, setPlaybackRate] = useState(1);
  const [isLooping, setIsLooping] = useState(false);

  // Poll current position from backend
  useEffect(() => {
    if (!isPlaying) return;
    const interval = setInterval(async () => {
      try {
        const pos = await invoke<number>('get_current_position');
        setCurrentTick(pos);
      } catch (e) {
        // Backend not ready
      }
    }, 50);
    return () => clearInterval(interval);
  }, [isPlaying, setCurrentTick, setPlaying]);

  const formatTime = (tick: number) => {
    const ticksPerSecond = (project.bpm * 480) / 60;
    const totalSeconds = tick / ticksPerSecond;
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = Math.floor(totalSeconds % 60);
    const ms = Math.floor((totalSeconds % 1) * 1000);
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${ms.toString().padStart(3, '0')}`;
  };

  const ticksPerBeat = 480;
  const ticksPerBar = ticksPerBeat * project.beatPerBar;
  const currentBar = Math.floor(currentTick / ticksPerBar) + 1;
  const currentBeat = Math.floor((currentTick % ticksPerBar) / ticksPerBeat) + 1;

  const handlePlay = async () => {
    try {
      if (isPlaying) {
        await invoke('pause_audio');
      } else {
        await invoke('resume_audio');
      }
      setPlaying(!isPlaying);
    } catch (e) {
      console.error('Play/pause error:', e);
    }
  };

  const handleStop = async () => {
    try {
      await invoke('stop_audio');
      setPlaying(false);
      setCurrentTick(0);
    } catch (e) {
      console.error('Stop error:', e);
    }
  };

  const handleRewind = async () => {
    try {
      await invoke('seek_audio', { position: 0 });
      setCurrentTick(0);
    } catch (e) {
      console.error('Seek error:', e);
    }
  };

  const handleFastForward = async () => {
    const endTick = 10000;
    try {
      await invoke('seek_audio', { position: endTick });
      setCurrentTick(endTick);
    } catch (e) {
      console.error('Seek error:', e);
    }
  };

  const handlePlaybackRateChange = async (rate: number) => {
    try {
      await invoke('set_playback_rate', { rate });
      setPlaybackRate(rate);
    } catch (e) {
      console.error('Rate change error:', e);
    }
  };

  return (
    <div className="flex items-center gap-3 px-4 py-2 bg-gray-800 border-b border-gray-700">
      {/* Time display */}
      <div className="flex items-center gap-2 min-w-[120px]">
        <span className="text-xl font-mono text-white">{formatTime(currentTick)}</span>
        <span className="text-xs text-gray-400">{currentBar}:{currentBeat}</span>
      </div>

      {/* Transport buttons */}
      <div className="flex items-center gap-1">
        <button
          onClick={handleRewind}
          className="p-2 hover:bg-gray-700 rounded text-gray-300 hover:text-white transition-colors"
          title="Rewind to start"
        >
          ⏮
        </button>

        <button
          onClick={handlePlay}
          className="p-2 bg-blue-600 hover:bg-blue-700 rounded text-white transition-colors"
          title={isPlaying ? 'Pause' : 'Play'}
        >
          {isPlaying ? '⏸' : '▶'}
        </button>

        <button
          onClick={handleStop}
          className="p-2 hover:bg-gray-700 rounded text-gray-300 hover:text-white transition-colors"
          title="Stop"
        >
          ⏹
        </button>

        <button
          onClick={handleFastForward}
          className="p-2 hover:bg-gray-700 rounded text-gray-300 hover:text-white transition-colors"
          title="Fast forward to end"
        >
          ⏭
        </button>
      </div>

      {/* Playback rate */}
      <div className="flex items-center gap-2">
        <span className="text-xs text-gray-400">Speed:</span>
        <select
          value={playbackRate}
          onChange={(e) => handlePlaybackRateChange(Number(e.target.value))}
          className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-sm text-white"
        >
          <option value={0.5}>0.5x</option>
          <option value={0.75}>0.75x</option>
          <option value={1}>1x</option>
          <option value={1.25}>1.25x</option>
          <option value={1.5}>1.5x</option>
          <option value={2}>2x</option>
        </select>
      </div>

      {/* Loop toggle */}
      <button
        onClick={() => setIsLooping(!isLooping)}
        className={`p-2 rounded transition-colors ${
          isLooping ? 'bg-blue-600 text-white' : 'hover:bg-gray-700 text-gray-300'
        }`}
        title="Toggle loop"
      >
        🔁
      </button>
    </div>
  );
}
