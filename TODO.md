# musicionist — follow-up tasks

## Module structure

- [ ] **Move `Degree` to the crate root** (`src/degree.rs`)

  Right now `Degree` lives inside `scales/degree.rs`, but it's a fundamental music theory concept — not a scales-specific one. The `Interval` type already uses `Degree` directly (via `Interval::for_degree()` and `Interval::to_degree()`), which means anyone using intervals has to reach into the `scales` module to get `Degree`. That leaks the module boundary.

  Moving it to the root alongside `note.rs` and `interval.rs` fixes this:
  - Add `pub mod degree;` to `lib.rs`
  - Move `src/scales/degree.rs` → `src/degree.rs`
  - Update `scales/mod.rs`: remove `mod degree; pub use degree::Degree;`, add `use crate::degree::Degree;`
  - Update any internal `use` paths that reference `scales::Degree`

  No logic changes, just path updates. Good `cargo build` + `cargo test` exercise.

## Features / exploration

- [ ] **Chord scale harmonization** — given a key and scale, produce the diatonic chord for each degree (e.g. C major → Cmaj, Dmin, Emin, Fmaj, Gmaj, Amin, Bdim)
- [ ] **Chord output in the CLI** — expose `musicionist chord <KEY> <TYPE>` subcommand
- [ ] **Tetrad harmonization** — extend harmonization to 7th chords
- [ ] **Named interval display** — print intervals by name, not just as `Debug` output
