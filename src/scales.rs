use crate::note::{Interval, Note};

const MAJOR_INTERVALS: &'static [u8] = &[2, 2, 1, 2, 2, 2, 1];

mod modes {
    pub enum DiatonicMode {
        Ionian,
        Dorian,
        Phrygian,
        Lydian,
        Mixolydian,
        Aeolian,
        Locrian,
    }

    impl DiatonicMode {
        pub fn intervals(&self) -> Vec<u8> {
            let shift = match self {
                DiatonicMode::Ionian => 0,
                DiatonicMode::Dorian => 1,
                DiatonicMode::Phrygian => 2,
                DiatonicMode::Lydian => 3,
                DiatonicMode::Mixolydian => 4,
                DiatonicMode::Aeolian => 5,
                DiatonicMode::Locrian => 6,
            };

            let mut int: Vec<u8> = super::MAJOR_INTERVALS.to_vec();
            int.rotate_left(shift);
            int
        }
    }
}

fn generate_scale<'a>(key: &'a Note, intervals: &[u8]) -> Vec<&'a Note> {
    let mut res: Vec<&Note> = vec![];
    let mut n: &Note = key;
    res.push(n);
    for &interval in intervals.iter() {
        for _ in 0..interval {
            n = n.half_step_up();
        }
        res.push(n);
    }

    res
}

use modes::DiatonicMode;

pub fn major(key: &Note) -> Vec<&Note> {
    generate_scale(key, MAJOR_INTERVALS)
}

pub fn minor(key: &Note) -> Vec<&Note> {
    generate_scale(key, &DiatonicMode::Aeolian.intervals())
}

pub fn diatonic_mode<'a>(key: &'a Note, mode: &DiatonicMode) -> Vec<&'a Note> {
    generate_scale(key, &mode.intervals())
}

#[cfg(test)]
mod tests {
    use super::*;
    use DiatonicMode::*;
    use Note::*;

    #[test]
    fn major_scale() {
        assert_eq!(major(&C), vec![&C, &D, &E, &F, &G, &A, &B, &C]);
        assert_eq!(major(&G), vec![&G, &A, &B, &C, &D, &E, &Gb, &G]);
    }

    #[test]
    fn minor_scale() {
        assert_eq!(minor(&A), vec![&A, &B, &C, &D, &E, &F, &G, &A]);
        assert_eq!(minor(&E), vec![&E, &Gb, &G, &A, &B, &C, &D, &E]);
    }

    #[test]
    fn diatonic_modes() {
        assert_eq!(
            diatonic_mode(&C, &Ionian),
            vec![&C, &D, &E, &F, &G, &A, &B, &C]
        );
        assert_eq!(
            diatonic_mode(&D, &Dorian),
            vec![&D, &E, &F, &G, &A, &B, &C, &D]
        );
        assert_eq!(
            diatonic_mode(&E, &Phrygian),
            vec![&E, &F, &G, &A, &B, &C, &D, &E]
        );
        assert_eq!(
            diatonic_mode(&F, &Lydian),
            vec![&F, &G, &A, &B, &C, &D, &E, &F]
        );
        assert_eq!(
            diatonic_mode(&G, &Mixolydian),
            vec![&G, &A, &B, &C, &D, &E, &F, &G]
        );
        assert_eq!(
            diatonic_mode(&A, &Aeolian),
            vec![&A, &B, &C, &D, &E, &F, &G, &A]
        );
        assert_eq!(
            diatonic_mode(&B, &Locrian),
            vec![&B, &C, &D, &E, &F, &G, &A, &B]
        );
    }
}
