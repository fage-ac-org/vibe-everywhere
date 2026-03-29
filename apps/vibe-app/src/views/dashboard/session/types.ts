import type { TaskDetailResponse, TaskEvent } from "@/types";

export type TranscriptTurn = {
  detail: TaskDetailResponse;
  assistantText: string;
  toolEvents: TaskEvent[];
  systemEvents: TaskEvent[];
  traceEvents: TaskEvent[];
};

export type ConversationListItem = {
  id: string;
  title: string;
  meta: string;
  status: string;
  statusClass: string;
  updatedAt: string;
};
