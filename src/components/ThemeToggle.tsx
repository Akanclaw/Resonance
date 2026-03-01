export function ThemeToggle() {
  const themes = ['Dark', 'Light', 'Blue', 'Green'];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Theme:</span>
      {themes.map(t => (
        <button
          key={t}
          className="px-2 py-1 bg-gray-700 hover:bg-gray-600 rounded text-white text-xs"
        >
          {t}
        </button>
      ))}
    </div>
  );
}
