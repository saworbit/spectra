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
}
