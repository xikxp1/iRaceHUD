import { decode } from '@msgpack/msgpack';

export interface WsMessageHandler {
  (data: any): void;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private messageHandlers: Map<string, WsMessageHandler> = new Map();
  private isConnected = false;
  private reconnectTimer: number | null = null;

  constructor(private url: string = 'ws://127.0.0.1:8384') {
    this.connect();
  }

  private connect() {
    try {
      this.ws = new WebSocket(this.url);
      this.ws.binaryType = 'arraybuffer';

      this.ws.onopen = () => {
        console.log('WebSocket connected');
        this.isConnected = true;
        if (this.reconnectTimer) {
          clearInterval(this.reconnectTimer);
          this.reconnectTimer = null;
        }
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
        this.isConnected = false;
        this.scheduleReconnect();
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        this.isConnected = false;
      };

      this.ws.onmessage = (event) => {
        try {
          const wsEvent = decode(new Uint8Array(event.data)) as [string, any];
          const handler = this.messageHandlers.get(wsEvent[0]);
          if (handler) {
            handler(wsEvent[1]);
          }
        } catch (error) {
          console.error('Error processing message:', error);
        }
      };
    } catch (error) {
      console.error('Error connecting to WebSocket:', error);
      this.scheduleReconnect();
    }
  }

  private scheduleReconnect() {
    if (!this.reconnectTimer) {
      this.reconnectTimer = window.setInterval(() => {
        if (!this.isConnected) {
          this.connect();
        }
      }, 5000);
    }
  }

  subscribe(event: string, handler: WsMessageHandler) {
    this.messageHandlers.set(event, handler);
  }

  unsubscribe(event: string) {
    this.messageHandlers.delete(event);
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
