import { useCallback, useMemo } from 'react';
import { ReactFlow, Background, Controls, type Node } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import '../../styles/react-flow.css';

import { useEventStore } from '../../stores/eventStore';
import { useUiStore } from '../../stores/uiStore';
import { AgentNode } from './nodes/AgentNode';
import { LlmCallNode } from './nodes/LlmCallNode';
import { ToolCallNode } from './nodes/ToolCallNode';
import { ExperienceNode } from './nodes/ExperienceNode';
import { DetailPanel } from './DetailPanel';

// Node types MUST be defined outside the component
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const nodeTypes: Record<string, any> = {
  agent: AgentNode,
  llmCall: LlmCallNode,
  toolCall: ToolCallNode,
  experience: ExperienceNode,
};

export function FlowCanvas() {
  const flowNodes = useEventStore((s) => s.nodes);
  const edges = useEventStore((s) => s.edges);
  const selectedNodeId = useUiStore((s) => s.selectedNodeId);
  const setSelectedNodeId = useUiStore((s) => s.setSelectedNodeId);

  // Cast to React Flow's Node type
  const nodes = flowNodes as unknown as Node[];

  const onNodeClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      setSelectedNodeId(node.id);
    },
    [setSelectedNodeId]
  );

  const onPaneClick = useCallback(() => {
    setSelectedNodeId(null);
  }, [setSelectedNodeId]);

  const selectedNode = useMemo(
    () => flowNodes.find((n) => n.id === selectedNodeId) || null,
    [flowNodes, selectedNodeId]
  );

  return (
    <div style={{ display: 'flex', width: '100%', height: '100%' }}>
      <div style={{ flex: 1 }}>
        <ReactFlow
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodeClick={onNodeClick}
          onPaneClick={onPaneClick}
          fitView
          proOptions={{ hideAttribution: true }}
          defaultEdgeOptions={{ type: 'smoothstep' }}
        >
          <Background color="var(--border)" gap={20} />
          <Controls
            showInteractive={false}
            style={{ background: 'var(--surface-elevated)', borderColor: 'var(--border)' }}
          />
        </ReactFlow>
      </div>
      {selectedNode && (
        <DetailPanel node={selectedNode} onClose={() => setSelectedNodeId(null)} />
      )}
    </div>
  );
}
