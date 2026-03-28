#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "usage: $0 <release-tag> [assets-dir]" >&2
  exit 1
fi

release_tag="$1"
assets_dir="${2:-}"

tag_notes_path="docs/releases/${release_tag}.md"
fallback_notes_path="docs/releases/unreleased.md"

notes_path=""
if [[ -f "$tag_notes_path" ]]; then
  notes_path="$tag_notes_path"
elif [[ -f "$fallback_notes_path" ]]; then
  notes_path="$fallback_notes_path"
else
  echo "no release notes source found for ${release_tag}" >&2
  exit 1
fi

printf '# Vibe Everywhere %s\n\n' "$release_tag"
printf '_Release notes source: `%s`._\n\n' "$notes_path"
cat "$notes_path"
printf '\n'

if [[ -n "$assets_dir" && -d "$assets_dir" ]]; then
  printf '\n## Release Assets\n\n'
  find "$assets_dir" -maxdepth 1 -type f -printf '%f\n' | sort | while IFS= read -r asset_name; do
    printf -- '- `%s`\n' "$asset_name"
  done
fi
