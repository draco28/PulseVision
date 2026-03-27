import { memo } from 'react';
import { useUiStore } from '../../stores/uiStore';
import { ConnectionStatus } from './ConnectionStatus';
import { CollectiveSelector } from './CollectiveSelector';
import type { ActiveTab } from '../../stores/types';

const tabs: { id: ActiveTab; label: string }[] = [
  { id: 'flow', label: 'Agent Flow' },
  { id: 'space', label: 'Substrate Space' },
];

export const Toolbar = memo(function Toolbar() {
  const activeTab = useUiStore((s) => s.activeTab);
  const setActiveTab = useUiStore((s) => s.setActiveTab);

  return (
    <div
      style={{
        height: 'var(--toolbar-height)',
        background: 'var(--surface)',
        borderBottom: '1px solid var(--border)',
        display: 'flex',
        alignItems: 'center',
        padding: '0 16px',
        gap: 24,
        flexShrink: 0,
      }}
    >
      <span
        style={{
          fontFamily: 'var(--font-mono)',
          fontWeight: 600,
          fontSize: 14,
          color: 'var(--text-primary)',
        }}
      >
        PulseVision
      </span>

      <ConnectionStatus />

      <div style={{ display: 'flex', gap: 0, marginLeft: 8 }}>
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            style={{
              background: 'transparent',
              border: 'none',
              color: activeTab === tab.id ? 'var(--text-primary)' : 'var(--text-secondary)',
              fontSize: 13,
              fontFamily: 'var(--font-sans)',
              fontWeight: 500,
              padding: '12px 16px',
              cursor: 'pointer',
              borderBottom: activeTab === tab.id ? '2px solid var(--accent)' : '2px solid transparent',
              transition: 'color 0.15s, border-color 0.15s',
            }}
          >
            {tab.label}
          </button>
        ))}
      </div>

      <div style={{ marginLeft: 'auto' }}>
        <CollectiveSelector />
      </div>
    </div>
  );
});
