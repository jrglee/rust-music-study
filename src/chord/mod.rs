pub use chord::Chord;
pub use tetrad::Tetrad;
pub use triad::Triad;
pub use generator::{augmented, diminished, major, minor, sus2, sus4};

pub mod chord;
pub mod tetrad;
pub mod triad;
mod generator;
