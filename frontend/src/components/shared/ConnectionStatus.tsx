import { memo } from 'react';
import { useUiStore } from '../../stores/uiStore';

const statusConfig = {
  connected: { color: 'var(--success)', label: 'Connected' },
  disconnected: { color: 'var(--error)', label: 'Disconnected' },
  reconnecting: { color: 'var(--warning)', label: 'Reconnecting...' },
} as const;

export const ConnectionStatus = memo(function ConnectionStatus() {
  const status = useUiStore((s) => s.connectionStatus);
  const config = statusConfig[status];

  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
      <div
        style={{
          width: 8,
          height: 8,
          borderRadius: '50%',
          backgroundColor: config.color,
          boxShadow: status === 'reconnecting' ? `0 0 6px ${config.color}` : undefined,
        }}
      />
      <span style={{ fontSize: 12, color: 'var(--text-secondary)' }}>{config.label}</span>
    </div>
  );
});
