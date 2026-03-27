import { Html } from '@react-three/drei';
import { useSpaceStore } from '../../stores/spaceStore';

export function HoverLabel() {
  const hoveredId = useSpaceStore((s) => s.hoveredId);
  const experiences = useSpaceStore((s) => s.experiences);
  const projections = useSpaceStore((s) => s.projections);

  if (!hoveredId) return null;

  const exp = experiences.find((e) => e.id === hoveredId);
  const proj = projections.get(hoveredId);
  if (!exp || !proj) return null;

  return (
    <Html position={[proj.x * 10, proj.y * 10 + 1, proj.z * 10]} center style={{ pointerEvents: 'none' }}>
      <div
        style={{
          background: 'var(--surface-elevated)',
          border: '1px solid var(--border)',
          borderRadius: 4,
          padding: '4px 8px',
          fontSize: 11,
          color: 'var(--text-primary)',
          fontFamily: 'var(--font-sans)',
          whiteSpace: 'nowrap',
          maxWidth: 200,
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }}
      >
        {exp.contentPreview.slice(0, 40)}
      </div>
    </Html>
  );
}
