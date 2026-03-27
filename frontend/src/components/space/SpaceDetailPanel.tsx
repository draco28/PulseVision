import { useEffect, useCallback } from 'react';
import { useSpaceStore } from '../../stores/spaceStore';

export function SpaceDetailPanel() {
  const selectedId = useSpaceStore((s) => s.selectedId);
  const experiences = useSpaceStore((s) => s.experiences);
  const setSelectedId = useSpaceStore((s) => s.setSelectedId);

  const exp = selectedId ? experiences.find((e) => e.id === selectedId) : null;

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') setSelectedId(null);
    },
    [setSelectedId]
  );

  useEffect(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  if (!exp) return null;

  const typeName = exp.experienceType.split(/[\s{(]/)[0];

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
        <span
          style={{
            fontSize: 11,
            fontWeight: 600,
            textTransform: 'uppercase',
            padding: '2px 10px',
            borderRadius: 12,
            background: 'var(--node-experience)',
            color: '#fff',
          }}
        >
          Experience
        </span>
        <button
          onClick={() => setSelectedId(null)}
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

      <dl style={{ display: 'grid', gap: 12 }}>
        <Field label="Type" value={typeName} />
        <Field label="Importance" value={exp.importance.toFixed(2)} />
        <Field label="Confidence" value={exp.confidence.toFixed(2)} />
        <Field label="Applications" value={String(exp.applications)} />
        <Field label="Domain" value={exp.domain.join(', ')} />
        <Field label="Content" value={exp.contentPreview} mono />
        <Field label="ID" value={exp.id} mono />
      </dl>
    </div>
  );
}

function Field({ label, value, mono }: { label: string; value: string; mono?: boolean }) {
  return (
    <div>
      <dt style={{ fontSize: 11, color: 'var(--text-secondary)', marginBottom: 2 }}>{label}</dt>
      <dd
        style={{
          fontSize: 13,
          color: 'var(--text-primary)',
          fontFamily: mono ? 'var(--font-mono)' : 'var(--font-sans)',
          wordBreak: 'break-word',
        }}
      >
        {value}
      </dd>
    </div>
  );
}
