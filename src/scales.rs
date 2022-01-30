use modes::DiatonicMode;

use crate::intervals::Interval;
use crate::note::Note;

const MAJOR_INTERVALS: &'static [Interval; 7] = &[
    Interval::PerfectUnison,
    Interval::MajorSecond,
    Interval::MajorThird,
    Interval::PerfectFourth,
    Interval::PerfectFifth,
    Interval::MajorSixth,
    Interval::MajorSeventh,
];

mod modes {
    use crate::intervals::Interval;

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
        pub fn intervals(&self) -> Vec<Interval> {
            let shift = match self {
                DiatonicMode::Ionian => 0,
                DiatonicMode::Dorian => 1,
                DiatonicMode::Phrygian => 2,
                DiatonicMode::Lydian => 3,
                DiatonicMode::Mixolydian => 4,
                DiatonicMode::Aeolian => 5,
                DiatonicMode::Locrian => 6,
            };

            let mut ints: [Interval; 7] = super::MAJOR_INTERVALS.clone();
            ints.rotate_left(shift);

            let offset = ints[0].semitones();
            ints.iter()
                .map(|int| Interval::diatonic_interval(int.semitones() + 12 - offset))
                .collect()
        }
    }
}

fn generate_scale(key: &Note, intervals: &[Interval]) -> Vec<Note> {
    let mut res: Vec<Note> = intervals.iter()
        .map(|int| int.apply_to_note(key))
        .collect();
    res.push(key.to_owned());
    res
}

pub fn major(key: &Note) -> Vec<Note> {
    generate_scale(key, &DiatonicMode::Ionian.intervals())
}

pub fn minor(key: &Note) -> Vec<Note> {
    generate_scale(key, &DiatonicMode::Aeolian.intervals())
}

pub fn diatonic_mode(key: &Note, mode: &DiatonicMode) -> Vec<Note> {
    generate_scale(key, &mode.intervals())
}

#[cfg(test)]
mod tests {
    use DiatonicMode::*;
    use Note::*;

    use super::*;

    #[test]
    fn major_scale() {
        assert_eq!(major(&C), vec![C, D, E, F, G, A, B, C]);
        assert_eq!(major(&G), vec![G, A, B, C, D, E, Gb, G]);
    }

    #[test]
    fn minor_scale() {
        assert_eq!(minor(&A), vec![A, B, C, D, E, F, G, A]);
        assert_eq!(minor(&E), vec![E, Gb, G, A, B, C, D, E]);
    }

    #[test]
    fn diatonic_modes() {
        assert_eq!(diatonic_mode(&C, &Ionian), vec![C, D, E, F, G, A, B, C]);
        assert_eq!(diatonic_mode(&D, &Dorian), vec![D, E, F, G, A, B, C, D]);
        assert_eq!(diatonic_mode(&E, &Phrygian), vec![E, F, G, A, B, C, D, E]);
        assert_eq!(diatonic_mode(&F, &Lydian), vec![F, G, A, B, C, D, E, F]);
        assert_eq!(diatonic_mode(&G, &Mixolydian), vec![G, A, B, C, D, E, F, G]);
        assert_eq!(diatonic_mode(&A, &Aeolian), vec![A, B, C, D, E, F, G, A]);
        assert_eq!(diatonic_mode(&B, &Locrian), vec![B, C, D, E, F, G, A, B]);
    }
}
