import { memo } from 'react';
import { useSpaceStore } from '../../stores/spaceStore';

const EXPERIENCE_TYPES = [
  { name: 'Generic', color: '#4A9EFF' },
  { name: 'Solution', color: '#4AFF7F' },
  { name: 'ErrorPattern', color: '#FF4A4A' },
  { name: 'Difficulty', color: '#FFA94A' },
  { name: 'SuccessPattern', color: '#7FFF4A' },
  { name: 'UserPreference', color: '#FF4AFF' },
  { name: 'ArchitecturalDecision', color: '#4AFFFF' },
  { name: 'TechInsight', color: '#FFD700' },
  { name: 'Fact', color: '#C0C0C0' },
];

export const FilterPanel = memo(function FilterPanel() {
  const filters = useSpaceStore((s) => s.filters);
  const toggleType = useSpaceStore((s) => s.toggleType);
  const setMinImportance = useSpaceStore((s) => s.setMinImportance);
  const toggleRelations = useSpaceStore((s) => s.toggleRelations);
  const toggleAttractors = useSpaceStore((s) => s.toggleAttractors);
  const experienceCount = useSpaceStore((s) => s.experiences.length);
  const varianceExplained = useSpaceStore((s) => s.varianceExplained);

  return (
    <div
      style={{
        width: 260,
        background: 'var(--surface)',
        borderLeft: '1px solid var(--border)',
        padding: 'var(--panel-padding)',
        overflowY: 'auto',
        flexShrink: 0,
        fontSize: 12,
      }}
    >
      <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 16, fontFamily: 'var(--font-mono)', color: 'var(--text-primary)' }}>
        Filters
      </div>

      {/* Stats */}
      <div style={{ marginBottom: 16, color: 'var(--text-secondary)', fontSize: 11 }}>
        {experienceCount} experiences
        {varianceExplained.length > 0 && (
          <span> | PCA variance: {(varianceExplained.reduce((a, b) => a + b, 0) * 100).toFixed(0)}%</span>
        )}
      </div>

      {/* Experience Types */}
      <SectionLabel>Experience Types</SectionLabel>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginBottom: 16 }}>
        {EXPERIENCE_TYPES.map((t) => (
          <label
            key={t.name}
            style={{ display: 'flex', alignItems: 'center', gap: 8, cursor: 'pointer', color: 'var(--text-primary)' }}
          >
            <input
              type="checkbox"
              checked={filters.types.has(t.name)}
              onChange={() => toggleType(t.name)}
              style={{ accentColor: t.color }}
            />
            <span
              style={{
                width: 8,
                height: 8,
                borderRadius: '50%',
                background: t.color,
                flexShrink: 0,
              }}
            />
            <span style={{ fontSize: 11 }}>{t.name}</span>
          </label>
        ))}
      </div>

      {/* Importance Threshold */}
      <SectionLabel>Min Importance: {filters.minImportance.toFixed(1)}</SectionLabel>
      <input
        type="range"
        min="0"
        max="1"
        step="0.1"
        value={filters.minImportance}
        onChange={(e) => setMinImportance(parseFloat(e.target.value))}
        style={{ width: '100%', marginBottom: 16, accentColor: 'var(--accent)' }}
      />

      {/* Toggles */}
      <SectionLabel>Visibility</SectionLabel>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
        <ToggleRow label="Relations" checked={filters.showRelations} onChange={toggleRelations} />
        <ToggleRow label="Attractors" checked={filters.showAttractors} onChange={toggleAttractors} />
      </div>
    </div>
  );
});

function SectionLabel({ children }: { children: React.ReactNode }) {
  return (
    <div style={{ fontSize: 10, fontWeight: 600, textTransform: 'uppercase', color: 'var(--text-secondary)', marginBottom: 8, letterSpacing: 0.5 }}>
      {children}
    </div>
  );
}

function ToggleRow({ label, checked, onChange }: { label: string; checked: boolean; onChange: () => void }) {
  return (
    <label style={{ display: 'flex', alignItems: 'center', gap: 8, cursor: 'pointer', color: 'var(--text-primary)', fontSize: 12 }}>
      <input type="checkbox" checked={checked} onChange={onChange} style={{ accentColor: 'var(--accent)' }} />
      {label}
    </label>
  );
}
