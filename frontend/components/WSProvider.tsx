"use client";

import { ReactNode, useEffect, useRef } from "react";
import { getSocket } from "../lib/ws-client";

export default function WSProvider({ children }: { children: ReactNode }) {
  const wsRef = useRef<WebSocket | null>(null);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    const ws = getSocket();
    if (!ws) return;
    wsRef.current = ws;

    const sendRefresh = () => {
      if (wsRef.current?.readyState === WebSocket.OPEN) {
        wsRef.current.send("Refresh Token");
      }
    };

    const startInterval = () => {
      if (intervalRef.current) return;

      intervalRef.current = setInterval(
        () => {
          sendRefresh();
        },
        4 * 60 * 1000,
      );
    };

    const handleOpen = () => {
      sendRefresh();
      startInterval();
    };

    const handleClose = () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };

    ws.addEventListener("open", handleOpen);
    ws.addEventListener("close", handleClose);

    if (ws.readyState === WebSocket.OPEN) {
      handleOpen();
    }

    return () => {
      ws.removeEventListener("open", handleOpen);
      ws.removeEventListener("close", handleClose);

      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, []);

  return children;
}
