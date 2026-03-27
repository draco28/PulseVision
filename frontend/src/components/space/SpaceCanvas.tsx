import { Canvas } from '@react-three/fiber';
import { OrbitControls } from '@react-three/drei';
import { EffectComposer, Bloom } from '@react-three/postprocessing';

import { useSubstrate } from '../../hooks/useSubstrate';
import { useSpaceStore } from '../../stores/spaceStore';
import { ExperienceCloud } from './ExperienceCloud';
import { RelationLines } from './RelationLines';
import { AttractorFields } from './AttractorFields';
import { HoverLabel } from './HoverLabel';
import { SpaceDetailPanel } from './SpaceDetailPanel';

export function SpaceCanvas() {
  useSubstrate();

  const loading = useSpaceStore((s) => s.loading);
  const selectedId = useSpaceStore((s) => s.selectedId);
  const experienceCount = useSpaceStore((s) => s.experiences.length);

  return (
    <div style={{ display: 'flex', width: '100%', height: '100%' }}>
      <div style={{ flex: 1, position: 'relative' }}>
        {loading && (
          <div
            style={{
              position: 'absolute',
              top: 16,
              left: '50%',
              transform: 'translateX(-50%)',
              zIndex: 10,
              background: 'var(--surface-elevated)',
              border: '1px solid var(--border)',
              borderRadius: 'var(--radius-panel)',
              padding: '8px 16px',
              fontSize: 12,
              color: 'var(--text-secondary)',
              fontFamily: 'var(--font-mono)',
            }}
          >
            Loading substrate...
          </div>
        )}

        {!loading && experienceCount === 0 && (
          <div
            style={{
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              height: '100%',
              color: 'var(--text-secondary)',
              fontFamily: 'var(--font-mono)',
            }}
          >
            No experiences in substrate
          </div>
        )}

        {experienceCount > 0 && (
          <Canvas camera={{ position: [0, 0, 30], fov: 60 }}>
            <color attach="background" args={['#0D1117']} />
            <ambientLight intensity={0.3} />
            <pointLight position={[10, 10, 10]} intensity={0.8} />
            <pointLight position={[-10, -10, -10]} intensity={0.3} color="#4A9EFF" />

            <ExperienceCloud />
            <RelationLines />
            <AttractorFields />
            <HoverLabel />

            <OrbitControls
              enableDamping
              dampingFactor={0.1}
              rotateSpeed={0.5}
              zoomSpeed={1.2}
              panSpeed={0.8}
              minDistance={5}
              maxDistance={200}
            />

            <EffectComposer>
              <Bloom
                intensity={0.4}
                luminanceThreshold={0.8}
                luminanceSmoothing={0.9}
              />
            </EffectComposer>
          </Canvas>
        )}
      </div>

      {selectedId && <SpaceDetailPanel />}
    </div>
  );
}
