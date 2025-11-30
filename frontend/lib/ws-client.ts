import { WEBSOCKET_URL } from "./export";

let socket: WebSocket | null = null;

export function getSocket() {
  if (typeof window == "undefined") return null;

  if (socket && (socket.readyState === WebSocket.OPEN || WebSocket.CONNECTING)) {
    return socket;
  }

  socket = new WebSocket(WEBSOCKET_URL);
  return socket;
}
