use triad::Triad;

use crate::note::Note;

pub mod tetrad;
pub mod triad;

pub fn major(root: &Note) -> [Note; 3] {
    Triad::Major.intervals().map(|int| int.apply_to_note(root))
}

pub fn minor(root: &Note) -> [Note; 3] {
    Triad::Minor.intervals().map(|int| int.apply_to_note(root))
}

#[cfg(test)]
mod tests {
    use Note::*;

    use super::*;

    #[test]
    fn major_chord() {
        assert_eq!(major(&C), [C, E, G]);
        assert_eq!(major(&G), [G, B, D]);
    }

    #[test]
    fn minor_chord() {
        assert_eq!(minor(&C), [C, Eb, G]);
        assert_eq!(minor(&A), [A, C, E]);
    }
}