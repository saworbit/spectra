/**
 * API client for Spectra Time-Travel Analytics
 *
 * Communicates with the Spectra Server to fetch historical snapshots
 * and velocity analytics.
 */

import { VelocityReport } from './types';

const SERVER_URL = import.meta.env.VITE_SERVER_URL || 'http://localhost:3000';

/**
 * Fetch available timestamps for a specific agent
 *
 * @param agentId - The agent identifier
 * @returns Array of Unix timestamps (seconds) when snapshots are available
 */
export async function fetchAgentHistory(agentId: string): Promise<number[]> {
  try {
    const response = await fetch(`${SERVER_URL}/api/v1/history/${agentId}`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to fetch agent history:', error);
    return [];
  }
}

/**
 * Calculate velocity (growth rate) between two timestamps
 *
 * @param agentId - The agent identifier
 * @param startTime - Start timestamp (Unix seconds)
 * @param endTime - End timestamp (Unix seconds)
 * @returns VelocityReport with growth metrics and extension deltas
 */
export async function fetchVelocity(
  agentId: string,
  startTime: number,
  endTime: number
): Promise<VelocityReport | null> {
  try {
    const url = `${SERVER_URL}/api/v1/velocity/${agentId}?start=${startTime}&end=${endTime}`;
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data: VelocityReport = await response.json();

    // Check if we got meaningful data
    if (data.duration_seconds === 0) {
      console.warn('No velocity data available for the selected time range');
      return null;
    }

    return data;
  } catch (error) {
    console.error('Failed to fetch velocity:', error);
    return null;
  }
}

/**
 * Format bytes to human-readable string with sign
 *
 * @param bytes - Number of bytes (can be negative)
 * @param showSign - Whether to show + sign for positive values
 * @returns Formatted string like "+1.2 GB" or "-500 MB"
 */
export function formatBytes(bytes: number, showSign = false): string {
  const sign = bytes >= 0 ? (showSign ? '+' : '') : '-';
  const absBytes = Math.abs(bytes);

  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let value = absBytes;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex++;
  }

  const formatted = unitIndex === 0
    ? value.toFixed(0)
    : value.toFixed(2);

  return `${sign}${formatted} ${units[unitIndex]}`;
}

/**
 * Format Unix timestamp to human-readable date
 *
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date string
 */
export function formatTimestamp(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString();
}

/**
 * Format velocity (bytes/sec) to human-readable rate
 *
 * @param bytesPerSecond - Velocity in bytes per second
 * @returns Formatted string like "1.5 MB/s"
 */
export function formatVelocity(bytesPerSecond: number): string {
  const absRate = Math.abs(bytesPerSecond);
  const sign = bytesPerSecond >= 0 ? '' : '-';

  if (absRate === 0) return '0 B/s';

  const units = [
    { threshold: 1, label: 'B/s' },
    { threshold: 1024, label: 'KB/s' },
    { threshold: 1024 * 1024, label: 'MB/s' },
    { threshold: 1024 * 1024 * 1024, label: 'GB/s' },
  ];

  for (let i = units.length - 1; i >= 0; i--) {
    if (absRate >= units[i].threshold) {
      const value = absRate / units[i].threshold;
      return `${sign}${value.toFixed(2)} ${units[i].label}`;
    }
  }

  return `${sign}${absRate.toFixed(2)} B/s`;
}
