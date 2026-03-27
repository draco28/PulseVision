import { useMemo } from 'react';
import * as THREE from 'three';
import { useSpaceStore } from '../../stores/spaceStore';

const RELATION_COLORS: Record<string, string> = {
  Supports: '#3FB950',
  Contradicts: '#F85149',
  Elaborates: '#A371F7',
  Supersedes: '#58A6FF',
  Implies: '#58A6FF',
  RelatedTo: '#8B949E',
};

export function RelationLines() {
  const relations = useSpaceStore((s) => s.relations);
  const projections = useSpaceStore((s) => s.projections);
  const showRelations = useSpaceStore((s) => s.filters.showRelations);

  const geometry = useMemo(() => {
    if (!showRelations || relations.length === 0) return null;

    const positions: number[] = [];
    const colors: number[] = [];
    const color = new THREE.Color();

    for (const rel of relations) {
      const source = projections.get(rel.sourceId);
      const target = projections.get(rel.targetId);
      if (!source || !target) continue;

      positions.push(source.x, source.y, source.z);
      positions.push(target.x, target.y, target.z);

      const relType = rel.relationType.split(/[\s{(]/)[0];
      color.set(RELATION_COLORS[relType] || '#8B949E');
      colors.push(color.r, color.g, color.b);
      colors.push(color.r, color.g, color.b);
    }

    if (positions.length === 0) return null;

    const geo = new THREE.BufferGeometry();
    geo.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
    geo.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
    return geo;
  }, [relations, projections, showRelations]);

  if (!geometry) return null;

  return (
    <lineSegments geometry={geometry}>
      <lineBasicMaterial vertexColors transparent opacity={0.4} />
    </lineSegments>
  );
}
