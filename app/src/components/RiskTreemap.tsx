import { ResponsiveTreeMap } from '@nivo/treemap';
import { useState } from 'react';

// Define the Data Shape
interface TreeNode {
  name: string;
  loc: number; // Size
  entropy: number;
  risk_score: number;
  children?: TreeNode[];
}

interface RiskTreemapProps {
  data: TreeNode; // The root node
}

const RiskTreemap = ({ data }: RiskTreemapProps) => {
  const [selectedNode, setSelectedNode] = useState<TreeNode | null>(null);

  return (
    <div style={{ height: '500px', width: '100%', background: '#1a1a1a' }}>
      <ResponsiveTreeMap
        data={data}
        identity="name"
        value="loc"
        valueFormat=".02s" // e.g. 1.2M
        margin={{ top: 10, right: 10, bottom: 10, left: 10 }}
        labelSkipSize={12}

        // --- The "Risk" Coloring Logic ---
        colors={(node: any) => {
          // Access the raw data of the node
          const entropy = node.data.entropy;

          // Green (Safe) -> Yellow (Mixed) -> Red (Danger)
          if (entropy < 3.0) return '#4caf50'; // Green
          if (entropy < 6.0) return '#ffeb3b'; // Yellow
          if (entropy < 7.5) return '#ff9800'; // Orange
          return '#f44336'; // Red (High Entropy/Risk)
        }}

        parentLabelTextColor={{ from: 'color', modifiers: [['darker', 2]] }}
        borderColor={{ from: 'color', modifiers: [['darker', 0.1]] }}

        // --- Interactivity ---
        onClick={(node) => {
          console.log("Inspecting:", node.data.name, "Entropy:", node.data.entropy);
          setSelectedNode(node.data as TreeNode);
        }}

        // Tooltip
        tooltip={({ node }) => (
            <div style={{ background: '#333', padding: '10px', color: '#fff', borderRadius: '4px' }}>
                <strong>{node.data.name}</strong><br/>
                Size: {node.formattedValue}<br/>
                Entropy: {(node.data as any).entropy.toFixed(2)} / 8.0
            </div>
        )}
      />
    </div>
  );
};

export default RiskTreemap;
