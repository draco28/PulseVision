import { memo } from 'react';
import { useUiStore } from '../../stores/uiStore';

export const CollectiveSelector = memo(function CollectiveSelector() {
  const collectives = useUiStore((s) => s.collectives);
  const collectiveId = useUiStore((s) => s.collectiveId);
  const setCollectiveId = useUiStore((s) => s.setCollectiveId);

  if (collectives.length === 0) return null;

  return (
    <select
      value={collectiveId || ''}
      onChange={(e) => setCollectiveId(e.target.value || null)}
      style={{
        background: 'var(--surface-elevated)',
        color: 'var(--text-primary)',
        border: '1px solid var(--border)',
        borderRadius: 'var(--radius-button)',
        padding: '4px 8px',
        fontSize: 12,
        fontFamily: 'var(--font-sans)',
        cursor: 'pointer',
        outline: 'none',
      }}
    >
      {collectives.map((c) => (
        <option key={c.id} value={c.id}>
          {c.name}
        </option>
      ))}
    </select>
  );
});
