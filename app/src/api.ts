/**
 * API client for Spectra Time-Travel Analytics
 *
 * Communicates with the Spectra Server to fetch historical snapshots
 * and velocity analytics.
 */

import { VelocityReport, AgentSnapshot, AggregateResponse } from './types';

const SERVER_URL = import.meta.env.VITE_SERVER_URL || 'http://localhost:3000';

/**
 * Fetch available timestamps for a specific agent
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
 * Fetch a snapshot at or closest before a given timestamp (#2 - Time-Travel)
 */
export async function fetchSnapshotAtTime(
  agentId: string,
  timestamp?: number
): Promise<AgentSnapshot | null> {
  try {
    const params = timestamp ? `?timestamp=${timestamp}` : '';
    const url = `${SERVER_URL}/api/v1/snapshot/${agentId}${params}`;
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    return await response.json();
  } catch (error) {
    console.error('Failed to fetch snapshot:', error);
    return null;
  }
}

/**
 * Fetch time-series aggregation with configurable bucket sizes (#2 - Time-Travel)
 *
 * Returns an AggregateResponse with a `truncated` flag indicating whether
 * the server-side snapshot cap was hit (results may be incomplete for very
 * large time ranges).
 */
export async function fetchAggregate(
  agentId: string,
  start: number,
  end: number,
  bucketSeconds: number = 3600
): Promise<AggregateResponse> {
  try {
    const url = `${SERVER_URL}/api/v1/aggregate/${agentId}?start=${start}&end=${end}&bucket_seconds=${bucketSeconds}`;
    const response = await fetch(url);

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data: AggregateResponse = await response.json();
    if (data.truncated) {
      console.warn(`Aggregate results truncated for ${agentId} (${start}-${end}). Consider narrowing the time range.`);
    }
    return data;
  } catch (error) {
    console.error('Failed to fetch aggregate:', error);
    return { buckets: [], truncated: false };
  }
}

/**
 * Format bytes to human-readable string with sign
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
 */
export function formatTimestamp(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString();
}

/**
 * Format velocity (bytes/sec) to human-readable rate
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
