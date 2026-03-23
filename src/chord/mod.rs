pub use chord::Chord;
pub use generator::{augmented, diminished, major, minor, sus2, sus4};
pub use tetrad::Tetrad;
pub use triad::Triad;

#[allow(clippy::module_inception)]
pub mod chord;
mod generator;
pub mod tetrad;
pub mod triad;
