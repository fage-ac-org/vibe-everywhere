import type { ProviderKind } from "@/types";

export function preferredProjectProvider(providers: ProviderKind[] | null | undefined): ProviderKind {
  return providers?.[0] ?? "codex";
}
