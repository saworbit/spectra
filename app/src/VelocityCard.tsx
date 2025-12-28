/**
 * VelocityCard Component
 *
 * Displays data velocity (growth/shrinkage) between two snapshots
 * Shows total change, velocity rate, and per-extension breakdown
 */

import { VelocityReport } from './types';
import { formatBytes, formatTimestamp, formatVelocity } from './api';

interface VelocityCardProps {
  report: VelocityReport;
}

export function VelocityCard({ report }: VelocityCardProps) {
  const isGrowth = report.growth_bytes >= 0;
  const velocityClass = isGrowth ? 'velocity-positive' : 'velocity-negative';

  return (
    <div className="card velocity full-width">
      <h2>⏳ Time-Travel Analytics</h2>

      {/* Time Range */}
      <div className="stat-row">
        <span>📅 Period:</span>
        <strong>
          {formatTimestamp(report.t_start)} → {formatTimestamp(report.t_end)}
        </strong>
      </div>

      <div className="stat-row">
        <span>⏱️ Duration:</span>
        <strong>{formatDuration(report.duration_seconds)}</strong>
      </div>

      {/* Primary Metrics */}
      <div className={`velocity-metric ${velocityClass}`}>
        <div className="metric-label">
          {isGrowth ? '📈 Data Growth' : '📉 Data Shrinkage'}
        </div>
        <div className="metric-value">
          {formatBytes(report.growth_bytes, true)}
        </div>
        <div className="metric-secondary">
          {isGrowth ? '+' : ''}{report.growth_files} files
        </div>
      </div>

      <div className={`velocity-metric ${velocityClass}`}>
        <div className="metric-label">⚡ Velocity</div>
        <div className="metric-value">
          {formatVelocity(report.bytes_per_second)}
        </div>
      </div>

      {/* Extension Breakdown */}
      {report.extension_deltas.length > 0 && (
        <div className="extension-deltas">
          <h3>🔍 Top Contributors</h3>
          <table>
            <thead>
              <tr>
                <th>Extension</th>
                <th>Size Δ</th>
                <th>Files Δ</th>
              </tr>
            </thead>
            <tbody>
              {report.extension_deltas.slice(0, 10).map((delta) => {
                const deltaClass =
                  delta.size_delta > 0
                    ? 'delta-positive'
                    : delta.size_delta < 0
                    ? 'delta-negative'
                    : 'delta-neutral';

                return (
                  <tr key={delta.extension}>
                    <td>
                      <span className="badge">.{delta.extension}</span>
                    </td>
                    <td className={deltaClass}>
                      {formatBytes(delta.size_delta, true)}
                    </td>
                    <td className={deltaClass}>
                      {delta.count_delta >= 0 ? '+' : ''}
                      {delta.count_delta}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}

      {report.extension_deltas.length === 0 && (
        <div className="no-data">
          📊 No significant changes detected in this period
        </div>
      )}
    </div>
  );
}

/**
 * Format duration in seconds to human-readable string
 */
function formatDuration(seconds: number): string {
  if (seconds < 60) return `${seconds}s`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
  if (seconds < 86400) {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }

  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  return `${days}d ${hours}h`;
}
