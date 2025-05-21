import { decode } from '@msgpack/msgpack';

type WsEvent = [string, any];

export interface WsMessageHandler<T> {
  (data: T): void;
}

export class WebSocketClient {
  private ws: WebSocket | null = null;
  private messageHandlers: Map<string, WsMessageHandler<any>> = new Map();
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
        this.handleMessage(event);
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
      console.error('Error processing message:', error);
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
