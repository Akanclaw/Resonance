import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Toolbar, TrackList, PianoRoll, NoteProperties, Timeline, StatusBar } from "./components";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";
import "./App.css";

function App() {
  const [version, setVersion] = useState("");
  
  useEffect(() => {
    invoke<string>("get_version").then(setVersion).catch(console.error);
  }, []);
  
  useKeyboardShortcuts();
  
  return (
    <div className="h-screen flex flex-col bg-gray-900 text-white">
      <div className="flex items-center justify-between px-4 py-1 bg-gray-800 border-b border-gray-700">
        <span className="text-sm font-medium">Resonance {version && `v${version}`}</span>
      </div>
      <Toolbar />
      <Timeline />
      <div className="flex-1 flex overflow-hidden">
        <TrackList />
        <PianoRoll />
        <NoteProperties />
      </div>
      <StatusBar />
    </div>
  );
}

export default App;
