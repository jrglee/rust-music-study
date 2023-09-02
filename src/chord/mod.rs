use triad::Triad;

use crate::note::Note;

pub mod tetrad;
pub mod triad;

pub fn major(root: Note) -> [Note; 3] {
    triad_notes(Triad::Major, root)
}

pub fn minor(root: Note) -> [Note; 3] {
    triad_notes(Triad::Minor, root)
}

pub fn augmented(root: Note) -> [Note; 3] {
    triad_notes(Triad::Augmented, root)
}

pub fn diminished(root: Note) -> [Note; 3] {
    triad_notes(Triad::Diminished, root)
}

pub fn sus2(root: Note) -> [Note; 3] {
    triad_notes(Triad::Sus2, root)
}

pub fn sus4(root: Note) -> [Note; 3] {
    triad_notes(Triad::Sus4, root)
}

fn triad_notes(triad: Triad, root: Note) -> [Note; 3] {
    triad.intervals().map(|int| int.apply_to_note(root))
}

#[cfg(test)]
mod tests {
    use Note::*;

    use super::*;

    #[test]
    fn major_chord() {
        assert_eq!(major(C), [C, E, G]);
        assert_eq!(major(G), [G, B, D]);
    }

    #[test]
    fn minor_chord() {
        assert_eq!(minor(C), [C, Eb, G]);
        assert_eq!(minor(A), [A, C, E]);
    }

    #[test]
    fn augmented_chord() {
        assert_eq!(augmented(C), [C, E, Ab]);
        assert_eq!(augmented(G), [G, B, Eb]);
    }

    #[test]
    fn diminished_chord() {
        assert_eq!(diminished(C), [C, Eb, Gb]);
        assert_eq!(diminished(G), [G, Bb, Db]);
    }

    #[test]
    fn sus2_chord() {
        assert_eq!(sus2(C), [C, D, G]);
    }

    #[test]
    fn sus4_chord() {
        assert_eq!(sus4(C), [C, F, G]);
    }
}
