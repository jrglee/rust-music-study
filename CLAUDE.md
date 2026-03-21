# CLAUDE.md

## Project

`musicionist` is a CLI tool for music theory exploration built in Rust. It currently supports scale generation (diatonic modes, harmonic minor and its modes) and has a foundation for chord work. Usage: `musicionist scale <KEY> <NAME>`.

## Context

This is a personal project used to learn Rust and explore music theory. It is not published or used by anyone else. The goal is play and exploration — there is no production concern.

## Working style

- Handle the gruntwork (boilerplate, wiring, test scaffolding) so the user can focus on the interesting parts.
- Do not worry about breaking existing features — tests exist for that.
- Keep solutions simple. This is a learning playground, not a production system.

## Process

- Every change must be fully planned before execution. Use plan mode to think through the complete approach first — this minimizes half-executed changes that are hard to debug. Verify by running `cargo build` and `cargo test` after implementation.
- For exploratory work or tasks with multiple viable approaches, use a git worktree and spawn subagents to try different approaches. Worktree exploration produces proposals only — nothing is merged back automatically, and it is valid to throw away the results entirely.

## Code review

- Feedback should be educational, not production-hardening. Explain *why* something works or could be improved, not just *what* to change.
- Prioritize helping the user build understanding over enforcing best practices.
