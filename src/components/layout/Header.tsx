import { Moon, Sun, RefreshCw } from 'lucide-react';
import { invoke } from '@tauri-apps/api/tauri';
import { useAppStore } from '@/store/useAppStore';
import { Button } from '@/components/ui/button';
import { useToast } from '@/hooks/use-toast';
import type { CommandResult, OllamaModel } from '@/types';

export function Header() {
  const {
    isDarkMode,
    toggleDarkMode,
    selectedModel,
    setModels,
    setOllamaConnection,
  } = useAppStore();
  const { toast } = useToast();

  const handleRefreshConnection = async () => {
    try {
      const result = await invoke<CommandResult<boolean>>(
        'test_ollama_connection'
      );
      if (result.success && result.data) {
        setOllamaConnection(true);

        const modelsResult =
          await invoke<CommandResult<OllamaModel[]>>('get_ollama_models');
        if (modelsResult.success) {
          setModels(modelsResult.data ?? []);
          toast({
            title: 'Connection Refreshed',
            description: `Found ${modelsResult.data?.length ?? 0} models`,
          });
        }
      } else {
        setOllamaConnection(false);
        toast({
          title: 'Connection Failed',
          description: 'Unable to connect to Ollama service',
          variant: 'destructive',
        });
      }
    } catch (_error) {
      toast({
        title: 'Error',
        description: 'Failed to refresh connection',
        variant: 'destructive',
      });
    }
  };

  return (
    <header className="h-16 border-b border-border bg-card/50 backdrop-blur supports-[backdrop-filter]:bg-card/50">
      <div className="flex h-full items-center justify-between px-6">
        <div className="flex items-center space-x-4">
          <h2 className="text-lg font-semibold text-foreground">
            {selectedModel ? `Using: ${selectedModel}` : 'No Model Selected'}
          </h2>
        </div>

        <div className="flex items-center space-x-2">
          <Button
            variant="ghost"
            size="icon"
            onClick={handleRefreshConnection}
            className="h-9 w-9"
          >
            <RefreshCw className="h-4 w-4" />
          </Button>

          <Button
            variant="ghost"
            size="icon"
            onClick={toggleDarkMode}
            className="h-9 w-9"
          >
            {isDarkMode ? (
              <Sun className="h-4 w-4" />
            ) : (
              <Moon className="h-4 w-4" />
            )}
          </Button>
        </div>
      </div>
    </header>
  );
}
