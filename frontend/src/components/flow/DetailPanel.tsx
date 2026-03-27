import { useEffect, useCallback } from 'react';
import type { FlowNode, AgentNodeData, LlmCallNodeData, ToolCallNodeData, ExperienceNodeData } from '../../stores/types';

interface DetailPanelProps {
  node: FlowNode;
  onClose: () => void;
}

export function DetailPanel({ node, onClose }: DetailPanelProps) {
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    },
    [onClose]
  );

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  return (
    <div
      style={{
        width: 'var(--detail-panel-width)',
        background: 'var(--surface-elevated)',
        borderLeft: '1px solid var(--border)',
        padding: 'var(--panel-padding)',
        overflowY: 'auto',
        flexShrink: 0,
      }}
    >
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
        <NodeTypeBadge nodeType={node.data.nodeType} />
        <button
          onClick={onClose}
          style={{
            background: 'transparent',
            border: '1px solid var(--border)',
            color: 'var(--text-secondary)',
            borderRadius: 'var(--radius-button)',
            padding: '2px 8px',
            cursor: 'pointer',
            fontSize: 12,
          }}
        >
          ESC
        </button>
      </div>

      <NodeDetail data={node.data} />
    </div>
  );
}

function NodeTypeBadge({ nodeType }: { nodeType: string }) {
  const colors: Record<string, string> = {
    agent: 'var(--node-agent)',
    llmCall: 'var(--node-llm)',
    toolCall: 'var(--node-tool)',
    experience: 'var(--node-experience)',
  };

  const labels: Record<string, string> = {
    agent: 'Agent',
    llmCall: 'LLM Call',
    toolCall: 'Tool Call',
    experience: 'Experience',
  };

  return (
    <span
      style={{
        fontSize: 11,
        fontWeight: 600,
        textTransform: 'uppercase',
        padding: '2px 10px',
        borderRadius: 12,
        background: colors[nodeType] || 'var(--border)',
        color: '#fff',
      }}
    >
      {labels[nodeType] || nodeType}
    </span>
  );
}

function NodeDetail({ data }: { data: FlowNode['data'] }) {
  switch (data.nodeType) {
    case 'agent':
      return <AgentDetail data={data} />;
    case 'llmCall':
      return <LlmCallDetail data={data} />;
    case 'toolCall':
      return <ToolCallDetail data={data} />;
    case 'experience':
      return <ExperienceDetail data={data} />;
    default:
      return null;
  }
}

function AgentDetail({ data }: { data: AgentNodeData }) {
  const duration = data.completedAt && data.startedAt
    ? ((data.completedAt - data.startedAt) / 1000).toFixed(1)
    : null;

  return (
    <dl style={{ display: 'grid', gap: 12 }}>
      <Field label="Name" value={data.name} />
      <Field label="Kind" value={data.kind} />
      <Field label="Status" value={data.status} />
      {data.outcome && (
        <Field
          label="Outcome"
          value={
            data.outcome.status === 'complete'
              ? data.outcome.response.slice(0, 200)
              : data.outcome.status === 'error'
              ? data.outcome.error
              : 'Max iterations reached'
          }
          mono
        />
      )}
      {duration && <Field label="Duration" value={`${duration}s`} />}
    </dl>
  );
}

function LlmCallDetail({ data }: { data: LlmCallNodeData }) {
  return (
    <dl style={{ display: 'grid', gap: 12 }}>
      <Field label="Model" value={data.model} />
      <Field label="Status" value={data.status} />
      {data.durationMs != null && <Field label="Duration" value={`${data.durationMs}ms`} />}
      {data.inputTokens != null && <Field label="Input Tokens" value={data.inputTokens.toLocaleString()} />}
      {data.outputTokens != null && <Field label="Output Tokens" value={data.outputTokens.toLocaleString()} />}
      {data.inputTokens != null && data.outputTokens != null && (
        <Field label="Total Tokens" value={(data.inputTokens + data.outputTokens).toLocaleString()} />
      )}
    </dl>
  );
}

function ToolCallDetail({ data }: { data: ToolCallNodeData }) {
  let prettyParams = data.params;
  try {
    prettyParams = JSON.stringify(JSON.parse(data.params), null, 2);
  } catch {
    // Use raw params
  }

  return (
    <dl style={{ display: 'grid', gap: 12 }}>
      <Field label="Tool" value={data.toolName} />
      <Field label="Status" value={data.status} />
      {data.durationMs != null && <Field label="Duration" value={`${data.durationMs}ms`} />}
      <Field label="Parameters" value={prettyParams} mono />
      {data.resultPreview && <Field label="Result" value={data.resultPreview} mono />}
    </dl>
  );
}

function ExperienceDetail({ data }: { data: ExperienceNodeData }) {
  return (
    <dl style={{ display: 'grid', gap: 12 }}>
      <Field label="ID" value={data.experienceId} mono />
      <Field label="Type" value={data.experienceType} />
      <Field label="Importance" value={data.importance.toFixed(2)} />
      <Field label="Content" value={data.contentPreview} mono />
    </dl>
  );
}

function Field({ label, value, mono }: { label: string; value: string | number; mono?: boolean }) {
  return (
    <div>
      <dt style={{ fontSize: 11, color: 'var(--text-secondary)', marginBottom: 2 }}>{label}</dt>
      <dd
        style={{
          fontSize: 13,
          color: 'var(--text-primary)',
          fontFamily: mono ? 'var(--font-mono)' : 'var(--font-sans)',
          wordBreak: 'break-word',
          whiteSpace: mono ? 'pre-wrap' : undefined,
        }}
      >
        {value}
      </dd>
    </div>
  );
}
