import { create } from 'zustand';
import type { Project, Note, Track } from '../types';

interface ProjectState {
  project: Project;
  currentTrackIndex: number;
  selectedNotes: number[];
  isPlaying: boolean;
  currentTick: number;
  setProject: (project: Project) => void;
  setCurrentTrack: (index: number) => void;
  addTrack: (track: Track) => void;
  addNote: (trackIndex: number, note: Note) => void;
  updateNote: (trackIndex: number, noteIndex: number, note: Partial<Note>) => void;
  deleteNote: (trackIndex: number, noteIndex: number) => void;
  setPlaying: (playing: boolean) => void;
  selectNote: (noteIndex: number) => void;
  clearSelection: () => void;
  setCurrentTick: (tick: number) => void;
}

const defaultProject: Project = {
  name: 'Untitled',
  bpm: 120,
  beatPerBar: 4,
  beatUnit: 4,
  tempo: [{ position: 0, bpm: 120 }],
  tracks: [{ name: 'Track 1', notes: [] }]
};

export const useProjectStore = create<ProjectState>((set) => ({
  project: defaultProject,
  currentTrackIndex: 0,
  selectedNotes: [],
  isPlaying: false,
  currentTick: 0,
  
  setProject: (project) => set({ project }),
  
  setCurrentTrack: (index) => set({ currentTrackIndex: index }),
  
  addTrack: (track) => set((state) => ({
    project: { ...state.project, tracks: [...state.project.tracks, track] }
  })),
  
  addNote: (trackIndex, note) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = { 
      ...tracks[trackIndex], 
      notes: [...tracks[trackIndex].notes, note].sort((a, b) => a.start - b.start) 
    };
    return { project: { ...state.project, tracks } };
  }),
  
  updateNote: (trackIndex, noteIndex, noteUpdate) => set((state) => {
    const tracks = [...state.project.tracks];
    const notes = [...tracks[trackIndex].notes];
    notes[noteIndex] = { ...notes[noteIndex], ...noteUpdate };
    return { project: { ...state.project, tracks } };
  }),
  
  deleteNote: (trackIndex, noteIndex) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = {
      ...tracks[trackIndex],
      notes: tracks[trackIndex].notes.filter((_, i) => i !== noteIndex)
    };
    return { project: { ...state.project, tracks }, selectedNotes: [] };
  }),
  
  setPlaying: (playing) => set({ isPlaying: playing }),
  
  selectNote: (noteIndex) => set({ selectedNotes: [noteIndex] }),
  
  clearSelection: () => set({ selectedNotes: [] }),
  
  setCurrentTick: (tick) => set({ currentTick: tick })
}));
