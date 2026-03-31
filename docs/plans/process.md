# Planning Process

## Purpose

This file keeps the minimum workflow rules for future iteration and remediation planning.

## Plan Shape

- use versioned files under `docs/plans/`
- keep a `summary` file and a `details` file for each active planning track
- use `docs/plans/README.md` as the lookup entry point

## Versioning Rules

- create a new version when scope or phase changes materially
- do not overwrite an old version into an unrelated new phase

## Update Rules

Before implementation:

1. identify the active plan track
2. update the relevant `summary` and `details` files when scope changes
3. update `PLAN.md` if the active plan entry changes

After implementation:

1. update the relevant active plan files
2. update `PLAN.md`
3. update `AGENTS.md` when a durable repository rule changes
4. verify and report GitHub Actions results after push

## Placeholder

This process file has been intentionally reduced and can be expanded again when the planning system
is rewritten.
