import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { ToolCallNodeData } from '../../../stores/types';
import '../../../styles/flow-animations.css';

export const ToolCallNode = memo(function ToolCallNode({ data }: { data: Record<string, unknown> }) {
  const d = data as unknown as ToolCallNodeData;

  return (
    <div
      className="flow-node"
      style={{
        background: 'var(--surface)',
        border: '2px solid var(--node-tool)',
        width: 90,
        height: 90,
        transform: 'rotate(45deg)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
      }}
    >
      <Handle
        type="target"
        position={Position.Left}
        style={{ background: 'var(--node-tool)', transform: 'rotate(-45deg)' }}
      />
      <div
        style={{
          transform: 'rotate(-45deg)',
          textAlign: 'center',
          fontFamily: 'var(--font-sans)',
        }}
      >
        <span style={{ fontSize: 10, color: 'var(--text-secondary)' }}>Tool</span>
        <br />
        <span style={{ fontSize: 11, fontWeight: 500, color: 'var(--text-primary)' }}>
          {d.toolName.length > 12 ? d.toolName.slice(0, 12) + '...' : d.toolName}
        </span>
        {d.durationMs != null && (
          <>
            <br />
            <span style={{ fontSize: 9, color: 'var(--node-tool)', fontFamily: 'var(--font-mono)' }}>
              {d.durationMs}ms
            </span>
          </>
        )}
      </div>
      <Handle
        type="source"
        position={Position.Right}
        style={{ background: 'var(--node-tool)', transform: 'rotate(-45deg)' }}
      />
    </div>
  );
});
