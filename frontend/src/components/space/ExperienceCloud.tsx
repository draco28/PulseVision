import { useRef, useMemo, useEffect } from 'react';
import * as THREE from 'three';
import { useSpaceStore } from '../../stores/spaceStore';

const EXPERIENCE_COLORS: Record<string, string> = {
  Generic: '#4A9EFF',
  Solution: '#4AFF7F',
  ErrorPattern: '#FF4A4A',
  Difficulty: '#FFA94A',
  SuccessPattern: '#7FFF4A',
  UserPreference: '#FF4AFF',
  ArchitecturalDecision: '#4AFFFF',
  TechInsight: '#FFD700',
  Fact: '#C0C0C0',
};

function getTypeColor(typeStr: string): string {
  const typeName = typeStr.split(/[\s{(]/)[0];
  return EXPERIENCE_COLORS[typeName] || '#4A9EFF';
}

export function ExperienceCloud() {
  const meshRef = useRef<THREE.InstancedMesh>(null);
  const tempObject = useMemo(() => new THREE.Object3D(), []);
  const tempColor = useMemo(() => new THREE.Color(), []);

  // Use raw experiences + filters to avoid creating new array every render
  const allExperiences = useSpaceStore((s) => s.experiences);
  const filters = useSpaceStore((s) => s.filters);
  const projections = useSpaceStore((s) => s.projections);

  const experiences = useMemo(() => {
    return allExperiences.filter((exp) => {
      const typeName = exp.experienceType.split(/[\s{(]/)[0];
      if (!filters.types.has(typeName)) return false;
      if (exp.importance < filters.minImportance) return false;
      return true;
    });
  }, [allExperiences, filters]);
  const setHoveredId = useSpaceStore((s) => s.setHoveredId);
  const setSelectedId = useSpaceStore((s) => s.setSelectedId);

  // Map from instance index → experience id
  const indexToId = useMemo(() => {
    const map = new Map<number, string>();
    let idx = 0;
    for (const exp of experiences) {
      if (projections.has(exp.id)) {
        map.set(idx, exp.id);
        idx++;
      }
    }
    return map;
  }, [experiences, projections]);

  const visibleCount = indexToId.size;

  useEffect(() => {
    if (!meshRef.current || visibleCount === 0) return;
    const mesh = meshRef.current;

    let idx = 0;
    for (const exp of experiences) {
      const proj = projections.get(exp.id);
      if (!proj) continue;

      // Scale up PCA coordinates (typically -1 to 1) to fill 3D space
      const SCALE = 10;
      tempObject.position.set(proj.x * SCALE, proj.y * SCALE, proj.z * SCALE);
      const scale = 0.15 + exp.importance * 0.35;
      tempObject.scale.setScalar(scale);
      tempObject.updateMatrix();
      mesh.setMatrixAt(idx, tempObject.matrix);

      tempColor.set(getTypeColor(exp.experienceType));
      mesh.setColorAt(idx, tempColor);

      idx++;
    }

    mesh.instanceMatrix.needsUpdate = true;
    if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
    mesh.count = visibleCount;
  }, [experiences, projections, visibleCount, tempObject, tempColor]);

  if (visibleCount === 0) return null;

  return (
    <instancedMesh
      ref={meshRef}
      args={[undefined, undefined, Math.max(visibleCount, 1)]}
      onPointerOver={(e) => {
        e.stopPropagation();
        if (e.instanceId != null) {
          const id = indexToId.get(e.instanceId);
          if (id) setHoveredId(id);
        }
      }}
      onPointerOut={() => setHoveredId(null)}
      onClick={(e) => {
        e.stopPropagation();
        if (e.instanceId != null) {
          const id = indexToId.get(e.instanceId);
          if (id) setSelectedId(id);
        }
      }}
    >
      <sphereGeometry args={[1, 16, 16]} />
      <meshStandardMaterial vertexColors transparent opacity={0.9} />
    </instancedMesh>
  );
}
