import { useEffect, useRef } from 'react';
import { useSpaceStore } from '../stores/spaceStore';
import { useUiStore } from '../stores/uiStore';

export function useSubstrate() {
  const collectiveId = useUiStore((s) => s.collectiveId);
  const fetchedRef = useRef<string | null>(null);

  useEffect(() => {
    if (!collectiveId || collectiveId === fetchedRef.current) return;
    fetchedRef.current = collectiveId;

    const store = useSpaceStore.getState();
    store.setLoading(true);

    Promise.all([
      fetch(`/api/substrate/experiences?collective_id=${collectiveId}&limit=5000`),
      fetch(`/api/substrate/embeddings?collective_id=${collectiveId}`),
      fetch(`/api/substrate/relations?collective_id=${collectiveId}&limit=5000`),
      fetch(`/api/substrate/attractors?collective_id=${collectiveId}&threshold=0.3`),
    ])
      .then(async ([expRes, projRes, relRes, attrRes]) => {
        const s = useSpaceStore.getState();

        if (expRes.ok) {
          const data = await expRes.json();
          s.setExperiences(
            (data.experiences || []).map((e: Record<string, unknown>) => ({
              id: e.id as string,
              contentPreview: e.content_preview as string,
              experienceType: e.experience_type as string,
              importance: e.importance as number,
              confidence: e.confidence as number,
              applications: e.applications as number,
              domain: e.domain as string[],
              timestampMs: e.timestamp_ms as number,
            }))
          );
        }

        if (projRes.ok) {
          const data = await projRes.json();
          s.setProjections(data.projections || []);
          s.setVarianceExplained(data.variance_explained || []);
          s.setEmbeddingDimension(data.embedding_dimension || 0);
        }

        if (relRes.ok) {
          const data = await relRes.json();
          s.setRelations(
            (data.relations || []).map((r: Record<string, unknown>) => ({
              id: r.id as string,
              sourceId: r.source_id as string,
              targetId: r.target_id as string,
              relationType: r.relation_type as string,
              strength: r.strength as number,
            }))
          );
        }

        if (attrRes.ok) {
          const data = await attrRes.json();
          s.setAttractors(
            (data.attractors || []).map((a: Record<string, unknown>) => ({
              experienceId: a.experience_id as string,
              position: a.position as { x: number; y: number; z: number },
              strength: a.strength as number,
              influenceRadius: a.influence_radius as number,
              warpFactor: a.warp_factor as number,
              experienceType: a.experience_type as string,
            }))
          );
        }
      })
      .catch(() => {
        // Backend may not be running
      })
      .finally(() => {
        useSpaceStore.getState().setLoading(false);
      });
  }, [collectiveId]);
}
