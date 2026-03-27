import { create } from 'zustand';
import dagre from '@dagrejs/dagre';
import type { FlowNode, FlowEdge, FlowNodeData, HiveEvent } from './types';

interface EventStoreState {
  nodes: FlowNode[];
  edges: FlowEdge[];
  events: HiveEvent[];
  agentIds: Set<string>;
  totalTokens: number;
  totalAgents: number;

  processEvent: (event: HiveEvent) => void;
  clear: () => void;
}

let layoutTimeout: ReturnType<typeof setTimeout> | null = null;

function layoutGraph(nodes: FlowNode[], edges: FlowEdge[]): FlowNode[] {
  if (nodes.length === 0) return nodes;

  const g = new dagre.graphlib.Graph();
  g.setDefaultEdgeLabel(() => ({}));
  g.setGraph({ rankdir: 'LR', nodesep: 50, ranksep: 80 });

  nodes.forEach((node) => {
    const width = node.type === 'agent' ? 180 : 140;
    const height = node.type === 'agent' ? 60 : 50;
    g.setNode(node.id, { width, height });
  });

  edges.forEach((edge) => {
    g.setEdge(edge.source, edge.target);
  });

  dagre.layout(g);

  return nodes.map((node) => {
    const pos = g.node(node.id);
    if (!pos) return node;
    const width = node.type === 'agent' ? 180 : 140;
    const height = node.type === 'agent' ? 60 : 50;
    return {
      ...node,
      position: { x: pos.x - width / 2, y: pos.y - height / 2 },
    };
  });
}

export const useEventStore = create<EventStoreState>((set, get) => ({
  nodes: [],
  edges: [],
  events: [],
  agentIds: new Set(),
  totalTokens: 0,
  totalAgents: 0,

  processEvent: (event: HiveEvent) => {
    const state = get();
    const newNodes = [...state.nodes];
    const newEdges = [...state.edges];
    const newEvents = [...state.events, event];
    let totalTokens = state.totalTokens;
    let totalAgents = state.totalAgents;
    const agentIds = new Set(state.agentIds);

    switch (event.type) {
      case 'agent_started': {
        agentIds.add(event.agent_id);
        totalAgents = agentIds.size;
        const node: FlowNode = {
          id: event.agent_id,
          type: 'agent',
          position: { x: 0, y: 0 },
          data: {
            nodeType: 'agent' as const,
            name: event.name,
            kind: event.kind,
            status: 'running' as const,
            startedAt: event.timestamp_ms,
          },
        };
        newNodes.push(node);
        break;
      }

      case 'agent_completed': {
        const idx = newNodes.findIndex((n) => n.id === event.agent_id);
        if (idx !== -1) {
          const data = newNodes[idx].data as FlowNodeData;
          if (data.nodeType === 'agent') {
            newNodes[idx] = {
              ...newNodes[idx],
              data: {
                ...data,
                status: event.outcome.status === 'complete' ? 'completed' : 'error',
                outcome: event.outcome,
                completedAt: event.timestamp_ms,
              },
            };
          }
        }
        break;
      }

      case 'llm_call_started': {
        const nodeId = `llm-${event.agent_id}-${event.timestamp_ms}`;
        const node: FlowNode = {
          id: nodeId,
          type: 'llmCall',
          position: { x: 0, y: 0 },
          data: {
            nodeType: 'llmCall' as const,
            model: event.model,
            status: 'running' as const,
          },
        };
        newNodes.push(node);
        newEdges.push({
          id: `e-${event.agent_id}-${nodeId}`,
          source: event.agent_id,
          target: nodeId,
          type: 'smoothstep',
        });
        break;
      }

      case 'llm_call_completed': {
        // Find the matching llm node (most recent for this agent)
        const llmIdx = newNodes.findLastIndex(
          (n) => n.data.nodeType === 'llmCall' && n.id.startsWith(`llm-${event.agent_id}-`)
        );
        if (llmIdx !== -1) {
          const data = newNodes[llmIdx].data as FlowNodeData;
          if (data.nodeType === 'llmCall') {
            newNodes[llmIdx] = {
              ...newNodes[llmIdx],
              data: {
                ...data,
                status: 'completed' as const,
                durationMs: event.duration_ms,
                inputTokens: event.input_tokens,
                outputTokens: event.output_tokens,
              },
            };
          }
        }
        totalTokens += (event.input_tokens || 0) + (event.output_tokens || 0);
        break;
      }

      case 'tool_call_started': {
        const nodeId = `tool-${event.agent_id}-${event.timestamp_ms}`;
        const node: FlowNode = {
          id: nodeId,
          type: 'toolCall',
          position: { x: 0, y: 0 },
          data: {
            nodeType: 'toolCall' as const,
            toolName: event.tool_name,
            params: event.params,
            status: 'running' as const,
          },
        };
        newNodes.push(node);
        newEdges.push({
          id: `e-${event.agent_id}-${nodeId}`,
          source: event.agent_id,
          target: nodeId,
          type: 'smoothstep',
        });
        break;
      }

      case 'tool_call_completed': {
        const toolIdx = newNodes.findLastIndex(
          (n) => n.data.nodeType === 'toolCall' && n.id.startsWith(`tool-${event.agent_id}-`)
        );
        if (toolIdx !== -1) {
          const data = newNodes[toolIdx].data as FlowNodeData;
          if (data.nodeType === 'toolCall') {
            newNodes[toolIdx] = {
              ...newNodes[toolIdx],
              data: {
                ...data,
                status: 'completed' as const,
                durationMs: event.duration_ms,
                resultPreview: event.result_preview,
              },
            };
          }
        }
        break;
      }

      case 'experience_recorded': {
        const nodeId = `exp-${event.experience_id}`;
        const node: FlowNode = {
          id: nodeId,
          type: 'experience',
          position: { x: 0, y: 0 },
          data: {
            nodeType: 'experience' as const,
            experienceId: event.experience_id,
            contentPreview: event.content_preview,
            experienceType: event.experience_type,
            importance: event.importance,
          },
        };
        newNodes.push(node);
        newEdges.push({
          id: `e-${event.agent_id}-${nodeId}`,
          source: event.agent_id,
          target: nodeId,
          type: 'smoothstep',
        });
        break;
      }

      default:
        // Other event types don't create nodes
        break;
    }

    // Debounced layout
    if (layoutTimeout) clearTimeout(layoutTimeout);
    layoutTimeout = setTimeout(() => {
      const current = get();
      const laid = layoutGraph(current.nodes, current.edges);
      set({ nodes: laid });
    }, 100);

    set({
      nodes: newNodes,
      edges: newEdges,
      events: newEvents,
      agentIds,
      totalTokens,
      totalAgents,
    });
  },

  clear: () =>
    set({
      nodes: [],
      edges: [],
      events: [],
      agentIds: new Set(),
      totalTokens: 0,
      totalAgents: 0,
    }),
}));
