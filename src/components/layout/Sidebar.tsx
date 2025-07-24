import {
  BarChart3,
  FileText,
  Wrench,
  Settings,
  CheckCircle,
  XCircle,
  FolderOpen,
  History,
} from 'lucide-react';
import { useAppStore } from '@/store/useAppStore';
import { cn } from '@/lib/utils';

const menuItems = [
  { id: 'dashboard', label: 'Dashboard', icon: BarChart3 },
  { id: 'analysis', label: 'Analysis', icon: FileText },
  { id: 'optimization', label: 'Optimization', icon: Wrench },
  { id: 'results', label: 'Results', icon: FolderOpen },
  { id: 'history', label: 'History', icon: History },
  { id: 'settings', label: 'Settings', icon: Settings },
];

export function Sidebar() {
  const { activeTab, setActiveTab, isOllamaConnected } = useAppStore();

  return (
    <div className="flex w-64 flex-col border-r border-border bg-card">
      {/* Logo and Title */}
      <div className="border-b border-border p-6">
        <h1 className="text-xl font-bold text-foreground">ATS Scanner</h1>
        <p className="text-sm text-muted-foreground">
          Local AI Resume Analysis
        </p>
      </div>

      {/* Connection Status */}
      <div className="border-b border-border px-6 py-4">
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
          {menuItems.map(item => {
            const Icon = item.icon;
            return (
              <li key={item.id}>
                <button
                  onClick={() => setActiveTab(item.id)}
                  className={cn(
                    'flex w-full items-center space-x-3 rounded-lg px-3 py-2 text-left transition-colors',
                    activeTab === item.id
                      ? 'bg-primary text-primary-foreground'
                      : 'text-muted-foreground hover:bg-accent hover:text-foreground'
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
      <div className="border-t border-border p-4">
        <p className="text-center text-xs text-muted-foreground">
          v1.0.0 - Privacy First
        </p>
      </div>
    </div>
  );
}
