export function ColorPicker() {
  const colors = ['#ef4444', '#f97316', '#eab308', '#22c55e', '#3b82f6', '#8b5cf6', '#ec4899', '#06b6d4'];
  
  return (
    <div className="flex items-center gap-1 ml-4">
      <span className="text-gray-400 text-sm">Color:</span>
      {colors.map(c => (
        <button
          key={c}
          className="w-5 h-5 rounded-full border border-gray-600"
          style={{ backgroundColor: c }}
        />
      ))}
    </div>
  );
}
