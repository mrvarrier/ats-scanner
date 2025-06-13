import React, { useState, useEffect } from 'react';
import { analyzeAPI } from '../../utils/api';
import { OllamaModel, ModelResponse } from '../../types';
import LoadingSpinner from '../Common/LoadingSpinner';
import ErrorMessage from '../Common/ErrorMessage';

interface ModelSelectorProps {
  selectedModel: string;
  onModelChange: (model: string) => void;
}

const ModelSelector: React.FC<ModelSelectorProps> = ({ selectedModel, onModelChange }) => {
  const [models, setModels] = useState<OllamaModel[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [ollamaAvailable, setOllamaAvailable] = useState(false);

  const defaultModels: OllamaModel[] = [
    {
      name: 'mistral:latest',
      description: 'Faster analysis, good for quick scans',
      speed: 'fast'
    },
    {
      name: 'qwen2.5:14b',
      description: 'More detailed analysis, better insights',
      speed: 'slower'
    }
  ];

  useEffect(() => {
    loadModels();
  }, []);

  const loadModels = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const response: ModelResponse = await analyzeAPI.getModels();
      
      if (response.available) {
        setOllamaAvailable(true);
        setModels(response.recommended || defaultModels);
      } else {
        setOllamaAvailable(false);
        setModels(defaultModels);
        setError('Ollama service is not available. Please ensure Ollama is running.');
      }
    } catch (err: any) {
      setOllamaAvailable(false);
      setModels(defaultModels);
      setError(err.response?.data?.error || 'Failed to check Ollama availability');
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="card">
        <LoadingSpinner size="sm" text="Checking available models..." />
      </div>
    );
  }

  return (
    <div className="card space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-lg font-medium text-gray-900">AI Model Selection</h3>
        <div className={`px-2 py-1 rounded-full text-xs font-medium ${
          ollamaAvailable 
            ? 'bg-green-100 text-green-800' 
            : 'bg-red-100 text-red-800'
        }`}>
          {ollamaAvailable ? 'Ollama Online' : 'Ollama Offline'}
        </div>
      </div>

      {error && (
        <ErrorMessage message={error} onRetry={loadModels} />
      )}

      <div className="space-y-3">
        {models.map((model) => (
          <label
            key={model.name}
            className={`block p-4 border rounded-lg cursor-pointer transition-colors ${
              selectedModel === model.name
                ? 'border-primary bg-blue-50'
                : 'border-gray-200 hover:border-gray-300'
            } ${!ollamaAvailable ? 'opacity-50 cursor-not-allowed' : ''}`}
          >
            <input
              type="radio"
              name="model"
              value={model.name}
              checked={selectedModel === model.name}
              onChange={(e) => onModelChange(e.target.value)}
              disabled={!ollamaAvailable}
              className="sr-only"
            />
            
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <div className="flex items-center space-x-2">
                  <h4 className="font-medium text-gray-900">{model.name}</h4>
                  <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                    model.speed === 'fast' 
                      ? 'bg-green-100 text-green-800' 
                      : 'bg-yellow-100 text-yellow-800'
                  }`}>
                    {model.speed === 'fast' ? 'Fast' : 'Detailed'}
                  </span>
                </div>
                <p className="text-sm text-gray-600 mt-1">{model.description}</p>
              </div>
              
              <div className={`w-4 h-4 rounded-full border-2 flex items-center justify-center ${
                selectedModel === model.name
                  ? 'border-primary bg-primary'
                  : 'border-gray-300'
              }`}>
                {selectedModel === model.name && (
                  <div className="w-2 h-2 bg-white rounded-full"></div>
                )}
              </div>
            </div>
          </label>
        ))}
      </div>

      {!ollamaAvailable && (
        <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
          <h4 className="font-medium text-yellow-900 mb-2">⚠️ Ollama Required</h4>
          <p className="text-sm text-yellow-800 mb-2">
            To use the ATS Scanner, you need to have Ollama running with the required models.
          </p>
          <div className="text-sm text-yellow-800">
            <p className="mb-2">Setup instructions:</p>
            <ol className="list-decimal list-inside space-y-1 ml-2">
              <li>Install Ollama from <a href="https://ollama.ai" target="_blank" rel="noopener noreferrer" className="underline">ollama.ai</a></li>
              <li>Run: <code className="bg-yellow-100 px-1 rounded">ollama pull mistral:latest</code></li>
              <li>Run: <code className="bg-yellow-100 px-1 rounded">ollama pull qwen2.5:14b</code></li>
              <li>Refresh this page</li>
            </ol>
          </div>
        </div>
      )}
    </div>
  );
};

export default ModelSelector;