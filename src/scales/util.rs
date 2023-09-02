use crate::interval::Interval;
use crate::note::Note;
use crate::scales::diatonic::Mode;

fn generate_scale(key: &Note, intervals: &[Interval]) -> Vec<Note> {
    intervals.iter().map(|int| int.apply_to_note(key)).collect()
}

pub fn major(key: &Note) -> Vec<Note> {
    generate_scale(key, &Mode::Ionian.intervals())
}

pub fn minor(key: &Note) -> Vec<Note> {
    generate_scale(key, &Mode::Aeolian.intervals())
}

pub fn diatonic_mode(key: &Note, mode: &Mode) -> Vec<Note> {
    generate_scale(key, &mode.intervals())
}

#[cfg(test)]
mod tests {
    use Mode::*;
    use Note::*;

    use super::*;

    #[test]
    fn major_scale() {
        assert_eq!(major(&C), vec![C, D, E, F, G, A, B]);
        assert_eq!(major(&G), vec![G, A, B, C, D, E, Gb]);
    }

    #[test]
    fn minor_scale() {
        assert_eq!(minor(&A), vec![A, B, C, D, E, F, G]);
        assert_eq!(minor(&E), vec![E, Gb, G, A, B, C, D]);
    }

    #[test]
    fn diatonic_modes() {
        assert_eq!(diatonic_mode(&C, &Ionian), vec![C, D, E, F, G, A, B]);
        assert_eq!(diatonic_mode(&D, &Dorian), vec![D, E, F, G, A, B, C]);
        assert_eq!(diatonic_mode(&E, &Phrygian), vec![E, F, G, A, B, C, D]);
        assert_eq!(diatonic_mode(&F, &Lydian), vec![F, G, A, B, C, D, E]);
        assert_eq!(diatonic_mode(&G, &Mixolydian), vec![G, A, B, C, D, E, F]);
        assert_eq!(diatonic_mode(&A, &Aeolian), vec![A, B, C, D, E, F, G]);
        assert_eq!(diatonic_mode(&B, &Locrian), vec![B, C, D, E, F, G, A]);
    }
}
