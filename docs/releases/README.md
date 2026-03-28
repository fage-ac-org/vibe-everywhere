# Release Notes Workflow

Last updated: 2026-03-28

This directory is the repository-owned source of truth for GitHub Release bodies.

## Files

- `unreleased.md`
  - the working notes for the next release cut
- `vX.Y.Z.md`
  - the frozen note file for a published or ready-to-publish release tag

## Required Workflow

1. When an iteration or remediation changes shipped behavior, release packaging, deployment flow, or
   user onboarding, update `unreleased.md` in the same change set.
2. Before pushing a release tag such as `v0.1.4`, copy the relevant content into
   `docs/releases/v0.1.4.md` and finalize the wording for that specific release.
3. Keep `unreleased.md` as the staging area for the next release after the tag-specific file has
   been created.
4. The release workflow renders the GitHub Release body from the tag-specific file when present, or
   falls back to `unreleased.md` if a tag-specific file has not been created yet.

## Asset Rules

- Published asset names must include the release tag or version.
- Published assets must stay minimal and operator-facing.
- Do not bundle repository README files into release archives.
- Prefer publishing the actual installer or runtime artifact when the user does not need the full
  build directory.
