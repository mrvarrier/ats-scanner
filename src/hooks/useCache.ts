import { useState, useEffect, useCallback, useRef } from 'react';

export interface CacheEntry<T> {
  data: T;
  timestamp: number;
  expiresAt: number;
  accessCount: number;
  lastAccessed: number;
}

export interface CacheOptions {
  ttl?: number; // Time to live in milliseconds
  maxSize?: number; // Maximum number of entries
  staleWhileRevalidate?: boolean; // Return stale data while fetching fresh
  onEvict?: (_key: string, _entry: CacheEntry<unknown>) => void;
}

export class MemoryCache {
  private cache = new Map<string, CacheEntry<unknown>>();
  private options: Required<CacheOptions>;

  constructor(options: CacheOptions = {}) {
    this.options = {
      ttl: 5 * 60 * 1000, // 5 minutes default
      maxSize: 100,
      staleWhileRevalidate: false,
      onEvict: () => {},
      ...options,
    };
  }

  set<T>(key: string, data: T, customTtl?: number): void {
    const now = Date.now();
    const ttl = customTtl ?? this.options.ttl;

    const entry: CacheEntry<T> = {
      data,
      timestamp: now,
      expiresAt: now + ttl,
      accessCount: 0,
      lastAccessed: now,
    };

    // Evict if cache is full
    if (this.cache.size >= this.options.maxSize && !this.cache.has(key)) {
      this.evictLRU();
    }

    this.cache.set(key, entry);
  }

  get<T>(key: string): T | null {
    const entry = this.cache.get(key) as CacheEntry<T> | undefined;

    if (!entry) {
      return null;
    }

    const now = Date.now();

    // Update access info
    entry.accessCount++;
    entry.lastAccessed = now;

    // Check if expired
    if (now > entry.expiresAt) {
      this.cache.delete(key);
      this.options.onEvict(key, entry);
      return null;
    }

    return entry.data;
  }

  has(key: string): boolean {
    const entry = this.cache.get(key);
    if (!entry) return false;

    const now = Date.now();
    if (now > entry.expiresAt) {
      this.cache.delete(key);
      this.options.onEvict(key, entry);
      return false;
    }

    return true;
  }

  delete(key: string): boolean {
    const entry = this.cache.get(key);
    const deleted = this.cache.delete(key);

    if (deleted && entry) {
      this.options.onEvict(key, entry);
    }

    return deleted;
  }

  clear(): void {
    const entries = Array.from(this.cache.entries());
    this.cache.clear();

    entries.forEach(([key, entry]) => {
      this.options.onEvict(key, entry);
    });
  }

  size(): number {
    return this.cache.size;
  }

  keys(): string[] {
    return Array.from(this.cache.keys());
  }

  // Get cache statistics
  getStats() {
    const entries = Array.from(this.cache.values());
    const now = Date.now();

    return {
      size: this.cache.size,
      maxSize: this.options.maxSize,
      expired: entries.filter(entry => now > entry.expiresAt).length,
      totalAccesses: entries.reduce((sum, entry) => sum + entry.accessCount, 0),
      averageAge:
        entries.length > 0
          ? entries.reduce((sum, entry) => sum + (now - entry.timestamp), 0) /
            entries.length
          : 0,
    };
  }

  private evictLRU(): void {
    let oldestKey: string | null = null;
    let oldestTime = Date.now();

    for (const [key, entry] of this.cache.entries()) {
      if (entry.lastAccessed < oldestTime) {
        oldestTime = entry.lastAccessed;
        oldestKey = key;
      }
    }

    if (oldestKey) {
      const entry = this.cache.get(oldestKey)!;
      this.cache.delete(oldestKey);
      this.options.onEvict(oldestKey, entry);
    }
  }

  // Clean up expired entries
  cleanup(): number {
    const now = Date.now();
    let cleanedUp = 0;

    for (const [key, entry] of this.cache.entries()) {
      if (now > entry.expiresAt) {
        this.cache.delete(key);
        this.options.onEvict(key, entry);
        cleanedUp++;
      }
    }

    return cleanedUp;
  }
}

// Global cache instance
const globalCache = new MemoryCache({
  ttl: 5 * 60 * 1000, // 5 minutes
  maxSize: 200,
  staleWhileRevalidate: true,
});

// React hook for using cache
export function useCache<T>(
  key: string,
  fetcher: () => Promise<T>,
  options: {
    ttl?: number;
    enabled?: boolean;
    staleWhileRevalidate?: boolean;
    onSuccess?: (_data: T) => void;
    onError?: (_error: Error) => void;
  } = {}
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [stale, setStale] = useState(false);

  const fetcherRef = useRef(fetcher);
  fetcherRef.current = fetcher;

  const fetchData = useCallback(
    async (force = false) => {
      if (!options.enabled && options.enabled !== undefined) {
        return;
      }

      // Check cache first
      const cached = globalCache.get<T>(key);
      if (cached && !force) {
        setData(cached);
        setError(null);
        setStale(false);
        options.onSuccess?.(cached);
        return cached;
      }

      // If stale-while-revalidate and we have cached data, return it but fetch fresh
      if (options.staleWhileRevalidate && cached && !force) {
        setData(cached);
        setStale(true);
      } else {
        setLoading(true);
      }

      try {
        const fresh = await fetcherRef.current();
        globalCache.set(key, fresh, options.ttl);
        setData(fresh);
        setError(null);
        setStale(false);
        setLoading(false);
        options.onSuccess?.(fresh);
        return fresh;
      } catch (err) {
        const error = err instanceof Error ? err : new Error(String(err));
        setError(error);
        setLoading(false);
        setStale(false);
        options.onError?.(error);
        throw error;
      }
    },
    [key, options]
  );

  const mutate = useCallback(
    (newData?: T) => {
      if (newData !== undefined) {
        globalCache.set(key, newData, options.ttl);
        setData(newData);
        setError(null);
        setStale(false);
      } else {
        // Revalidate
        void fetchData(true);
      }
    },
    [key, options.ttl, fetchData]
  );

  const invalidate = useCallback(() => {
    globalCache.delete(key);
    setData(null);
    setError(null);
    setStale(false);
  }, [key]);

  // Fetch on mount and when key changes
  useEffect(() => {
    void fetchData();
  }, [fetchData]);

  return {
    data,
    loading,
    error,
    stale,
    mutate,
    invalidate,
    refetch: () => fetchData(true),
  };
}

// Hook for analysis result caching
export function useAnalysisCache<T>(
  resumeContent: string,
  jobDescriptionId: string,
  modelName: string,
  fetcher: () => Promise<T>
) {
  // Create a stable cache key from inputs
  const cacheKey = `analysis:${hashString(resumeContent)}:${jobDescriptionId}:${modelName}`;

  return useCache(cacheKey, fetcher, {
    ttl: 30 * 60 * 1000, // 30 minutes for analysis results
    enabled: !!(resumeContent && jobDescriptionId && modelName),
    staleWhileRevalidate: true,
  });
}

// Hook for model list caching
export function useModelCache<T>(fetcher: () => Promise<T>) {
  return useCache('ollama-models', fetcher, {
    ttl: 10 * 60 * 1000, // 10 minutes for model list
    staleWhileRevalidate: true,
  });
}

// Hook for job descriptions caching
export function useJobDescriptionsCache<T>(fetcher: () => Promise<T>) {
  return useCache('job-descriptions', fetcher, {
    ttl: 5 * 60 * 1000, // 5 minutes for job descriptions
    staleWhileRevalidate: true,
  });
}

// Persistent cache using localStorage
export class PersistentCache extends MemoryCache {
  private storageKey: string;
  private saveInterval: ReturnType<typeof setInterval>;
  private beforeUnloadHandler: () => void;

  constructor(storageKey: string, options: CacheOptions = {}) {
    super(options);
    this.storageKey = storageKey;
    this.loadFromStorage();

    // Save to storage periodically
    this.saveInterval = setInterval(() => {
      this.saveToStorage();
    }, 60000); // Every minute

    // Save on page unload
    this.beforeUnloadHandler = () => {
      this.saveToStorage();
    };
    window.addEventListener('beforeunload', this.beforeUnloadHandler);
  }

  destroy(): void {
    if (this.saveInterval) {
      clearInterval(this.saveInterval);
    }
    window.removeEventListener('beforeunload', this.beforeUnloadHandler);
    this.saveToStorage(); // Final save
    this.clear();
  }

  set<T>(key: string, data: T, customTtl?: number): void {
    super.set(key, data, customTtl);
    this.saveToStorage();
  }

  delete(key: string): boolean {
    const result = super.delete(key);
    this.saveToStorage();
    return result;
  }

  clear(): void {
    super.clear();
    this.saveToStorage();
  }

  private loadFromStorage(): void {
    try {
      const stored = localStorage.getItem(this.storageKey);
      if (stored) {
        const data = JSON.parse(stored) as Record<string, unknown>;
        const now = Date.now();

        // Restore non-expired entries
        Object.entries(data).forEach(([key, entry]) => {
          if (entry && typeof entry === 'object' && 'expiresAt' in entry) {
            const cacheEntry = entry as CacheEntry<unknown>;
            if (cacheEntry.expiresAt > now) {
              this.cache.set(key, cacheEntry);
            }
          }
        });
      }
    } catch {
      // Failed to load cache from storage - continue with empty cache
    }
  }

  private saveToStorage(): void {
    try {
      const data = Object.fromEntries(this.cache.entries());
      localStorage.setItem(this.storageKey, JSON.stringify(data));
    } catch {
      // Failed to save cache to storage - continue without persistence
    }
  }
}

// Utility functions
function hashString(str: string): string {
  let hash = 0;
  for (let i = 0; i < str.length; i++) {
    const char = str.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return hash.toString(36);
}

// Cache invalidation patterns
export const cacheInvalidation = {
  // Invalidate analysis cache when job description changes
  onJobDescriptionUpdate: (jobDescriptionId: string) => {
    const keys = globalCache.keys();
    keys.forEach(key => {
      if (key.includes(`analysis:`) && key.includes(`:${jobDescriptionId}:`)) {
        globalCache.delete(key);
      }
    });
  },

  // Invalidate all analysis cache
  onModelUpdate: () => {
    const keys = globalCache.keys();
    keys.forEach(key => {
      if (key.startsWith('analysis:')) {
        globalCache.delete(key);
      }
    });
  },

  // Clear expired entries
  cleanup: () => {
    return globalCache.cleanup();
  },
};

// Export global cache for direct access
export { globalCache };
