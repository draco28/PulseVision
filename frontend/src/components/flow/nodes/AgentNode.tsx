import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { AgentNodeData } from '../../../stores/types';
import '../../../styles/flow-animations.css';

const kindBadgeColors: Record<string, string> = {
  llm: 'var(--node-agent)',
  sequential: 'var(--node-agent-sequential)',
  parallel: 'var(--node-agent-parallel)',
  loop: 'var(--node-agent-loop)',
};

export const AgentNode = memo(function AgentNode({ data }: { data: Record<string, unknown> }) {
  const d = data as unknown as AgentNodeData;
  const borderColor = kindBadgeColors[d.kind] || 'var(--node-agent)';
  const isError = d.status === 'error';

  const animClass = d.status === 'running'
    ? 'flow-node--running'
    : d.status === 'error'
    ? 'flow-node--error'
    : 'flow-node';

  return (
    <div
      className={animClass}
      style={{
        background: 'var(--surface)',
        border: `2px solid ${isError ? 'var(--node-error)' : borderColor}`,
        borderRadius: 'var(--radius-node)',
        padding: 'var(--node-padding)',
        minWidth: 160,
        fontFamily: 'var(--font-sans)',
      }}
    >
      <Handle type="target" position={Position.Left} style={{ background: borderColor }} />
      <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
        <span style={{ fontSize: 13, fontWeight: 600, color: 'var(--text-primary)' }}>
          {d.name}
        </span>
        {d.status === 'completed' && (
          <span className="flow-node__checkmark" style={{ color: 'var(--success)', fontSize: 14 }}>
            &#10003;
          </span>
        )}
        {d.status === 'error' && (
          <span style={{ color: 'var(--error)', fontSize: 14 }}>&#10007;</span>
        )}
      </div>
      <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
        <span
          style={{
            fontSize: 10,
            padding: '1px 6px',
            borderRadius: 3,
            background: borderColor,
            color: d.kind === 'sequential' ? 'var(--text-secondary)' : '#fff',
            fontWeight: 500,
            textTransform: 'uppercase',
          }}
        >
          {d.kind}
        </span>
      </div>
      <Handle type="source" position={Position.Right} style={{ background: borderColor }} />
    </div>
  );
});
