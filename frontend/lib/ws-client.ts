import { WEBSOCKET_URL } from "./export";
let socket: WebSocket | null = null;

export function getSocket(token: string | null) {
  if (typeof window == "undefined") return null;

  if (socket && (socket.readyState === WebSocket.OPEN || WebSocket.CONNECTING)) {
    return socket;
  }

  if (!token) {
    console.warn("No auth_token found in cookies");
    return null;
  }

  const url = new URL(WEBSOCKET_URL);
  url.searchParams.append("token", token);

  socket = new WebSocket(url.toString());

  return socket;
}
