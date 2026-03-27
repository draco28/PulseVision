import { describe, it, expect, beforeEach } from 'vitest';
import { useEventStore } from '../eventStore';

beforeEach(() => {
  useEventStore.getState().clear();
});

describe('eventStore', () => {
  describe('agent_started', () => {
    it('creates an agent node', () => {
      useEventStore.getState().processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });

      const { nodes, totalAgents } = useEventStore.getState();
      expect(nodes).toHaveLength(1);
      expect(nodes[0].id).toBe('agent-1');
      expect(nodes[0].type).toBe('agent');
      expect(nodes[0].data.nodeType).toBe('agent');
      if (nodes[0].data.nodeType === 'agent') {
        expect(nodes[0].data.name).toBe('explorer');
        expect(nodes[0].data.kind).toBe('llm');
        expect(nodes[0].data.status).toBe('running');
      }
      expect(totalAgents).toBe(1);
    });
  });

  describe('agent_completed', () => {
    it('updates agent node status to completed', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'agent_completed',
        timestamp_ms: 1711500005000,
        agent_id: 'agent-1',
        outcome: { status: 'complete', response: 'Done' },
      });

      const { nodes } = useEventStore.getState();
      expect(nodes).toHaveLength(1);
      if (nodes[0].data.nodeType === 'agent') {
        expect(nodes[0].data.status).toBe('completed');
        expect(nodes[0].data.outcome?.status).toBe('complete');
      }
    });

    it('marks agent as error on error outcome', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'agent_completed',
        timestamp_ms: 1711500005000,
        agent_id: 'agent-1',
        outcome: { status: 'error', error: 'Something failed' },
      });

      const { nodes } = useEventStore.getState();
      if (nodes[0].data.nodeType === 'agent') {
        expect(nodes[0].data.status).toBe('error');
      }
    });
  });

  describe('llm_call', () => {
    it('creates LLM node as child of agent', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'llm_call_started',
        timestamp_ms: 1711500001000,
        agent_id: 'agent-1',
        model: 'GLM-4.7',
        message_count: 3,
      });

      const { nodes, edges } = useEventStore.getState();
      expect(nodes).toHaveLength(2);
      expect(nodes[1].type).toBe('llmCall');
      if (nodes[1].data.nodeType === 'llmCall') {
        expect(nodes[1].data.model).toBe('GLM-4.7');
        expect(nodes[1].data.status).toBe('running');
      }

      // Edge from agent to LLM
      expect(edges).toHaveLength(1);
      expect(edges[0].source).toBe('agent-1');
      expect(edges[0].target).toBe(nodes[1].id);
    });

    it('updates LLM node with tokens on completion', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'llm_call_started',
        timestamp_ms: 1711500001000,
        agent_id: 'agent-1',
        model: 'GLM-4.7',
        message_count: 3,
      });
      store.processEvent({
        type: 'llm_call_completed',
        timestamp_ms: 1711500002000,
        agent_id: 'agent-1',
        model: 'GLM-4.7',
        duration_ms: 1500,
        input_tokens: 200,
        output_tokens: 50,
      });

      const { nodes, totalTokens } = useEventStore.getState();
      const llmNode = nodes.find((n) => n.data.nodeType === 'llmCall');
      expect(llmNode).toBeDefined();
      if (llmNode?.data.nodeType === 'llmCall') {
        expect(llmNode.data.status).toBe('completed');
        expect(llmNode.data.durationMs).toBe(1500);
        expect(llmNode.data.inputTokens).toBe(200);
        expect(llmNode.data.outputTokens).toBe(50);
      }
      expect(totalTokens).toBe(250);
    });
  });

  describe('tool_call', () => {
    it('creates tool node as child of agent', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'tool_call_started',
        timestamp_ms: 1711500001000,
        agent_id: 'agent-1',
        tool_name: 'file_read',
        params: '{"path": "src/main.rs"}',
      });

      const { nodes, edges } = useEventStore.getState();
      expect(nodes).toHaveLength(2);
      expect(nodes[1].type).toBe('toolCall');
      if (nodes[1].data.nodeType === 'toolCall') {
        expect(nodes[1].data.toolName).toBe('file_read');
        expect(nodes[1].data.params).toBe('{"path": "src/main.rs"}');
      }
      expect(edges[0].source).toBe('agent-1');
    });
  });

  describe('experience_recorded', () => {
    it('creates experience node as child of agent', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.processEvent({
        type: 'experience_recorded',
        timestamp_ms: 1711500001000,
        experience_id: 'exp-123',
        agent_id: 'agent-1',
        content_preview: 'Discovered a pattern...',
        experience_type: 'TechInsight',
        importance: 0.8,
      });

      const { nodes } = useEventStore.getState();
      expect(nodes).toHaveLength(2);
      const expNode = nodes.find((n) => n.data.nodeType === 'experience');
      expect(expNode).toBeDefined();
      if (expNode?.data.nodeType === 'experience') {
        expect(expNode.data.experienceId).toBe('exp-123');
        expect(expNode.data.contentPreview).toBe('Discovered a pattern...');
        expect(expNode.data.experienceType).toBe('TechInsight');
        expect(expNode.data.importance).toBe(0.8);
      }
    });
  });

  describe('clear', () => {
    it('resets all state', () => {
      const store = useEventStore.getState();
      store.processEvent({
        type: 'agent_started',
        timestamp_ms: 1711500000000,
        agent_id: 'agent-1',
        name: 'explorer',
        kind: 'llm',
      });
      store.clear();

      const { nodes, edges, events, totalTokens, totalAgents } = useEventStore.getState();
      expect(nodes).toHaveLength(0);
      expect(edges).toHaveLength(0);
      expect(events).toHaveLength(0);
      expect(totalTokens).toBe(0);
      expect(totalAgents).toBe(0);
    });
  });

  describe('multi-agent pipeline', () => {
    it('handles a complete 2-agent flow', () => {
      const store = useEventStore.getState();

      // Agent 1 starts
      store.processEvent({ type: 'agent_started', timestamp_ms: 1000, agent_id: 'a1', name: 'explorer', kind: 'llm' });
      // Agent 1 makes LLM call
      store.processEvent({ type: 'llm_call_started', timestamp_ms: 2000, agent_id: 'a1', model: 'GLM-4.7', message_count: 2 });
      store.processEvent({ type: 'llm_call_completed', timestamp_ms: 3000, agent_id: 'a1', model: 'GLM-4.7', duration_ms: 1000, input_tokens: 100, output_tokens: 50 });
      // Agent 1 uses tool
      store.processEvent({ type: 'tool_call_started', timestamp_ms: 4000, agent_id: 'a1', tool_name: 'search', params: '{}' });
      store.processEvent({ type: 'tool_call_completed', timestamp_ms: 5000, agent_id: 'a1', tool_name: 'search', duration_ms: 500, result_preview: 'Found it' });
      // Agent 1 completes
      store.processEvent({ type: 'agent_completed', timestamp_ms: 6000, agent_id: 'a1', outcome: { status: 'complete', response: 'Done' } });

      // Agent 2 starts
      store.processEvent({ type: 'agent_started', timestamp_ms: 7000, agent_id: 'a2', name: 'planner', kind: 'llm' });
      store.processEvent({ type: 'agent_completed', timestamp_ms: 8000, agent_id: 'a2', outcome: { status: 'complete', response: 'Planned' } });

      const { nodes, edges, totalTokens, totalAgents } = useEventStore.getState();

      // 2 agents + 1 LLM + 1 tool = 4 nodes
      expect(nodes).toHaveLength(4);
      expect(totalAgents).toBe(2);
      expect(totalTokens).toBe(150);

      // 2 edges: agent→llm, agent→tool
      expect(edges).toHaveLength(2);
    });
  });
});
