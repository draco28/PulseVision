# UI/UX Guidelines

**Product:** PulseVision
**Version:** 0.1.0
**Date:** 2026-03-27

---

## 1. Design Principles

| Principle | Description |
|-----------|-------------|
| **Observability-first** | Every pixel serves debugging or understanding. No decorative elements. |
| **Zero-friction** | No onboarding, no accounts, no configuration. Value in <60 seconds. |
| **Information density** | Show maximum data with minimum chrome. Developer tools should be dense. |
| **Real-time feedback** | Animations communicate state changes. Nodes appear, pulse, complete. |
| **Dark-first** | Optimized for 3D visualization contrast and developer preference. |

---

## 2. Color Palette

### Base Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--bg` | `#0D1117` | Canvas/app background |
| `--surface` | `#161B22` | Panels, toolbars |
| `--surface-elevated` | `#21262D` | Detail panels, dropdowns |
| `--border` | `#30363D` | Panel borders, dividers |
| `--text-primary` | `#E6EDF3` | Headings, values |
| `--text-secondary` | `#8B949E` | Labels, descriptions |
| `--accent` | `#58A6FF` | Selected items, links |
| `--success` | `#3FB950` | Completed nodes |
| `--error` | `#F85149` | Error nodes |
| `--warning` | `#D29922` | Warnings |

### ExperienceType Colors

| Type | Hex |
|------|-----|
| Generic | `#4A9EFF` |
| Solution | `#4AFF7F` |
| ErrorPattern | `#FF4A4A` |
| Difficulty | `#FFA94A` |
| SuccessPattern | `#7FFF4A` |
| UserPreference | `#FF4AFF` |
| ArchitecturalDecision | `#4AFFFF` |
| TechInsight | `#FFD700` |
| Fact | `#C0C0C0` |

### Node Type Styles

| Node | Color | Shape |
|------|-------|-------|
| Agent (LLM) | `#58A6FF` | Rounded rectangle |
| Agent (Sequential) | `#30363D` border | Container |
| Agent (Parallel) | `#A371F7` border | Container |
| Agent (Loop) | `#D29922` border | Container |
| LLM Call | `#3FB950` | Circle |
| Tool Call | `#D29922` | Diamond |
| Experience | `#39D2C0` | Star |
| Error | `#F85149` | Octagon |

### Relation Line Styles

| Relation | Color | Style |
|----------|-------|-------|
| RelatedTo | `#8B949E` | 1px solid |
| Supports | `#3FB950` | 1.5px solid + arrow |
| Contradicts | `#F85149` | 1.5px dashed |
| Supersedes | `#58A6FF` | 2px solid + arrow |
| Implies | `#58A6FF` | 1px dotted |
| Elaborates | `#A371F7` | 1.5px solid |

---

## 3. Typography

| Element | Font | Size | Weight |
|---------|------|------|--------|
| Headings | JetBrains Mono, monospace | 14-18px | 600 |
| Body | Inter, sans-serif | 13px | 400 |
| Code/values | JetBrains Mono | 12px | 400 |
| Node labels | Inter | 11px | 500 |
| Stats bar | JetBrains Mono | 14px | 600 |

---

## 4. Layout

```
┌───────────────────────────────────────────────────────────────┐
│ [Toolbar]  ● Connected  |  Agent Flow  |  Substrate Space  |  │
├─────────────────────────────────────────┬─────────────────────┤
│                                         │  Detail Panel       │
│          Canvas (Flow or Space)          │  (on node click)    │
│                                         │                     │
├─────────────────────────────────────────┴─────────────────────┤
│ [Stats Bar]  Tokens: 12,450  |  Time: 34.2s  |  Agents: 4    │
└───────────────────────────────────────────────────────────────┘
```

- Toolbar: 48px height
- Detail panel: 360px width (conditional)
- Stats bar: 32px height

---

## 5. Animation Timings

| Animation | Duration | Easing |
|-----------|----------|--------|
| Node fade-in | 300ms | ease-out |
| Active pulse | 1500ms loop | ease-in-out |
| Edge draw | 200ms | linear |
| Completion checkmark | 200ms | ease-out |
| Error flash | 500ms (2x) | ease-in-out |
| Detail panel slide | 250ms | ease-out |
| 3D sphere pop | 400ms | spring(0.6) |
| Attractor pulse | 2000ms loop | sine |
| Camera zoom-to | 500ms | ease-in-out |

---

## 6. Responsive Behavior

- Min supported: 1280x720
- < 1280px: detail panel overlays canvas
- < 1024px: "Desktop required" message

---

## 7. Accessibility

- Keyboard navigation for panels and filters
- ARIA labels on interactive elements
- 4.5:1 contrast ratio for all text
- `prefers-reduced-motion` disables animations
- 3D canvas is inherently visual — textual data available in detail panels
