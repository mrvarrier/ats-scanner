import { useEffect, useState, useCallback } from 'react';
import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from '@tauri-apps/api/updater';
import { relaunch } from '@tauri-apps/api/process';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Progress } from '@/components/ui/progress';
import { Download, RefreshCw, CheckCircle, AlertCircle } from 'lucide-react';
import { useToast } from '@/hooks/use-toast';

interface UpdateStatus {
  available: boolean;
  version?: string;
  date?: string;
  body?: string;
  downloading: boolean;
  downloaded: boolean;
  progress: number;
  error?: string;
}

export function UpdateChecker() {
  const [updateStatus, setUpdateStatus] = useState<UpdateStatus>({
    available: false,
    downloading: false,
    downloaded: false,
    progress: 0,
  });
  const [checking, setChecking] = useState(false);
  const { toast } = useToast();

  const checkForUpdates = useCallback(async () => {
    try {
      setChecking(true);
      setUpdateStatus(prev => ({ ...prev, error: undefined }));

      const update = await checkUpdate();

      if (update.shouldUpdate) {
        setUpdateStatus({
          available: true,
          version: update.manifest?.version,
          date: update.manifest?.date,
          body: update.manifest?.body,
          downloading: false,
          downloaded: false,
          progress: 0,
        });

        toast({
          title: 'Update Available',
          description: `Version ${update.manifest?.version} is available for download.`,
        });
      } else {
        setUpdateStatus(prev => ({ ...prev, available: false }));
      }
    } catch (error) {
      setUpdateStatus(prev => ({ ...prev, error: String(error) }));
    } finally {
      setChecking(false);
    }
  }, [toast]);

  useEffect(() => {
    // Listen for updater events
    const unlisten = onUpdaterEvent(({ error, status }) => {
      if (error) {
        setUpdateStatus(prev => ({ ...prev, error: error }));
        toast({
          title: 'Update Error',
          description: error,
          variant: 'destructive',
        });
      }

      if (status === 'PENDING') {
        setUpdateStatus(prev => ({ ...prev, downloading: true }));
      } else if (status === 'DONE') {
        setUpdateStatus(prev => ({
          ...prev,
          downloading: false,
          downloaded: true,
          progress: 100,
        }));
        toast({
          title: 'Update Downloaded',
          description:
            'The update has been downloaded and is ready to install.',
        });
      } else if (status === 'UPTODATE') {
        toast({
          title: 'No Updates',
          description: 'You are running the latest version.',
        });
      }
    });

    // Check for updates on component mount
    void checkForUpdates();

    return () => {
      void unlisten.then(fn => fn());
    };
  }, [checkForUpdates, toast]);

  const downloadAndInstall = async () => {
    try {
      setUpdateStatus(prev => ({ ...prev, downloading: true, progress: 0 }));

      // Install the update
      await installUpdate();
    } catch (error) {
      setUpdateStatus(prev => ({
        ...prev,
        downloading: false,
        error: String(error),
      }));
      toast({
        title: 'Installation Failed',
        description: String(error),
        variant: 'destructive',
      });
    }
  };

  const restartAndUpdate = async () => {
    try {
      await relaunch();
    } catch {
      toast({
        title: 'Restart Failed',
        description:
          'Please restart the application manually to complete the update.',
        variant: 'destructive',
      });
    }
  };

  if (!updateStatus.available && !updateStatus.error && !checking) {
    return (
      <Button
        variant="ghost"
        size="sm"
        onClick={checkForUpdates}
        disabled={checking}
        className="text-xs"
      >
        <RefreshCw
          className={`mr-1 h-3 w-3 ${checking ? 'animate-spin' : ''}`}
        />
        Check for Updates
      </Button>
    );
  }

  return (
    <Card className="w-full max-w-md">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          {updateStatus.error && (
            <AlertCircle className="h-5 w-5 text-red-500" />
          )}
          {updateStatus.downloaded && (
            <CheckCircle className="h-5 w-5 text-green-500" />
          )}
          {updateStatus.downloading && (
            <Download className="h-5 w-5 text-blue-500" />
          )}
          Application Update
        </CardTitle>
        <CardDescription>
          {updateStatus.available && !updateStatus.error && (
            <>Version {updateStatus.version} is available</>
          )}
          {updateStatus.error && <>Update check failed</>}
          {checking && <>Checking for updates...</>}
        </CardDescription>
      </CardHeader>

      <CardContent className="space-y-4">
        {updateStatus.available && updateStatus.body && (
          <div className="space-y-2">
            <h4 className="text-sm font-medium">What&apos;s New:</h4>
            <div className="max-h-32 overflow-y-auto whitespace-pre-wrap text-xs text-muted-foreground">
              {updateStatus.body}
            </div>
          </div>
        )}

        {updateStatus.downloading && (
          <div className="space-y-2">
            <div className="flex justify-between text-xs">
              <span>Downloading update...</span>
              <span>{updateStatus.progress}%</span>
            </div>
            <Progress value={updateStatus.progress} />
          </div>
        )}

        {updateStatus.error && (
          <div className="rounded bg-red-50 p-2 text-xs text-red-600">
            {updateStatus.error}
          </div>
        )}

        <div className="flex gap-2">
          {updateStatus.available &&
            !updateStatus.downloading &&
            !updateStatus.downloaded && (
              <Button onClick={downloadAndInstall} size="sm" className="flex-1">
                <Download className="mr-2 h-4 w-4" />
                Download Update
              </Button>
            )}

          {updateStatus.downloaded && (
            <Button onClick={restartAndUpdate} size="sm" className="flex-1">
              <RefreshCw className="mr-2 h-4 w-4" />
              Restart & Install
            </Button>
          )}

          <Button
            variant="outline"
            onClick={checkForUpdates}
            disabled={checking || updateStatus.downloading}
            size="sm"
          >
            <RefreshCw
              className={`h-4 w-4 ${checking ? 'animate-spin' : ''}`}
            />
          </Button>
        </div>

        {updateStatus.date && (
          <div className="text-xs text-muted-foreground">
            Released: {new Date(updateStatus.date).toLocaleDateString()}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
