import { create } from 'zustand';
import type { ActiveTab, Collective, ConnectionStatus } from './types';

interface UiStoreState {
  activeTab: ActiveTab;
  selectedNodeId: string | null;
  connectionStatus: ConnectionStatus;
  collectiveId: string | null;
  collectives: Collective[];

  setActiveTab: (tab: ActiveTab) => void;
  setSelectedNodeId: (id: string | null) => void;
  setConnectionStatus: (status: ConnectionStatus) => void;
  setCollectiveId: (id: string | null) => void;
  setCollectives: (collectives: Collective[]) => void;
}

export const useUiStore = create<UiStoreState>((set) => ({
  activeTab: 'flow',
  selectedNodeId: null,
  connectionStatus: 'disconnected',
  collectiveId: null,
  collectives: [],

  setActiveTab: (tab) => set({ activeTab: tab }),
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  setConnectionStatus: (status) => set({ connectionStatus: status }),
  setCollectiveId: (id) => set({ collectiveId: id }),
  setCollectives: (collectives) => set({ collectives }),
}));
