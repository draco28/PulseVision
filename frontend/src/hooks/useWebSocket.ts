import { useEffect, useRef, useCallback } from 'react';
import { useEventStore } from '../stores/eventStore';
import { useUiStore } from '../stores/uiStore';
import type { HiveEvent } from '../stores/types';

export function useWebSocket(url: string) {
  const wsRef = useRef<WebSocket | null>(null);
  const retryRef = useRef(0);
  const maxRetryDelay = 30000;

  const processEvent = useEventStore((s) => s.processEvent);
  const setConnectionStatus = useUiStore((s) => s.setConnectionStatus);

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return;

    setConnectionStatus('reconnecting');
    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => {
      retryRef.current = 0;
      setConnectionStatus('connected');
    };

    ws.onmessage = (ev) => {
      try {
        const event: HiveEvent = JSON.parse(ev.data);
        if (event.type) {
          processEvent(event);
        }
      } catch {
        // Ignore non-JSON messages (pings, etc.)
      }
    };

    ws.onclose = () => {
      setConnectionStatus('disconnected');
      wsRef.current = null;

      // Exponential backoff reconnect
      const delay = Math.min(1000 * Math.pow(2, retryRef.current), maxRetryDelay);
      retryRef.current++;
      setTimeout(connect, delay);
    };

    ws.onerror = () => {
      ws.close();
    };
  }, [url, processEvent, setConnectionStatus]);

  useEffect(() => {
    connect();
    return () => {
      wsRef.current?.close();
      wsRef.current = null;
    };
  }, [connect]);
}
