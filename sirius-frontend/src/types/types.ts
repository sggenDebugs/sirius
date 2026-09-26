const UserRole = {
    Viewer: 0,
    Editor: 1,
    Admin: 2
} as const;
type UserRoleType = typeof UserRole[keyof typeof UserRole];

export const KanbanBoardOp = {
    CreateTask: 0,
    MoveTask: 1,
    UpdateTask: 2,
    DeleteTask: 3,
    ChatMessage: 4
} as const;

type KanbanBoardOpType = typeof KanbanBoardOp[keyof typeof KanbanBoardOp];

export interface UserJoinPayload {
    userId: string;       // UUID
    username: string;     // User's display name
    roomId: string;       // The specific Kanban board the user is joining
    role?: UserRoleType; // Optional: Permission level for the session
    timestamp: number;    // Time of join for sorting or logging
}

export interface KanbanBoardPayload {
    roomId: string;       // Ensures the message is routed to the correct board ID
    type: KanbanBoardOpType; // Discriminator for the action
    taskId?: string;      // ID of the task being affected
    columnId?: string;    // ID of the source or destination column
    content?: string;     // The actual data (e.g., task title, description, or chat text)
    metadata?: any;       // Optional: For extra data like priority, tags, or assignee
    userId: string;       // Who performed the action (for audit logs and presence)
    timestamp: number;    // When the action occurred
}

export interface Message {
  id: string;
  text: string;
  sender: string;
  timestamp: number;
}
