export function AboutDialog() {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-800 p-6 rounded-lg max-w-md">
        <h2 className="text-xl font-bold mb-4">Resonance</h2>
        <p className="text-gray-400 mb-2">Open Singing Synthesis Platform</p>
        <p className="text-gray-500 text-sm mb-4">A Rust + React rewrite of OpenUtau</p>
        <div className="text-gray-500 text-xs">
          <p>Version: 0.1.0</p>
          <p>Built with Tauri 2.0</p>
          <p className="mt-2">© 2026 Resonance Team</p>
        </div>
        <button 
          className="mt-4 px-4 py-2 bg-blue-600 rounded text-white"
          onClick={() => document.querySelector('.about-dialog')?.remove()}
        >
          Close
        </button>
      </div>
    </div>
  );
}
