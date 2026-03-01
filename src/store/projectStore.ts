import { create } from 'zustand';
import type { Project, Note, Track } from '../types';

interface ProjectState {
  project: Project;
  currentTrackIndex: number;
  selectedNotes: number[];
  isPlaying: boolean;
  setProject: (project: Project) => void;
  setCurrentTrack: (index: number) => void;
  addTrack: (track: Track) => void;
  addNote: (trackIndex: number, note: Note) => void;
  setPlaying: (playing: boolean) => void;
  selectNote: (noteIndex: number) => void;
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
  setProject: (project) => set({ project }),
  setCurrentTrack: (index) => set({ currentTrackIndex: index }),
  addTrack: (track) => set((state) => ({
    project: { ...state.project, tracks: [...state.project.tracks, track] }
  })),
  addNote: (trackIndex, note) => set((state) => {
    const tracks = [...state.project.tracks];
    tracks[trackIndex] = { ...tracks[trackIndex], notes: [...tracks[trackIndex].notes, note] };
    return { project: { ...state.project, tracks } };
  }),
  setPlaying: (playing) => set({ isPlaying: playing }),
  selectNote: (noteIndex) => set({ selectedNotes: [noteIndex] })
}));
