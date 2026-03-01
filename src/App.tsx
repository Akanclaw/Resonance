import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Toolbar, TrackList, PianoRoll } from "./components";
import "./App.css";

function App() {
  const [version, setVersion] = useState("");
  
  useEffect(() => {
    invoke<string>("get_version").then(setVersion).catch(console.error);
  }, []);
  
  return (
    <div className="h-screen flex flex-col bg-gray-900 text-white">
      <div className="flex items-center justify-between px-4 py-1 bg-gray-800 border-b border-gray-700">
        <span className="text-sm font-medium">Resonance {version && `v${version}`}</span>
      </div>
      <Toolbar />
      <div className="flex-1 flex overflow-hidden">
        <TrackList />
        <PianoRoll />
      </div>
      <div className="flex items-center justify-between px-4 py-1 bg-gray-800 border-t border-gray-700 text-xs text-gray-400">
        <span>Ready</span>
        <span>© 2026 Resonance Team</span>
      </div>
    </div>
  );
}

export default App;
