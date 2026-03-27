import { describe, it, expect, beforeEach } from 'vitest';
import { useSpaceStore } from '../spaceStore';

beforeEach(() => {
  useSpaceStore.setState({
    experiences: [],
    projections: new Map(),
    relations: [],
    attractors: [],
    hoveredId: null,
    selectedId: null,
    loading: false,
    varianceExplained: [],
    embeddingDimension: 0,
    filters: {
      showRelations: true,
      showAttractors: true,
      types: new Set([
        'Generic', 'Solution', 'ErrorPattern', 'Difficulty', 'SuccessPattern',
        'UserPreference', 'ArchitecturalDecision', 'TechInsight', 'Fact',
      ]),
      minImportance: 0,
    },
  });
});

describe('spaceStore', () => {
  it('loads experiences', () => {
    useSpaceStore.getState().setExperiences([
      { id: '1', contentPreview: 'Test', experienceType: 'Generic', importance: 0.5, confidence: 0.8, applications: 0, domain: ['rust'], timestampMs: 1000 },
      { id: '2', contentPreview: 'Solution', experienceType: 'Solution', importance: 0.9, confidence: 0.9, applications: 3, domain: ['rust'], timestampMs: 2000 },
    ]);
    expect(useSpaceStore.getState().experiences).toHaveLength(2);
  });

  it('loads projections as map', () => {
    useSpaceStore.getState().setProjections([
      { id: '1', x: 1.0, y: 2.0, z: 3.0 },
      { id: '2', x: -1.0, y: 0.5, z: -2.0 },
    ]);
    const projections = useSpaceStore.getState().projections;
    expect(projections.size).toBe(2);
    expect(projections.get('1')?.x).toBe(1.0);
    expect(projections.get('2')?.z).toBe(-2.0);
  });

  it('filters by experience type', () => {
    useSpaceStore.getState().setExperiences([
      { id: '1', contentPreview: 'A', experienceType: 'Generic', importance: 0.5, confidence: 0.8, applications: 0, domain: [], timestampMs: 1000 },
      { id: '2', contentPreview: 'B', experienceType: 'Solution', importance: 0.5, confidence: 0.8, applications: 0, domain: [], timestampMs: 1000 },
      { id: '3', contentPreview: 'C', experienceType: 'ErrorPattern', importance: 0.5, confidence: 0.8, applications: 0, domain: [], timestampMs: 1000 },
    ]);

    // All types shown by default
    expect(useSpaceStore.getState().filteredExperiences()).toHaveLength(3);

    // Toggle off Generic
    useSpaceStore.getState().toggleType('Generic');
    expect(useSpaceStore.getState().filteredExperiences()).toHaveLength(2);

    // Toggle off Solution
    useSpaceStore.getState().toggleType('Solution');
    expect(useSpaceStore.getState().filteredExperiences()).toHaveLength(1);
    expect(useSpaceStore.getState().filteredExperiences()[0].id).toBe('3');
  });

  it('filters by minimum importance', () => {
    useSpaceStore.getState().setExperiences([
      { id: '1', contentPreview: 'Low', experienceType: 'Generic', importance: 0.3, confidence: 0.8, applications: 0, domain: [], timestampMs: 1000 },
      { id: '2', contentPreview: 'High', experienceType: 'Generic', importance: 0.8, confidence: 0.8, applications: 0, domain: [], timestampMs: 1000 },
    ]);

    expect(useSpaceStore.getState().filteredExperiences()).toHaveLength(2);

    useSpaceStore.getState().setMinImportance(0.5);
    expect(useSpaceStore.getState().filteredExperiences()).toHaveLength(1);
    expect(useSpaceStore.getState().filteredExperiences()[0].id).toBe('2');
  });

  it('toggles relation visibility', () => {
    expect(useSpaceStore.getState().filters.showRelations).toBe(true);
    useSpaceStore.getState().toggleRelations();
    expect(useSpaceStore.getState().filters.showRelations).toBe(false);
  });

  it('toggles attractor visibility', () => {
    expect(useSpaceStore.getState().filters.showAttractors).toBe(true);
    useSpaceStore.getState().toggleAttractors();
    expect(useSpaceStore.getState().filters.showAttractors).toBe(false);
  });

  it('manages hover and selection', () => {
    useSpaceStore.getState().setHoveredId('exp-1');
    expect(useSpaceStore.getState().hoveredId).toBe('exp-1');

    useSpaceStore.getState().setSelectedId('exp-2');
    expect(useSpaceStore.getState().selectedId).toBe('exp-2');

    useSpaceStore.getState().setSelectedId(null);
    expect(useSpaceStore.getState().selectedId).toBeNull();
  });
});
