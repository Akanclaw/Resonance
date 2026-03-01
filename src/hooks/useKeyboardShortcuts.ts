import { useEffect, useCallback } from 'react';
import { useProjectStore } from '../store/projectStore';

export function useKeyboardShortcuts() {
  const { isPlaying, setPlaying, selectedNotes, currentTrackIndex, deleteNote, undo, redo, canUndo, canRedo } = useProjectStore();
  
  const handleKeyDown = useCallback((e: KeyboardEvent) => {
    // Undo: Ctrl+Z
    if (e.code === 'KeyZ' && (e.ctrlKey || e.metaKey) && !e.shiftKey) {
      e.preventDefault();
      if (canUndo()) undo();
    }
    
    // Redo: Ctrl+Shift+Z or Ctrl+Y
    if ((e.code === 'KeyZ' && (e.ctrlKey || e.metaKey) && e.shiftKey) || 
        (e.code === 'KeyY' && (e.ctrlKey || e.metaKey))) {
      e.preventDefault();
      if (canRedo()) redo();
    }
    
    // Space: Play/Stop
    if (e.code === 'Space' && !e.ctrlKey && !e.metaKey && !e.target?.toString().includes('input')) {
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
  }, [isPlaying, selectedNotes, currentTrackIndex, setPlaying, deleteNote, undo, redo, canUndo, canRedo]);
  
  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);
}
