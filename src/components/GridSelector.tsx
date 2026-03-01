import { useState } from 'react';

export function GridSelector() {
  const [snap, setSnap] = useState(true);
  
  const gridOptions = [
    { name: '1/4', value: 120 },
    { name: '1/8', value: 60 },
    { name: '1/16', value: 30 },
    { name: '1/32', value: 15 },
  ];
  
  return (
    <div className="flex items-center gap-2 ml-4">
      <span className="text-gray-400 text-sm">Grid:</span>
      <button 
        onClick={() => setSnap(!snap)}
        className={`px-2 py-1 rounded text-sm ${snap ? 'bg-blue-700' : 'bg-gray-700'}`}
      >
        {snap ? 'ON' : 'OFF'}
      </button>
      <select 
        className="px-2 py-1 bg-gray-700 border border-gray-600 rounded text-white text-sm"
        defaultValue="60"
      >
        {gridOptions.map(opt => (
          <option key={opt.value} value={opt.value}>{opt.name}</option>
        ))}
      </select>
    </div>
  );
}
