import type { Edge } from '@xyflow/react';

// ── HiveEvent Types ──────────────────────────────────────────────────

export type HiveEvent =
  | AgentStartedEvent
  | AgentCompletedEvent
  | LlmCallStartedEvent
  | LlmCallCompletedEvent
  | LlmTokenStreamedEvent
  | ToolCallStartedEvent
  | ToolCallCompletedEvent
  | ToolApprovalRequestedEvent
  | ExperienceRecordedEvent
  | RelationshipInferredEvent
  | InsightGeneratedEvent
  | SubstratePerceivedEvent
  | EmbeddingComputedEvent
  | WatchNotificationEvent;

interface BaseEvent {
  timestamp_ms: number;
}

export interface AgentStartedEvent extends BaseEvent {
  type: 'agent_started';
  agent_id: string;
  name: string;
  kind: 'llm' | 'sequential' | 'parallel' | 'loop';
}

export interface AgentCompletedEvent extends BaseEvent {
  type: 'agent_completed';
  agent_id: string;
  outcome: AgentOutcome;
}

export type AgentOutcome =
  | { status: 'complete'; response: string }
  | { status: 'error'; error: string }
  | { status: 'max_iterations_reached' };

export interface LlmCallStartedEvent extends BaseEvent {
  type: 'llm_call_started';
  agent_id: string;
  model: string;
  message_count: number;
}

export interface LlmCallCompletedEvent extends BaseEvent {
  type: 'llm_call_completed';
  agent_id: string;
  model: string;
  duration_ms: number;
  input_tokens: number;
  output_tokens: number;
}

export interface LlmTokenStreamedEvent extends BaseEvent {
  type: 'llm_token_streamed';
  agent_id: string;
  token: string;
}

export interface ToolCallStartedEvent extends BaseEvent {
  type: 'tool_call_started';
  agent_id: string;
  tool_name: string;
  params: string;
}

export interface ToolCallCompletedEvent extends BaseEvent {
  type: 'tool_call_completed';
  agent_id: string;
  tool_name: string;
  duration_ms: number;
  result_preview: string;
}

export interface ToolApprovalRequestedEvent extends BaseEvent {
  type: 'tool_approval_requested';
  agent_id: string;
  tool_name: string;
  description: string;
}

export interface ExperienceRecordedEvent extends BaseEvent {
  type: 'experience_recorded';
  experience_id: string;
  agent_id: string;
  content_preview: string;
  experience_type: string;
  importance: number;
}

export interface RelationshipInferredEvent extends BaseEvent {
  type: 'relationship_inferred';
  relation_id: string;
  agent_id: string;
}

export interface InsightGeneratedEvent extends BaseEvent {
  type: 'insight_generated';
  insight_id: string;
  source_count: number;
  agent_id: string;
}

export interface SubstratePerceivedEvent extends BaseEvent {
  type: 'substrate_perceived';
  agent_id: string;
  experience_count: number;
  insight_count: number;
}

export interface EmbeddingComputedEvent extends BaseEvent {
  type: 'embedding_computed';
  agent_id: string;
  dimensions: number;
  duration_ms: number;
}

export interface WatchNotificationEvent extends BaseEvent {
  type: 'watch_notification';
  experience_id: string;
  collective_id: string;
  event_type: string;
}

// ── Flow Node Data Types ─────────────────────────────────────────────

export type FlowNodeType = 'agent' | 'llmCall' | 'toolCall' | 'experience';

export interface AgentNodeData {
  nodeType: 'agent';
  name: string;
  kind: 'llm' | 'sequential' | 'parallel' | 'loop';
  status: 'running' | 'completed' | 'error';
  outcome?: AgentOutcome;
  startedAt: number;
  completedAt?: number;
}

export interface LlmCallNodeData {
  nodeType: 'llmCall';
  model: string;
  status: 'running' | 'completed';
  durationMs?: number;
  inputTokens?: number;
  outputTokens?: number;
}

export interface ToolCallNodeData {
  nodeType: 'toolCall';
  toolName: string;
  params: string;
  status: 'running' | 'completed';
  durationMs?: number;
  resultPreview?: string;
}

export interface ExperienceNodeData {
  nodeType: 'experience';
  experienceId: string;
  contentPreview: string;
  experienceType: string;
  importance: number;
}

export type FlowNodeData = AgentNodeData | LlmCallNodeData | ToolCallNodeData | ExperienceNodeData;

// React Flow v12 expects Record<string, unknown> for node data.
// We use a wider type for the node and cast in components.
export interface FlowNode {
  id: string;
  type: string;
  position: { x: number; y: number };
  data: FlowNodeData;
  [key: string]: unknown;
}

export type FlowEdge = Edge;

// ── API Response Types ───────────────────────────────────────────────

export interface Collective {
  id: string;
  name: string;
}

export type ConnectionStatus = 'connected' | 'disconnected' | 'reconnecting';
export type ActiveTab = 'flow' | 'space';
