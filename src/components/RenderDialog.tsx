import { useState } from 'react';

export type AudioFormat = 'wav' | 'mp3' | 'flac';
export type SampleRate = 44100 | 48000;
export type BitDepth = 16 | 24;

interface RenderDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onRender: (options: RenderOptions) => void;
}

export interface RenderOptions {
  format: AudioFormat;
  sampleRate: SampleRate;
  bitDepth: BitDepth;
  range: 'all' | 'selected';
}

export function RenderDialog({ isOpen, onClose, onRender }: RenderDialogProps) {
  const [format, setFormat] = useState<AudioFormat>('wav');
  const [sampleRate, setSampleRate] = useState<SampleRate>(44100);
  const [bitDepth, setBitDepth] = useState<BitDepth>(16);
  const [range, setRange] = useState<'all' | 'selected'>('all');
  const [isRendering, setIsRendering] = useState(false);
  const [progress, setProgress] = useState(0);
  
  const handleRender = () => {
    setIsRendering(true);
    setProgress(0);
    
    // Simulate rendering progress
    const interval = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval);
          setIsRendering(false);
          onClose();
          return 100;
        }
        return prev + 10;
      });
    }, 200);
    
    onRender({ format, sampleRate, bitDepth, range });
  };
  
  const handleCancel = () => {
    if (isRendering) {
      setIsRendering(false);
      setProgress(0);
    } else {
      onClose();
    }
  };
  
  if (!isOpen) return null;
  
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div 
        className="absolute inset-0 bg-black/60"
        onClick={!isRendering ? onClose : undefined}
      />
      
      {/* Dialog */}
      <div className="relative bg-gray-800 rounded-lg shadow-xl w-96 border border-gray-700">
        {/* Header */}
        <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
          <h2 className="text-lg font-semibold text-white">Export Audio</h2>
          <button
            onClick={handleCancel}
            className="text-gray-400 hover:text-white transition-colors"
            disabled={isRendering}
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd"/>
            </svg>
          </button>
        </div>
        
        {/* Content */}
        <div className="p-4 space-y-4">
          {/* Format selection */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Output Format
            </label>
            <div className="flex gap-2">
              {(['wav', 'mp3', 'flac'] as AudioFormat[]).map(fmt => (
                <button
                  key={fmt}
                  onClick={() => setFormat(fmt)}
                  disabled={isRendering}
                  className={`flex-1 py-2 px-3 rounded text-sm font-medium uppercase transition-colors ${
                    format === fmt
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                  }`}
                >
                  {fmt}
                </button>
              ))}
            </div>
          </div>
          
          {/* Sample rate */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Sample Rate
            </label>
            <div className="flex gap-2">
              {([44100, 48000] as SampleRate[]).map(rate => (
                <button
                  key={rate}
                  onClick={() => setSampleRate(rate)}
                  disabled={isRendering}
                  className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
                    sampleRate === rate
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                  }`}
                >
                  {rate / 1000} kHz
                </button>
              ))}
            </div>
          </div>
          
          {/* Bit depth */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Bit Depth
            </label>
            <div className="flex gap-2">
              {([16, 24] as BitDepth[]).map(depth => (
                <button
                  key={depth}
                  onClick={() => setBitDepth(depth)}
                  disabled={isRendering}
                  className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
                    bitDepth === depth
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                  }`}
                >
                  {depth}-bit
                </button>
              ))}
            </div>
          </div>
          
          {/* Output range */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Output Range
            </label>
            <div className="flex gap-2">
              <button
                onClick={() => setRange('all')}
                disabled={isRendering}
                className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
                  range === 'all'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                Entire Project
              </button>
              <button
                onClick={() => setRange('selected')}
                disabled={isRendering}
                className={`flex-1 py-2 px-3 rounded text-sm font-medium transition-colors ${
                  range === 'selected'
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                }`}
              >
                Selected Tracks
              </button>
            </div>
          </div>
          
          {/* Progress bar */}
          {isRendering && (
            <div>
              <div className="flex justify-between text-sm text-gray-400 mb-1">
                <span>Rendering...</span>
                <span>{progress}%</span>
              </div>
              <div className="h-2 bg-gray-700 rounded-full overflow-hidden">
                <div 
                  className="h-full bg-blue-600 transition-all duration-200"
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          )}
        </div>
        
        {/* Footer */}
        <div className="flex justify-end gap-2 px-4 py-3 border-t border-gray-700">
          <button
            onClick={handleCancel}
            className={`px-4 py-2 rounded text-sm font-medium transition-colors ${
              isRendering
                ? 'bg-red-600 hover:bg-red-500 text-white'
                : 'bg-gray-700 hover:bg-gray-600 text-gray-300'
            }`}
          >
            {isRendering ? 'Cancel' : 'Close'}
          </button>
          {!isRendering && (
            <button
              onClick={handleRender}
              className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded text-sm font-medium transition-colors"
            >
              Export
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
