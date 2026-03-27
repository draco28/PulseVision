import { useRef } from 'react';
import { useFrame } from '@react-three/fiber';
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

function AttractorSphere({ attractor }: { attractor: ReturnType<typeof useSpaceStore.getState>['attractors'][0] }) {
  const meshRef = useRef<THREE.Mesh>(null);
  const materialRef = useRef<THREE.MeshStandardMaterial>(null);

  const typeName = attractor.experienceType.split(/[\s{(]/)[0];
  const color = EXPERIENCE_COLORS[typeName] || '#4A9EFF';

  useFrame((state) => {
    if (materialRef.current) {
      const t = state.clock.elapsedTime;
      const pulse = attractor.strength * (1 + 0.3 * Math.sin(t * Math.PI));
      materialRef.current.emissiveIntensity = pulse;
    }
  });

  return (
    <group position={[attractor.position.x, attractor.position.y, attractor.position.z]}>
      {/* Glow sphere */}
      <mesh ref={meshRef}>
        <sphereGeometry args={[0.8 + attractor.strength * 0.5, 16, 16]} />
        <meshStandardMaterial
          ref={materialRef}
          color={color}
          emissive={color}
          emissiveIntensity={attractor.strength}
          transparent
          opacity={0.8}
        />
      </mesh>

      {/* Influence radius (wireframe) */}
      <mesh>
        <sphereGeometry args={[attractor.influenceRadius, 16, 16]} />
        <meshBasicMaterial
          color={color}
          transparent
          opacity={0.05}
          wireframe
        />
      </mesh>
    </group>
  );
}

export function AttractorFields() {
  const attractors = useSpaceStore((s) => s.attractors);
  const showAttractors = useSpaceStore((s) => s.filters.showAttractors);

  if (!showAttractors || attractors.length === 0) return null;

  return (
    <>
      {attractors.map((a) => (
        <AttractorSphere key={a.experienceId} attractor={a} />
      ))}
    </>
  );
}
