import { useCallback } from 'react';
import { useUiStore } from '../stores/uiStore';
import type { Collective } from '../stores/types';

export function useApi() {
  const setCollectives = useUiStore((s) => s.setCollectives);
  const setCollectiveId = useUiStore((s) => s.setCollectiveId);

  const fetchCollectives = useCallback(async () => {
    try {
      const res = await fetch('/api/substrate/collectives');
      if (!res.ok) return;
      const data = await res.json();
      const collectives: Collective[] = data.collectives || [];
      setCollectives(collectives);
      if (collectives.length > 0 && !useUiStore.getState().collectiveId) {
        setCollectiveId(collectives[0].id);
      }
    } catch {
      // Backend may not be running yet
    }
  }, [setCollectives, setCollectiveId]);

  return { fetchCollectives };
}
