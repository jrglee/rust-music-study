# musicionist

A CLI for music theory exploration, written in Rust.

This is a personal project for learning Rust and noodling around with music theory concepts. Not a production tool — just a playground.

## Usage

```
musicionist scale <KEY> <NAME>
```

`KEY` is a note name (`C`, `D#`, `Eb`, `F#`, etc. — sharps and flats both work, case-insensitive).

`NAME` is the scale or mode name.

### Examples

```sh
cargo run -- scale C major
cargo run -- scale A aeolian
cargo run -- scale G harmonic-minor
cargo run -- scale Bb dorian
```

## Supported scales

### Diatonic modes

| Name | Aliases |
|------|---------|
| `major` | `ionian` |
| `dorian` | |
| `phrygian` | |
| `lydian` | |
| `mixolydian` | |
| `minor` | `aeolian` |
| `locrian` | |

### Harmonic minor modes

| Name | Aliases |
|------|---------|
| `harmonic-minor` | `harmonic minor` |
| `locrian-maj6` | `locrian major6`, `locrian maj6` |
| `ionian-aug5` | `ionian #5`, `ionian aug5` |
| `dorian-lydian` | `dorian #4` |
| `phrygian-dominant` | `phrygian maj3`, `phrygian dominant` |
| `lydian-aug2` | `lydian #2`, `lydian aug2` |
| `superlocrian` | `super locrian`, `super-locrian` |

## Setup

See [DEVELOPMENT.md](DEVELOPMENT.md) for build and toolchain setup.
