import { create } from 'zustand';

export interface SubstrateExperience {
  id: string;
  contentPreview: string;
  experienceType: string;
  importance: number;
  confidence: number;
  applications: number;
  domain: string[];
  timestampMs: number;
}

export interface Projection {
  id: string;
  x: number;
  y: number;
  z: number;
}

export interface SubstrateRelation {
  id: string;
  sourceId: string;
  targetId: string;
  relationType: string;
  strength: number;
}

export interface SubstrateAttractor {
  experienceId: string;
  position: { x: number; y: number; z: number };
  strength: number;
  influenceRadius: number;
  warpFactor: number;
  experienceType: string;
}

export interface SpaceFilters {
  showRelations: boolean;
  showAttractors: boolean;
  types: Set<string>;
  minImportance: number;
}

interface SpaceStoreState {
  experiences: SubstrateExperience[];
  projections: Map<string, Projection>;
  relations: SubstrateRelation[];
  attractors: SubstrateAttractor[];
  hoveredId: string | null;
  selectedId: string | null;
  filters: SpaceFilters;
  loading: boolean;
  varianceExplained: number[];
  embeddingDimension: number;

  setExperiences: (experiences: SubstrateExperience[]) => void;
  setProjections: (projections: Projection[]) => void;
  setRelations: (relations: SubstrateRelation[]) => void;
  setAttractors: (attractors: SubstrateAttractor[]) => void;
  setHoveredId: (id: string | null) => void;
  setSelectedId: (id: string | null) => void;
  setLoading: (loading: boolean) => void;
  setVarianceExplained: (v: number[]) => void;
  setEmbeddingDimension: (d: number) => void;
  toggleType: (type: string) => void;
  setMinImportance: (v: number) => void;
  toggleRelations: () => void;
  toggleAttractors: () => void;

  // Computed
  filteredExperiences: () => SubstrateExperience[];
}

const ALL_TYPES = new Set([
  'Generic', 'Solution', 'ErrorPattern', 'Difficulty', 'SuccessPattern',
  'UserPreference', 'ArchitecturalDecision', 'TechInsight', 'Fact',
]);

export const useSpaceStore = create<SpaceStoreState>((set, get) => ({
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
    types: new Set(ALL_TYPES),
    minImportance: 0,
  },

  setExperiences: (experiences) => set({ experiences }),
  setProjections: (projections) => {
    const map = new Map<string, Projection>();
    for (const p of projections) map.set(p.id, p);
    set({ projections: map });
  },
  setRelations: (relations) => set({ relations }),
  setAttractors: (attractors) => set({ attractors }),
  setHoveredId: (hoveredId) => set({ hoveredId }),
  setSelectedId: (selectedId) => set({ selectedId }),
  setLoading: (loading) => set({ loading }),
  setVarianceExplained: (varianceExplained) => set({ varianceExplained }),
  setEmbeddingDimension: (embeddingDimension) => set({ embeddingDimension }),

  toggleType: (type) => set((state) => {
    const types = new Set(state.filters.types);
    if (types.has(type)) types.delete(type);
    else types.add(type);
    return { filters: { ...state.filters, types } };
  }),
  setMinImportance: (minImportance) => set((state) => ({
    filters: { ...state.filters, minImportance },
  })),
  toggleRelations: () => set((state) => ({
    filters: { ...state.filters, showRelations: !state.filters.showRelations },
  })),
  toggleAttractors: () => set((state) => ({
    filters: { ...state.filters, showAttractors: !state.filters.showAttractors },
  })),

  filteredExperiences: () => {
    const { experiences, filters } = get();
    return experiences.filter((exp) => {
      // Extract type name from debug string (e.g., "Solution { ... }" → "Solution")
      const typeName = exp.experienceType.split(/[\s{(]/)[0];
      if (!filters.types.has(typeName)) return false;
      if (exp.importance < filters.minImportance) return false;
      return true;
    });
  },
}));
