import React from 'react';
import { 
  BarChart3, 
  FileText, 
  Wrench, 
  Settings,
  CheckCircle,
  XCircle,
  FolderOpen
} from 'lucide-react';
import { useAppStore } from '@/store/useAppStore';
import { cn } from '@/lib/utils';

const menuItems = [
  { id: 'dashboard', label: 'Dashboard', icon: BarChart3 },
  { id: 'analysis', label: 'Analysis', icon: FileText },
  { id: 'optimization', label: 'Optimization', icon: Wrench },
  { id: 'results', label: 'Results', icon: FolderOpen },
  { id: 'settings', label: 'Settings', icon: Settings },
];

export function Sidebar() {
  const { activeTab, setActiveTab, isOllamaConnected } = useAppStore();

  return (
    <div className="w-64 bg-card border-r border-border flex flex-col">
      {/* Logo and Title */}
      <div className="p-6 border-b border-border">
        <h1 className="text-xl font-bold text-foreground">ATS Scanner</h1>
        <p className="text-sm text-muted-foreground">Local AI Resume Analysis</p>
      </div>

      {/* Connection Status */}
      <div className="px-6 py-4 border-b border-border">
        <div className="flex items-center space-x-2">
          {isOllamaConnected ? (
            <>
              <CheckCircle className="h-4 w-4 text-green-500" />
              <span className="text-sm text-green-600">Ollama Connected</span>
            </>
          ) : (
            <>
              <XCircle className="h-4 w-4 text-red-500" />
              <span className="text-sm text-red-600">Ollama Disconnected</span>
            </>
          )}
        </div>
      </div>

      {/* Navigation Menu */}
      <nav className="flex-1 px-4 py-6">
        <ul className="space-y-2">
          {menuItems.map((item) => {
            const Icon = item.icon;
            return (
              <li key={item.id}>
                <button
                  onClick={() => setActiveTab(item.id)}
                  className={cn(
                    "w-full flex items-center space-x-3 px-3 py-2 rounded-lg text-left transition-colors",
                    activeTab === item.id
                      ? "bg-primary text-primary-foreground"
                      : "text-muted-foreground hover:text-foreground hover:bg-accent"
                  )}
                >
                  <Icon className="h-5 w-5" />
                  <span>{item.label}</span>
                </button>
              </li>
            );
          })}
        </ul>
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-border">
        <p className="text-xs text-muted-foreground text-center">
          v1.0.0 - Privacy First
        </p>
      </div>
    </div>
  );
}