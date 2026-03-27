import { useCallback, useEffect } from 'react';
import { useSpaceStore } from '../stores/spaceStore';
import { useUiStore } from '../stores/uiStore';

export function useSubstrate() {
  const collectiveId = useUiStore((s) => s.collectiveId);
  const setExperiences = useSpaceStore((s) => s.setExperiences);
  const setProjections = useSpaceStore((s) => s.setProjections);
  const setRelations = useSpaceStore((s) => s.setRelations);
  const setAttractors = useSpaceStore((s) => s.setAttractors);
  const setLoading = useSpaceStore((s) => s.setLoading);
  const setVarianceExplained = useSpaceStore((s) => s.setVarianceExplained);
  const setEmbeddingDimension = useSpaceStore((s) => s.setEmbeddingDimension);

  const fetchSubstrate = useCallback(async () => {
    if (!collectiveId) return;
    setLoading(true);

    try {
      const [expRes, projRes, relRes, attrRes] = await Promise.all([
        fetch(`/api/substrate/experiences?collective_id=${collectiveId}&limit=5000`),
        fetch(`/api/substrate/embeddings?collective_id=${collectiveId}`),
        fetch(`/api/substrate/relations?collective_id=${collectiveId}&limit=5000`),
        fetch(`/api/substrate/attractors?collective_id=${collectiveId}&threshold=0.3`),
      ]);

      if (expRes.ok) {
        const data = await expRes.json();
        setExperiences(
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
        setProjections(data.projections || []);
        setVarianceExplained(data.variance_explained || []);
        setEmbeddingDimension(data.embedding_dimension || 0);
      }

      if (relRes.ok) {
        const data = await relRes.json();
        setRelations(
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
        setAttractors(
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
    } catch {
      // Backend may not be running
    } finally {
      setLoading(false);
    }
  }, [collectiveId, setExperiences, setProjections, setRelations, setAttractors, setLoading, setVarianceExplained, setEmbeddingDimension]);

  // Fetch when collective changes
  useEffect(() => {
    fetchSubstrate();
  }, [fetchSubstrate]);

  return { fetchSubstrate };
}
