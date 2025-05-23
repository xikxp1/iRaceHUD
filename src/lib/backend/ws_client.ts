import { invoke } from '@tauri-apps/api/core';
import { decode } from '@msgpack/msgpack';

export interface WsMessageHandler<T> {
  (data: T): void;
}

type WsEvent = [string, any];

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private messageHandlers: Map<string, WsMessageHandler<any>> = new Map();
  private isConnected = false;
  private reconnectTimer: number | null = null;
  private port: number | null = null;

  constructor() {
    this.initialize();
  }

  private async initialize() {
    try {
      this.port = await invoke<number>('get_ws_port');
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
      this.ws = new WebSocket(`ws://127.0.0.1:${this.port}`);
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
