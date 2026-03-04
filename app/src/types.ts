export interface FileRecord {
  path: string;
  size_bytes: number;
}

export interface ExtensionStat {
  count: number;
  size: number;
}

export interface ScanStats {
  root_path: string;
  total_files: number;
  total_folders: number;
  total_size_bytes: number;
  scan_duration_ms: number;
  extensions: Record<string, ExtensionStat>;
  top_files: FileRecord[];
  device_type?: string;
  threads_used?: number;
}

// --- Scan Progress (#1 - Progressive scan) ---

export interface ScanProgress {
  files_scanned: number;
  folders_scanned: number;
  bytes_scanned: number;
}

// --- Time-Travel Analytics Types ---

export interface AgentSnapshot {
  agent_id: string;
  timestamp: number; // Unix epoch in seconds
  hostname: string;
  total_size_bytes: number;
  file_count: number;
  top_extensions: [string, number, number][]; // [extension, size, count]
}

export interface ExtensionDelta {
  extension: string;
  size_delta: number;
  count_delta: number;
}

export interface VelocityReport {
  agent_id: string;
  t_start: number;
  t_end: number;
  duration_seconds: number;
  growth_bytes: number; // Can be negative (shrinkage)
  growth_files: number;
  bytes_per_second: number; // The Velocity (delta_bytes / delta_time)
  extension_deltas: ExtensionDelta[];
}

// --- Time-Series Aggregation (#2) ---

export interface TimeSeriesBucket {
  bucket_start: number;
  bucket_end: number;
  avg_size_bytes: number;
  avg_file_count: number;
  snapshot_count: number;
}

export interface AggregateResponse {
  buckets: TimeSeriesBucket[];
  /** True if the snapshot count hit the server-side cap (results may be incomplete). */
  truncated: boolean;
}
