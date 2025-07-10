import { useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useAppStore, UserPreferences } from '@/store/useAppStore';
import { useToast } from '@/hooks/use-toast';

export function useUserPreferences() {
  const {
    userPreferences,
    isLoadingPreferences,
    setUserPreferences,
    setIsLoadingPreferences,
  } = useAppStore();
  const { toast } = useToast();

  useEffect(() => {
    void loadUserPreferences();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const loadUserPreferences = useCallback(async () => {
    try {
      setIsLoadingPreferences(true);
      const result = await invoke<{
        success: boolean;
        data: UserPreferences;
        error?: string;
      }>('get_user_preferences');

      if (result.success && result.data) {
        setUserPreferences(result.data);
      } else {
        // Failed to load user preferences - use defaults
      }
    } catch {
      // Failed to load user preferences - use defaults
    } finally {
      setIsLoadingPreferences(false);
    }
  }, [setIsLoadingPreferences, setUserPreferences]);

  const updateUserPreferences = async (updates: Partial<UserPreferences>) => {
    try {
      const result = await invoke<{
        success: boolean;
        data: UserPreferences;
        error?: string;
      }>('update_user_preferences', {
        updates,
      });

      if (result.success && result.data) {
        setUserPreferences(result.data);
        return true;
      } else {
        toast({
          title: 'Error',
          description: result.error ?? 'Failed to update preferences',
          variant: 'destructive',
        });
        return false;
      }
    } catch {
      toast({
        title: 'Error',
        description: 'Failed to update preferences',
        variant: 'destructive',
      });
      return false;
    }
  };

  const resetUserPreferences = async () => {
    try {
      const result = await invoke<{
        success: boolean;
        data: UserPreferences;
        error?: string;
      }>('reset_user_preferences');

      if (result.success && result.data) {
        setUserPreferences(result.data);
        toast({
          title: 'Settings Reset',
          description: 'All preferences have been reset to defaults.',
        });
        return true;
      } else {
        toast({
          title: 'Error',
          description: result.error ?? 'Failed to reset preferences',
          variant: 'destructive',
        });
        return false;
      }
    } catch {
      toast({
        title: 'Error',
        description: 'Failed to reset preferences',
        variant: 'destructive',
      });
      return false;
    }
  };

  return {
    userPreferences,
    isLoadingPreferences,
    loadUserPreferences,
    updateUserPreferences,
    resetUserPreferences,
  };
}
