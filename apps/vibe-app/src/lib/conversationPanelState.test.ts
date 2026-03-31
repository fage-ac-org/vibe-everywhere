import { describe, expect, it } from "vitest";
import {
  buildOptimisticTaskDetail,
  createOptimisticTaskRecord,
  mergeConversationPanelDetail,
  pruneResolvedOptimisticTasks
} from "@/lib/conversationPanelState";
import type { ConversationPanelDetail, ConversationRecord } from "@/types";

function conversationRecord(): ConversationRecord {
  return {
    id: "conversation-1",
    deviceId: "device-1",
    title: "Test conversation",
    provider: "open_code",
    executionProtocol: "acp",
    executionMode: "workspace_write",
    cwd: "/workspace",
    model: "gpt-5.4",
    providerSessionId: null,
    latestTaskId: "task-1",
    pendingInputRequestId: null,
    archived: false,
    createdAtEpochMs: 10,
    updatedAtEpochMs: 20
  };
}

function panelDetail(): ConversationPanelDetail {
  return {
    conversation: conversationRecord(),
    pendingInputRequest: null,
    tasks: [
      buildOptimisticTaskDetail(
        createOptimisticTaskRecord(
          "existing",
          "device-1",
          "open_code",
          "acp",
          "workspace_write",
          "/workspace",
          "gpt-5.4",
          "conversation-1"
        )
      )
    ]
  };
}

describe("conversationPanelState", () => {
  it("merges optimistic tasks after server tasks", () => {
    const detail = panelDetail();
    const optimistic = buildOptimisticTaskDetail(
      createOptimisticTaskRecord(
        "next prompt",
        "device-1",
        "open_code",
        "acp",
        "workspace_write",
        "/workspace",
        "gpt-5.4",
        "conversation-1"
      )
    );

    optimistic.task.createdAtEpochMs = detail.tasks[0].task.createdAtEpochMs + 1000;

    const merged = mergeConversationPanelDetail(detail, [optimistic]);

    expect(merged?.tasks).toHaveLength(2);
    expect(merged?.tasks[1]?.task.prompt).toBe("next prompt");
  });

  it("does not duplicate tasks already present in server detail", () => {
    const detail = panelDetail();
    const merged = mergeConversationPanelDetail(detail, [detail.tasks[0]]);

    expect(merged?.tasks).toHaveLength(1);
  });

  it("prunes optimistic tasks once server detail contains them", () => {
    const detail = panelDetail();
    const remaining = pruneResolvedOptimisticTasks(detail.tasks, detail);

    expect(remaining).toEqual([]);
  });
});
