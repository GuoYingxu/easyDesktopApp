import { defineStore } from 'pinia';
import type { LogConfig } from '../models/LogConfig';
import { invoke } from '@tauri-apps/api/core'
export const useLogStore = defineStore('log', {
  // State
  state: ():{
    logConfig: LogConfig;
    isLoading: boolean;
    error: string |null;
  } => ({
    logConfig: {
      max_file_size: 10 * 1024 * 1024, // 10 MB default
      keep_files: 10,                   // Keep 10 archived files
      level: 'Info',                    // Default log level
      to_stdout: true,                  // Also output to stdout
    } as LogConfig,
    isLoading: false,
    error: null as string | null,
  }),

  // Actions
  actions: {
    async loadLogConfig(): Promise<void> {
      this.isLoading = true;
      this.error = null;
      try{
        // Call the Tauri command to get log configuration
        const config = await invoke<LogConfig>('get_log_config');
        this.logConfig = config;
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Failed to load log configuration';
        console.error('Error loading log config:', err);
      } finally {
        this.isLoading = false;
      }
    },

    async updateLogConfig(config: LogConfig): Promise<void> {
      this.isLoading = true;
      this.error = null;

      try {
        // Validate the config before sending
        if (config.keep_files <= 0) {
          throw new Error('Keep files must be greater than 0');
        }
        if (config.max_file_size !== null && config.max_file_size <= 0) {
          throw new Error('Max file size must be greater than 0');
        }

        // Update the local state immediately
        this.logConfig = config;

        // Call the Tauri command to update log configuration
        await invoke('update_log_config', { config });
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Failed to update log configuration';
        console.error('Error updating log config:', err);
        // Revert the local state on error
        await this.loadLogConfig();
        throw err;
      } finally {
        this.isLoading = false;
      }
    },

    async resetLogConfig(): Promise<void> {
      this.isLoading = true;
      this.error = null;

      try {
        // Call the Tauri command to reset log configuration
        const defaultConfig = await invoke<LogConfig>('reset_log_config');
        this.logConfig = defaultConfig;
      } catch (err) {
        this.error = err instanceof Error ? err.message : 'Failed to reset log configuration';
        console.error('Error resetting log config:', err);
      } finally {
        this.isLoading = false;
      }
    },

    // Helper methods
    setLogLevel(level: string): void {
      this.logConfig.level = level;
    },

    setLogMaxFileSize(size: number | null): void {
      this.logConfig.max_file_size = size;
    },

    setKeepFiles(count: number): void {
      this.logConfig.keep_files = count;
    },

    setToStdout(enabled: boolean): void {
      this.logConfig.to_stdout = enabled;
    },
  },
});