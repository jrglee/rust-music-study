use crate::interval::Interval;
use crate::note::Note;
use crate::scales::diatonic::Mode;

pub fn generate_scale(key: Note, intervals: &[Interval]) -> Vec<Note> {
    let mut scale: Vec<Note> = intervals.iter().map(|int| int.apply_to_note(key)).collect();
    scale.push(key);
    scale
}

pub fn major(key: Note) -> Vec<Note> {
    diatonic_mode(key, Mode::Ionian)
}

pub fn minor(key: Note) -> Vec<Note> {
    diatonic_mode(key, Mode::Aeolian)
}

pub fn diatonic_mode(key: Note, mode: Mode) -> Vec<Note> {
    generate_scale(key, &mode.intervals())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use Mode::*;
    use Note::*;

    use super::*;

    #[rstest]
    #[case(major(C),                   [C, D, E, F, G, A, B, C])]
    #[case(major(G),                   [G, A, B, C, D, E, Gb, G])]
    #[case(minor(A),                   [A, B, C, D, E, F, G, A])]
    #[case(minor(E),                   [E, Gb, G, A, B, C, D, E])]
    #[case(diatonic_mode(C, Ionian),   [C, D, E, F, G, A, B, C])]
    #[case(diatonic_mode(D, Dorian),   [D, E, F, G, A, B, C, D])]
    #[case(diatonic_mode(E, Phrygian), [E, F, G, A, B, C, D, E])]
    #[case(diatonic_mode(F, Lydian),   [F, G, A, B, C, D, E, F])]
    #[case(diatonic_mode(G, Mixolydian), [G, A, B, C, D, E, F, G])]
    #[case(diatonic_mode(A, Aeolian),  [A, B, C, D, E, F, G, A])]
    #[case(diatonic_mode(B, Locrian),  [B, C, D, E, F, G, A, B])]
    fn scale(#[case] result: Vec<Note>, #[case] expected: [Note; 8]) {
        assert_eq!(result, expected);
    }
}
