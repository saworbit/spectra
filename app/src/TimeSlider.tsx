/**
 * TimeSlider Component
 *
 * Interactive timeline control for selecting time ranges
 * Allows users to pick two snapshots and trigger velocity calculation
 */

import { useState, useEffect } from 'react';
import { fetchAgentHistory, formatTimestamp } from './api';

interface TimeSliderProps {
  agentId: string;
  onRangeSelect: (startTime: number, endTime: number) => void;
}

export function TimeSlider({ agentId, onRangeSelect }: TimeSliderProps) {
  const [timestamps, setTimestamps] = useState<number[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedStart, setSelectedStart] = useState<number | null>(null);
  const [selectedEnd, setSelectedEnd] = useState<number | null>(null);

  useEffect(() => {
    async function loadHistory() {
      setLoading(true);
      const history = await fetchAgentHistory(agentId);

      if (history.length > 0) {
        // Sort timestamps in ascending order (oldest to newest)
        const sorted = history.sort((a, b) => a - b);
        setTimestamps(sorted);

        // Auto-select first and last if we have at least 2 snapshots
        if (sorted.length >= 2) {
          setSelectedStart(sorted[0]);
          setSelectedEnd(sorted[sorted.length - 1]);
          onRangeSelect(sorted[0], sorted[sorted.length - 1]);
        }
      }

      setLoading(false);
    }

    loadHistory();
  }, [agentId, onRangeSelect]);

  const handleStartChange = (index: number) => {
    const startTime = timestamps[index];
    setSelectedStart(startTime);

    // Ensure end is after start
    if (selectedEnd !== null && startTime >= selectedEnd) {
      const newEnd = timestamps[Math.min(index + 1, timestamps.length - 1)];
      setSelectedEnd(newEnd);
      onRangeSelect(startTime, newEnd);
    } else if (selectedEnd !== null) {
      onRangeSelect(startTime, selectedEnd);
    }
  };

  const handleEndChange = (index: number) => {
    const endTime = timestamps[index];
    setSelectedEnd(endTime);

    // Ensure start is before end
    if (selectedStart !== null && endTime <= selectedStart) {
      const newStart = timestamps[Math.max(index - 1, 0)];
      setSelectedStart(newStart);
      onRangeSelect(newStart, endTime);
    } else if (selectedStart !== null) {
      onRangeSelect(selectedStart, endTime);
    }
  };

  if (loading) {
    return (
      <div className="card time-slider">
        <h2>⏳ Time-Travel Controls</h2>
        <div className="loading">Loading timeline...</div>
      </div>
    );
  }

  if (timestamps.length === 0) {
    return (
      <div className="card time-slider">
        <h2>⏳ Time-Travel Controls</h2>
        <div className="no-data">
          📊 No historical data available for this agent.
          <br />
          Run the agent with <code>--server</code> flag to enable telemetry.
        </div>
      </div>
    );
  }

  if (timestamps.length === 1) {
    return (
      <div className="card time-slider">
        <h2>⏳ Time-Travel Controls</h2>
        <div className="no-data">
          📊 Only one snapshot available. Run another scan to compare.
        </div>
        <div className="timestamp-info">
          <strong>Available snapshot:</strong>
          <br />
          {formatTimestamp(timestamps[0])}
        </div>
      </div>
    );
  }

  const startIndex = selectedStart !== null
    ? timestamps.indexOf(selectedStart)
    : 0;

  const endIndex = selectedEnd !== null
    ? timestamps.indexOf(selectedEnd)
    : timestamps.length - 1;

  return (
    <div className="card time-slider">
      <h2>⏳ Time-Travel Controls</h2>

      <div className="slider-info">
        <span>📊 {timestamps.length} snapshots available</span>
        <span>
          📅 {formatTimestamp(timestamps[0])} → {formatTimestamp(timestamps[timestamps.length - 1])}
        </span>
      </div>

      <div className="slider-section">
        <label htmlFor="start-slider">
          <strong>Start Time</strong>
        </label>
        <div className="timestamp-display">
          {selectedStart !== null && formatTimestamp(selectedStart)}
        </div>
        <input
          id="start-slider"
          type="range"
          min={0}
          max={timestamps.length - 1}
          value={startIndex}
          onChange={(e) => handleStartChange(parseInt(e.target.value))}
          className="time-range-slider"
        />
        <div className="slider-markers">
          <span>Oldest</span>
          <span>Newest</span>
        </div>
      </div>

      <div className="slider-section">
        <label htmlFor="end-slider">
          <strong>End Time</strong>
        </label>
        <div className="timestamp-display">
          {selectedEnd !== null && formatTimestamp(selectedEnd)}
        </div>
        <input
          id="end-slider"
          type="range"
          min={0}
          max={timestamps.length - 1}
          value={endIndex}
          onChange={(e) => handleEndChange(parseInt(e.target.value))}
          className="time-range-slider"
        />
        <div className="slider-markers">
          <span>Oldest</span>
          <span>Newest</span>
        </div>
      </div>

      {selectedStart !== null && selectedEnd !== null && (
        <div className="range-summary">
          <strong>Selected Range:</strong>
          <br />
          {formatTimestamp(selectedStart)} → {formatTimestamp(selectedEnd)}
          <br />
          <em>
            ({endIndex - startIndex + 1} snapshot{endIndex - startIndex + 1 !== 1 ? 's' : ''})
          </em>
        </div>
      )}
    </div>
  );
}
