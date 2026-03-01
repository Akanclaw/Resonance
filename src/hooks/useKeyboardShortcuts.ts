import { useEffect, useCallback } from 'react';
import { useProjectStore } from '../store/projectStore';

export function useKeyboardShortcuts() {
  const { isPlaying, setPlaying, selectedNotes, currentTrackIndex, deleteNote } = useProjectStore();
  
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    // Space: Play/Stop
    if (e.code === 'Space' && !e.ctrlKey && !e.metaKey) {
      e.preventDefault();
      setPlaying(!isPlaying);
    }
    
    // Delete/Backspace: Delete selected note
    if ((e.code === 'Delete' || e.code === 'Backspace') && selectedNotes.length > 0) {
      e.preventDefault();
      deleteNote(currentTrackIndex, selectedNotes[0]);
    }
    
    // Escape: Clear selection
    if (e.code === 'Escape') {
      useProjectStore.getState().clearSelection();
    }
  }, [isPlaying, selectedNotes, currentTrackIndex, setPlaying, deleteNote]);
  
  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
}
