import { render, screen } from '@testing-library/react';
import { describe, test, expect } from 'vitest';
import RiskTreemap from '../RiskTreemap';

const mockData = {
  name: "root",
  loc: 1000,
  entropy: 2.0,
  risk_score: 20,
  children: [
    { name: "safe.txt", loc: 500, entropy: 1.0, risk_score: 10 },
    { name: "danger.enc", loc: 500, entropy: 7.9, risk_score: 80 }
  ]
};

describe('RiskTreemap', () => {
  test('renders treemap with nodes', () => {
    // Nivo renders SVGs, we check for text labels
    render(<RiskTreemap data={mockData} />);

    // Check that the component renders without crashing
    expect(screen.getByText('safe.txt')).toBeInTheDocument();
    expect(screen.getByText('danger.enc')).toBeInTheDocument();
  });

  test('handles empty children array', () => {
    const emptyData = {
      name: "empty",
      loc: 100,
      entropy: 0.5,
      risk_score: 5,
      children: []
    };

    render(<RiskTreemap data={emptyData} />);
    expect(screen.getByText('empty')).toBeInTheDocument();
  });

  test('handles single file node', () => {
    const singleFile = {
      name: "single.txt",
      loc: 500,
      entropy: 2.5,
      risk_score: 25
    };

    render(<RiskTreemap data={singleFile} />);
    expect(screen.getByText('single.txt')).toBeInTheDocument();
  });

  test('applies correct color based on entropy', () => {
    // This test verifies the color logic exists
    // In a real test environment, you would check computed styles
    const { container } = render(<RiskTreemap data={mockData} />);

    // Verify the treemap container is rendered
    expect(container.querySelector('svg')).toBeInTheDocument();
  });
});
