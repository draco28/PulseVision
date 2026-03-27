import { describe, it, expect, beforeEach } from 'vitest';
import { useUiStore } from '../uiStore';

beforeEach(() => {
  // Reset store to defaults
  useUiStore.setState({
    activeTab: 'flow',
    selectedNodeId: null,
    connectionStatus: 'disconnected',
    collectiveId: null,
    collectives: [],
  });
});

describe('uiStore', () => {
  it('defaults to flow tab', () => {
    expect(useUiStore.getState().activeTab).toBe('flow');
  });

  it('switches tabs', () => {
    useUiStore.getState().setActiveTab('space');
    expect(useUiStore.getState().activeTab).toBe('space');
    useUiStore.getState().setActiveTab('flow');
    expect(useUiStore.getState().activeTab).toBe('flow');
  });

  it('manages node selection', () => {
    expect(useUiStore.getState().selectedNodeId).toBeNull();
    useUiStore.getState().setSelectedNodeId('node-1');
    expect(useUiStore.getState().selectedNodeId).toBe('node-1');
    useUiStore.getState().setSelectedNodeId(null);
    expect(useUiStore.getState().selectedNodeId).toBeNull();
  });

  it('tracks connection status', () => {
    expect(useUiStore.getState().connectionStatus).toBe('disconnected');
    useUiStore.getState().setConnectionStatus('reconnecting');
    expect(useUiStore.getState().connectionStatus).toBe('reconnecting');
    useUiStore.getState().setConnectionStatus('connected');
    expect(useUiStore.getState().connectionStatus).toBe('connected');
  });

  it('manages collectives', () => {
    const collectives = [
      { id: 'c1', name: 'project-1' },
      { id: 'c2', name: 'project-2' },
    ];
    useUiStore.getState().setCollectives(collectives);
    expect(useUiStore.getState().collectives).toHaveLength(2);
    expect(useUiStore.getState().collectives[0].name).toBe('project-1');
  });

  it('selects a collective', () => {
    useUiStore.getState().setCollectiveId('c1');
    expect(useUiStore.getState().collectiveId).toBe('c1');
  });
});
