import { useEffect } from 'react';
import { useWebSocket } from './hooks/useWebSocket';
import { useApi } from './hooks/useApi';
import { useUiStore } from './stores/uiStore';
import { useEventStore } from './stores/eventStore';
import { Toolbar } from './components/shared/Toolbar';
import { FlowCanvas } from './components/flow/FlowCanvas';
import { SpaceCanvas } from './components/space/SpaceCanvas';

function StatsBar() {
  const totalTokens = useEventStore((s) => s.totalTokens);
  const totalAgents = useEventStore((s) => s.totalAgents);
  const eventCount = useEventStore((s) => s.events.length);

  return (
    <div
      style={{
        height: 'var(--stats-bar-height)',
        background: 'var(--surface)',
        borderTop: '1px solid var(--border)',
        display: 'flex',
        alignItems: 'center',
        padding: '0 16px',
        gap: 24,
        flexShrink: 0,
        fontSize: 12,
      }}
    >
      <span>
        <span style={{ color: 'var(--text-secondary)' }}>Tokens </span>
        <span style={{ fontFamily: 'var(--font-mono)', fontWeight: 600 }}>
          {totalTokens.toLocaleString()}
        </span>
      </span>
      <span>
        <span style={{ color: 'var(--text-secondary)' }}>Agents </span>
        <span style={{ fontFamily: 'var(--font-mono)', fontWeight: 600 }}>{totalAgents}</span>
      </span>
      <span>
        <span style={{ color: 'var(--text-secondary)' }}>Events </span>
        <span style={{ fontFamily: 'var(--font-mono)', fontWeight: 600 }}>{eventCount}</span>
      </span>
    </div>
  );
}

export default function App() {
  const activeTab = useUiStore((s) => s.activeTab);
  const { fetchCollectives } = useApi();

  // Connect to WebSocket
  useWebSocket('/ws/events');

  // Load collectives on mount
  useEffect(() => {
    fetchCollectives();
  }, [fetchCollectives]);

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
      <Toolbar />
      <div style={{ flex: 1, overflow: 'hidden' }}>
        {activeTab === 'flow' && <FlowCanvas />}
        {activeTab === 'space' && <SpaceCanvas />}
      </div>
      <StatsBar />
    </div>
  );
}
