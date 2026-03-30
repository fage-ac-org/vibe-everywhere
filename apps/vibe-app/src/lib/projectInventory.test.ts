import { describe, expect, it } from "vitest";
import {
  clearRediscoveredHiddenProjectKeys,
  filterVisibleProjectKeys,
  removeSuppressedProjectKey,
  suppressProjectKey
} from "@/lib/projectInventory";

describe("project inventory suppression", () => {
  it("suppresses a project key only once", () => {
    expect(suppressProjectKey([], "device::/repo/current")).toEqual(["device::/repo/current"]);
    expect(suppressProjectKey(["device::/repo/current"], "device::/repo/current")).toEqual([
      "device::/repo/current"
    ]);
  });

  it("removes rediscovered keys from suppression", () => {
    expect(
      clearRediscoveredHiddenProjectKeys(
        ["device::/repo/current", "device::/repo/old"],
        ["device::/repo/current"]
      )
    ).toEqual(["device::/repo/old"]);
  });

  it("filters hidden projects from visible summaries", () => {
    expect(
      filterVisibleProjectKeys(
        [
          { key: "device::/repo/current", label: "current" },
          { key: "device::/repo/archive", label: "archive" }
        ],
        ["device::/repo/archive"]
      )
    ).toEqual([{ key: "device::/repo/current", label: "current" }]);
  });

  it("can remove a suppressed key explicitly", () => {
    expect(removeSuppressedProjectKey(["a", "b"], "a")).toEqual(["b"]);
  });
});
