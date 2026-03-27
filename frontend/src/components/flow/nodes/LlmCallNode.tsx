import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { LlmCallNodeData } from '../../../stores/types';

export const LlmCallNode = memo(function LlmCallNode({ data }: { data: Record<string, unknown> }) {
  const d = data as unknown as LlmCallNodeData;

  return (
    <div
      style={{
        background: 'var(--surface)',
        border: '2px solid var(--node-llm)',
        borderRadius: '50%',
        width: 80,
        height: 80,
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        fontFamily: 'var(--font-sans)',
      }}
    >
      <Handle type="target" position={Position.Left} style={{ background: 'var(--node-llm)' }} />
      <span style={{ fontSize: 10, color: 'var(--text-secondary)' }}>LLM</span>
      <span style={{ fontSize: 11, fontWeight: 500, color: 'var(--text-primary)' }}>
        {d.model.length > 10 ? d.model.slice(0, 10) + '...' : d.model}
      </span>
      {d.inputTokens != null && d.outputTokens != null && (
        <span style={{ fontSize: 9, color: 'var(--node-llm)', fontFamily: 'var(--font-mono)' }}>
          {d.inputTokens + d.outputTokens}t
        </span>
      )}
      <Handle type="source" position={Position.Right} style={{ background: 'var(--node-llm)' }} />
    </div>
  );
});
