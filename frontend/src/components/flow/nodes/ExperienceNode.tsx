import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { ExperienceNodeData } from '../../../stores/types';

export const ExperienceNode = memo(function ExperienceNode({ data }: { data: Record<string, unknown> }) {
  const d = data as unknown as ExperienceNodeData;

  return (
    <div
      style={{
        background: 'var(--surface)',
        border: '2px solid var(--node-experience)',
        borderRadius: 'var(--radius-node)',
        padding: 10,
        minWidth: 120,
        maxWidth: 180,
        fontFamily: 'var(--font-sans)',
        clipPath: 'polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%)',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        textAlign: 'center',
        minHeight: 80,
      }}
    >
      <Handle type="target" position={Position.Left} style={{ background: 'var(--node-experience)' }} />
      <span style={{ fontSize: 10, color: 'var(--node-experience)' }}>Exp</span>
      <span
        style={{
          fontSize: 10,
          color: 'var(--text-primary)',
          overflow: 'hidden',
          textOverflow: 'ellipsis',
          whiteSpace: 'nowrap',
          maxWidth: 100,
        }}
      >
        {d.contentPreview.slice(0, 30)}
      </span>
      <Handle type="source" position={Position.Right} style={{ background: 'var(--node-experience)' }} />
    </div>
  );
});
