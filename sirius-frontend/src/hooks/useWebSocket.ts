import { useState, useEffect, useRef, useCallback } from 'react';
import type { Message, UserJoinPayload, KanbanBoardPayload } from '../types/types';
import { KanbanBoardOp } from '../types/types';

export const useSiriusSocket = (url: string, user: UserJoinPayload) => {
    const [tasks, setTasks] = useState<Record<string, any>>({});
    const [columns, setColumns] = useState<Record<string, any>>({});
    const ws = useRef<WebSocket | null>(null);

    useEffect(() => {
        ws.current = new WebSocket(url);

        ws.current.onopen = () => {
            console.log('Connected to Sirius Backend');
            // Automatically join the room upon connection
            if (ws.current) {
                ws.current.send(JSON.stringify({ ...user, type: 'JOIN_ROOM' }));
            }
        };

        ws.current.onmessage = (event) => {
            const payload: KanbanBoardPayload = JSON.parse(event.data);

            // Handle different operation types from the backend
            switch (payload.type) {
                case KanbanBoardOp.MoveTask:
                    console.log("Moved Task");
                    break;
                case KanbanBoardOp.CreateTask:
                    console.log("Created Task");
                    break;
                case KanbanBoardOp.DeleteTask:
                    console.log("Deleted Task");
                    break;
                case KanbanBoardOp.UpdateTask:
                    console.log("Updated Task");
                    break;
                case KanbanBoardOp.ChatMessage:
                    console.log("Chat Message");
                    break;
                default:
                    break;
            }
        };
        return () => ws.current?.close();
    }, [url, user]);

    const sendOperation = useCallback((op: KanbanBoardPayload) => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            ws.current.send(JSON.stringify(op));
        }
    }, []);

    return { tasks, columns, sendOperation };
};