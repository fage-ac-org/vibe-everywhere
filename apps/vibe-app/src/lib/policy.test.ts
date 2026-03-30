import { describe, expect, it } from "vitest";
import { preferredProjectProvider } from "@/lib/policy";

describe("preferredProjectProvider", () => {
  it("returns the first discovered provider", () => {
    expect(preferredProjectProvider(["open_code", "codex"])).toBe("open_code");
  });

  it("falls back to codex when no provider is available", () => {
    expect(preferredProjectProvider(undefined)).toBe("codex");
  });
});
