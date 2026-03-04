/**
 * SunburstChart Component
 *
 * Alternative to treemaps for hierarchical filesystem data.
 * Concentric rings map naturally to directory depth and are more
 * visually stable than treemaps (which shuffle on small data changes).
 *
 * Inspired by Filelight/DaisyDisk visualization patterns.
 */

import { ResponsiveSunburst } from '@nivo/sunburst';
import { ExtensionStat } from '../types';

interface SunburstData {
  name: string;
  value?: number;
  children?: SunburstData[];
}

interface SunburstChartProps {
  extensions: Record<string, ExtensionStat>;
}

function extensionsToSunburst(
  extensions: Record<string, ExtensionStat>
): SunburstData {
  const sorted = Object.entries(extensions)
    .sort(([, a], [, b]) => b.size - a.size)
    .slice(0, 20);

  // Group small extensions into "other"
  const top = sorted.slice(0, 12);
  const rest = sorted.slice(12);
  const otherSize = rest.reduce((acc, [, d]) => acc + d.size, 0);

  const children: SunburstData[] = top.map(([ext, data]) => ({
    name: `.${ext}`,
    value: data.size,
  }));

  if (otherSize > 0) {
    children.push({
      name: 'other',
      value: otherSize,
    });
  }

  return {
    name: 'root',
    children,
  };
}

export function SunburstChart({ extensions }: SunburstChartProps) {
  const data = extensionsToSunburst(extensions);

  if (!data.children || data.children.length === 0) {
    return (
      <div className="no-data">No extension data to visualize</div>
    );
  }

  return (
    <div style={{ height: 380 }}>
      <ResponsiveSunburst
        data={data}
        margin={{ top: 10, right: 10, bottom: 10, left: 10 }}
        id="name"
        value="value"
        cornerRadius={3}
        borderWidth={1}
        borderColor={{ theme: 'background' }}
        colors={{ scheme: 'paired' }}
        inheritColorFromParent={true}
        enableArcLabels={true}
        arcLabelsSkipAngle={12}
        arcLabelsTextColor="#fff"
      />
    </div>
  );
}

export default SunburstChart;
