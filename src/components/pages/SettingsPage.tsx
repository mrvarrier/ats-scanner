import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useToast } from '@/hooks/use-toast';
import { useUserPreferences } from '@/hooks/useUserPreferences';
import { useAppStore } from '@/store/useAppStore';
import { UpdateChecker } from '@/components/UpdateChecker';

interface UserPreferences {
  id: string;
  user_id: string;
  
  // Ollama Settings
  ollama_host: string;
  ollama_port: number;
  default_model: string | null;
  connection_timeout_seconds: number;
  auto_connect_on_startup: boolean;
  
  // Analysis Settings
  default_optimization_level: 'Conservative' | 'Balanced' | 'Aggressive';
  auto_save_analyses: boolean;
  analysis_history_retention_days: number;
  enable_batch_notifications: boolean;
  
  // UI Preferences
  theme: 'Light' | 'Dark' | 'System' | 'HighContrast';
  language: string;
  sidebar_collapsed: boolean;
  show_advanced_features: boolean;
  animation_speed: 'None' | 'Reduced' | 'Normal' | 'Fast';
  
  // Data & Privacy
  data_storage_location: string | null;
  auto_backup_enabled: boolean;
  backup_frequency_hours: number;
  telemetry_enabled: boolean;
  
  // Notifications
  desktop_notifications: boolean;
  sound_notifications: boolean;
  email_notifications: boolean;
  notification_email: string | null;
  
  // Performance
  max_concurrent_analyses: number;
  cache_size_mb: number;
  enable_gpu_acceleration: boolean;
  
  // Export Settings
  default_export_format: 'JSON' | 'CSV' | 'PDF' | 'HTML';
  include_metadata_in_exports: boolean;
  compress_exports: boolean;
  
  created_at: string;
  updated_at: string;
}

export function SettingsPage() {
  const { userPreferences, isLoadingPreferences, updateUserPreferences, resetUserPreferences } = useUserPreferences();
  const [saving, setSaving] = useState(false);
  const { toast } = useToast();

  const updatePreference = async (updates: Partial<UserPreferences>) => {
    if (!userPreferences) return;

    try {
      setSaving(true);
      const success = await updateUserPreferences(updates);
      
      if (success) {
        toast({
          title: "Settings Updated",
          description: "Your preferences have been saved successfully.",
        });
      }
    } catch (error) {
      console.error('Failed to update preferences:', error);
    } finally {
      setSaving(false);
    }
  };

  const resetPreferences = async () => {
    try {
      setSaving(true);
      await resetUserPreferences();
    } catch (error) {
      console.error('Failed to reset preferences:', error);
    } finally {
      setSaving(false);
    }
  };

  const exportPreferences = async () => {
    try {
      const result = await invoke<{ success: boolean; data: string; error?: string }>('export_user_preferences');
      
      if (result.success && result.data) {
        const blob = new Blob([result.data], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'ats-scanner-userPreferences.json';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
        
        toast({
          title: "Export Complete",
          description: "Your preferences have been exported successfully.",
        });
      } else {
        toast({
          title: "Error",
          description: result.error || "Failed to export preferences",
          variant: "destructive",
        });
      }
    } catch (error) {
      console.error('Failed to export preferences:', error);
      toast({
        title: "Error",
        description: "Failed to export preferences",
        variant: "destructive",
      });
    }
  };

  if (isLoadingPreferences) {
    return (
      <div className="space-y-6">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">Loading your userPreferences...</p>
        </div>
      </div>
    );
  }

  if (!userPreferences) {
    return (
      <div className="space-y-6">
        <div className="space-y-2">
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground">Failed to load userPreferences.</p>
        </div>
        <Button onClick={() => window.location.reload()}>Retry</Button>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
        <p className="text-muted-foreground">
          Configure your ATS scanner preferences and manage your local setup.
        </p>
      </div>

      {/* Ollama Connection Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Ollama Connection</CardTitle>
          <CardDescription>Configure your connection to the Ollama service</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="ollama-host">Host</Label>
              <Input
                id="ollama-host"
                value={userPreferences.ollama_host}
                onChange={(e) => updatePreference({ ollama_host: e.target.value })}
                placeholder="http://localhost"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="ollama-port">Port</Label>
              <Input
                id="ollama-port"
                type="number"
                value={userPreferences.ollama_port}
                onChange={(e) => updatePreference({ ollama_port: parseInt(e.target.value) })}
                placeholder="11434"
              />
            </div>
          </div>
          <div className="space-y-2">
            <Label htmlFor="connection-timeout">Connection Timeout (seconds)</Label>
            <Input
              id="connection-timeout"
              type="number"
              value={userPreferences.connection_timeout_seconds}
              onChange={(e) => updatePreference({ connection_timeout_seconds: parseInt(e.target.value) })}
              placeholder="30"
            />
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.auto_connect_on_startup}
              onCheckedChange={(checked) => updatePreference({ auto_connect_on_startup: checked })}
            />
            <Label>Auto-connect on startup</Label>
          </div>
        </CardContent>
      </Card>

      {/* Analysis Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Analysis Settings</CardTitle>
          <CardDescription>Configure default analysis behavior</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="optimization-level">Default Optimization Level</Label>
            <Select
              value={userPreferences.default_optimization_level}
              onValueChange={(value) => updatePreference({ default_optimization_level: value as any })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select optimization level" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Conservative">Conservative</SelectItem>
                <SelectItem value="Balanced">Balanced</SelectItem>
                <SelectItem value="Aggressive">Aggressive</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="retention-days">Analysis History Retention (days)</Label>
            <Input
              id="retention-days"
              type="number"
              value={userPreferences.analysis_history_retention_days}
              onChange={(e) => updatePreference({ analysis_history_retention_days: parseInt(e.target.value) })}
              placeholder="90"
            />
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.auto_save_analyses}
              onCheckedChange={(checked) => updatePreference({ auto_save_analyses: checked })}
            />
            <Label>Auto-save analyses</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.enable_batch_notifications}
              onCheckedChange={(checked) => updatePreference({ enable_batch_notifications: checked })}
            />
            <Label>Enable batch analysis notifications</Label>
          </div>
        </CardContent>
      </Card>

      {/* UI Preferences */}
      <Card>
        <CardHeader>
          <CardTitle>User Interface</CardTitle>
          <CardDescription>Customize the application appearance</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="theme">Theme</Label>
            <Select
              value={userPreferences.theme}
              onValueChange={(value) => updatePreference({ theme: value as any })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select theme" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="Light">Light</SelectItem>
                <SelectItem value="Dark">Dark</SelectItem>
                <SelectItem value="System">System</SelectItem>
                <SelectItem value="HighContrast">High Contrast</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-2">
            <Label htmlFor="animation-speed">Animation Speed</Label>
            <Select
              value={userPreferences.animation_speed}
              onValueChange={(value) => updatePreference({ animation_speed: value as any })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select animation speed" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="None">None</SelectItem>
                <SelectItem value="Reduced">Reduced</SelectItem>
                <SelectItem value="Normal">Normal</SelectItem>
                <SelectItem value="Fast">Fast</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.sidebar_collapsed}
              onCheckedChange={(checked) => updatePreference({ sidebar_collapsed: checked })}
            />
            <Label>Collapse sidebar by default</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.show_advanced_features}
              onCheckedChange={(checked) => updatePreference({ show_advanced_features: checked })}
            />
            <Label>Show advanced features</Label>
          </div>
        </CardContent>
      </Card>

      {/* Performance Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Performance</CardTitle>
          <CardDescription>Configure performance and resource usage</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="max-concurrent">Maximum Concurrent Analyses</Label>
            <Input
              id="max-concurrent"
              type="number"
              value={userPreferences.max_concurrent_analyses}
              onChange={(e) => updatePreference({ max_concurrent_analyses: parseInt(e.target.value) })}
              placeholder="3"
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="cache-size">Cache Size (MB)</Label>
            <Input
              id="cache-size"
              type="number"
              value={userPreferences.cache_size_mb}
              onChange={(e) => updatePreference({ cache_size_mb: parseInt(e.target.value) })}
              placeholder="256"
            />
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.enable_gpu_acceleration}
              onCheckedChange={(checked) => updatePreference({ enable_gpu_acceleration: checked })}
            />
            <Label>Enable GPU acceleration (experimental)</Label>
          </div>
        </CardContent>
      </Card>

      {/* Privacy & Data */}
      <Card>
        <CardHeader>
          <CardTitle>Privacy & Data</CardTitle>
          <CardDescription>Manage your data and privacy settings</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="backup-frequency">Backup Frequency (hours)</Label>
            <Input
              id="backup-frequency"
              type="number"
              value={userPreferences.backup_frequency_hours}
              onChange={(e) => updatePreference({ backup_frequency_hours: parseInt(e.target.value) })}
              placeholder="24"
            />
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.auto_backup_enabled}
              onCheckedChange={(checked) => updatePreference({ auto_backup_enabled: checked })}
            />
            <Label>Enable automatic backups</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.telemetry_enabled}
              onCheckedChange={(checked) => updatePreference({ telemetry_enabled: checked })}
            />
            <Label>Enable telemetry</Label>
          </div>
        </CardContent>
      </Card>

      {/* Notifications */}
      <Card>
        <CardHeader>
          <CardTitle>Notifications</CardTitle>
          <CardDescription>Configure notification preferences</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.desktop_notifications}
              onCheckedChange={(checked) => updatePreference({ desktop_notifications: checked })}
            />
            <Label>Desktop notifications</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.sound_notifications}
              onCheckedChange={(checked) => updatePreference({ sound_notifications: checked })}
            />
            <Label>Sound notifications</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.email_notifications}
              onCheckedChange={(checked) => updatePreference({ email_notifications: checked })}
            />
            <Label>Email notifications</Label>
          </div>
          {userPreferences.email_notifications && (
            <div className="space-y-2">
              <Label htmlFor="notification-email">Notification Email</Label>
              <Input
                id="notification-email"
                type="email"
                value={userPreferences.notification_email || ''}
                onChange={(e) => updatePreference({ notification_email: e.target.value })}
                placeholder="your@email.com"
              />
            </div>
          )}
        </CardContent>
      </Card>

      {/* Export Settings */}
      <Card>
        <CardHeader>
          <CardTitle>Export Settings</CardTitle>
          <CardDescription>Configure default export behavior</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="export-format">Default Export Format</Label>
            <Select
              value={userPreferences.default_export_format}
              onValueChange={(value) => updatePreference({ default_export_format: value as any })}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select export format" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="JSON">JSON</SelectItem>
                <SelectItem value="CSV">CSV</SelectItem>
                <SelectItem value="PDF">PDF</SelectItem>
                <SelectItem value="HTML">HTML</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.include_metadata_in_exports}
              onCheckedChange={(checked) => updatePreference({ include_metadata_in_exports: checked })}
            />
            <Label>Include metadata in exports</Label>
          </div>
          <div className="flex items-center space-x-2">
            <Switch
              checked={userPreferences.compress_exports}
              onCheckedChange={(checked) => updatePreference({ compress_exports: checked })}
            />
            <Label>Compress export files</Label>
          </div>
        </CardContent>
      </Card>

      {/* Application Updates */}
      <Card>
        <CardHeader>
          <CardTitle>Application Updates</CardTitle>
          <CardDescription>Check for and install application updates</CardDescription>
        </CardHeader>
        <CardContent>
          <UpdateChecker />
        </CardContent>
      </Card>

      {/* Management Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Settings Management</CardTitle>
          <CardDescription>Export, import, or reset your settings</CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex gap-4">
            <Button onClick={exportPreferences} variant="outline">
              Export Settings
            </Button>
            <Button 
              onClick={resetPreferences} 
              variant="destructive"
              disabled={saving}
            >
              {saving ? 'Resetting...' : 'Reset to Defaults'}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}