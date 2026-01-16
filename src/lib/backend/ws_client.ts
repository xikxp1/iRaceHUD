import { decode } from '@msgpack/msgpack';
import { isVrMode, getVrHost } from './vr_mode';

export interface WsMessageHandler<T> {
  (data: T): void;
}

type WsEvent = [string, any];

// Check if running inside Tauri
function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}

// Get WebSocket port via Tauri IPC
async function getWsPortViaTauri(): Promise<number | null> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<number>('get_ws_port');
}

// Get WebSocket port via HTTP API (for VR mode)
async function getWsPortViaHttp(): Promise<number | null> {
  try {
    const host = getVrHost();
    const response = await fetch(`${host}/api/ports`);
    if (response.ok) {
      const data = await response.json();
      return data.ws_port;
    }
  } catch (error) {
    console.error('Failed to fetch WS port via HTTP:', error);
  }
  return null;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private messageHandlers: Map<string, WsMessageHandler<any>> = new Map();
  private isConnected = false;
  private reconnectTimer: number | null = null;
  private port: number | null = null;
  private wsHost: string = '127.0.0.1';

  constructor() {
    this.initialize();
  }

  private async initialize() {
    try {
      if (isTauri()) {
        this.port = await getWsPortViaTauri();
        this.wsHost = '127.0.0.1';
      } else {
        // VR mode - use HTTP API
        this.port = await getWsPortViaHttp();
        // Use the same host as the HTTP server
        this.wsHost = getVrHost().replace(/^https?:\/\//, '').split(':')[0];
      }

      if (this.port) {
        this.connect();
      } else {
        console.error('Failed to get WebSocket port');
        this.scheduleReconnect();
      }
    } catch (error) {
      console.error('Error initializing WebSocket:', error);
      this.scheduleReconnect();
    }
  }

  private connect() {
    if (!this.port) {
      console.error('No WebSocket port available');
      return;
    }

    try {
      this.ws = new WebSocket(`ws://${this.wsHost}:${this.port}`);
      this.ws.binaryType = 'arraybuffer';

      this.ws.onopen = () => {
        console.log(`WebSocket connected to port ${this.port}`);
        this.isConnected = true;
        if (this.reconnectTimer) {
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.isConnected = false;
        this.scheduleReconnect();
      };

      this.ws.onerror = (error) => {
        console.error(`WebSocket error on port ${this.port}:`, error);
        this.isConnected = false;
      };

      this.ws.onmessage = (event) => {
        this.handleMessage(event);
      };
    } catch (error) {
      console.error(`Error connecting to WebSocket on port ${this.port}:`, error);
      this.scheduleReconnect();
    }
  }

  private scheduleReconnect() {
    if (!this.reconnectTimer) {
      this.reconnectTimer = window.setInterval(async () => {
        if (!this.isConnected) {
          await this.initialize();
        }
      }, 5000);
    }
  }

  subscribe<T>(event: string, handler: WsMessageHandler<T>) {
    this.messageHandlers.set(event, handler as WsMessageHandler<any>);
  }

  unsubscribe(event: string) {
    this.messageHandlers.delete(event);
  }

  private handleMessage<T>(event: MessageEvent) {
    try {
      const data = new Uint8Array(event.data);
      const [eventName, eventData] = decode(data) as WsEvent;
      const handler = this.messageHandlers.get(eventName) as WsMessageHandler<T> | undefined;

      if (handler) {
        try {
          const decodedData = eventData as T;
          handler(decodedData);
        } catch (error) {
          console.error(`Error decoding data for ${eventName}: ${error}\nRaw event data:`, eventData);
        }
      }
    } catch (error) {
      console.error(`Error processing message on port ${this.port}:`, error);
    }
  }

  close() {
    if (this.reconnectTimer) {
      clearInterval(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }
}

// Create a singleton instance
export const wsClient = new WebSocketClient();
