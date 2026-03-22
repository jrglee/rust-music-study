use std::collections::HashSet;

use crate::note::Note;
use crate::scales::Degree;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Interval {
    PerfectUnison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    PerfectOctave,
    Tritone,
    DiminishedSecond,
    DiminishedThird,
    DiminishedFourth,
    DiminishedFifth,
    DiminishedSixth,
    DiminishedSeventh,
    DiminishedOctave,
    AugmentedUnison,
    AugmentedSecond,
    AugmentedThird,
    AugmentedFourth,
    AugmentedFifth,
    AugmentedSixth,
    AugmentedSeventh,
}

impl Interval {
    pub fn for_degree(degree: &Degree) -> HashSet<Interval> {
        match degree {
            Degree::First => HashSet::from([Interval::PerfectUnison]),
            Degree::Second => HashSet::from([
                Interval::MinorSecond,
                Interval::MajorSecond,
                Interval::DiminishedSecond,
                Interval::AugmentedSecond,
            ]),
            Degree::Third => HashSet::from([
                Interval::MinorThird,
                Interval::MajorThird,
                Interval::DiminishedThird,
                Interval::AugmentedThird,
            ]),
            Degree::Fourth => HashSet::from([
                Interval::PerfectFourth,
                Interval::DiminishedFourth,
                Interval::AugmentedFourth,
            ]),
            Degree::Fifth => HashSet::from([
                Interval::PerfectFifth,
                Interval::DiminishedFifth,
                Interval::AugmentedFifth,
            ]),
            Degree::Sixth => HashSet::from([
                Interval::MinorSixth,
                Interval::MajorSixth,
                Interval::DiminishedSixth,
                Interval::AugmentedSixth,
            ]),
            Degree::Seventh => HashSet::from([
                Interval::MinorSeventh,
                Interval::MajorSeventh,
                Interval::DiminishedSeventh,
                Interval::AugmentedSeventh,
            ]),
        }
    }

    pub fn to_degree(&self) -> Option<Degree> {
        match self {
            Interval::PerfectUnison => Some(Degree::First),
            Interval::MinorSecond => Some(Degree::Second),
            Interval::MajorSecond => Some(Degree::Second),
            Interval::MinorThird => Some(Degree::Third),
            Interval::MajorThird => Some(Degree::Third),
            Interval::PerfectFourth => Some(Degree::Fourth),
            Interval::PerfectFifth => Some(Degree::Fifth),
            Interval::MinorSixth => Some(Degree::Sixth),
            Interval::MajorSixth => Some(Degree::Sixth),
            Interval::MinorSeventh => Some(Degree::Seventh),
            Interval::MajorSeventh => Some(Degree::Seventh),
            Interval::PerfectOctave => Some(Degree::Fifth),
            Interval::Tritone => None,
            Interval::DiminishedSecond => Some(Degree::Second),
            Interval::DiminishedThird => Some(Degree::Third),
            Interval::DiminishedFourth => Some(Degree::Fourth),
            Interval::DiminishedFifth => Some(Degree::Fifth),
            Interval::DiminishedSixth => Some(Degree::Sixth),
            Interval::DiminishedSeventh => Some(Degree::Seventh),
            Interval::DiminishedOctave => Some(Degree::First),
            Interval::AugmentedUnison => Some(Degree::First),
            Interval::AugmentedSecond => Some(Degree::Second),
            Interval::AugmentedThird => Some(Degree::Third),
            Interval::AugmentedFourth => Some(Degree::Fourth),
            Interval::AugmentedFifth => Some(Degree::Fifth),
            Interval::AugmentedSixth => Some(Degree::Sixth),
            Interval::AugmentedSeventh => Some(Degree::Seventh),
        }
    }

    pub fn semitones(&self) -> usize {
        match self {
            Interval::PerfectUnison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::PerfectOctave => 12,
            Interval::Tritone => 6,
            Interval::DiminishedSecond => 0,
            Interval::DiminishedThird => 2,
            Interval::DiminishedFourth => 4,
            Interval::DiminishedSixth => 7,
            Interval::DiminishedSeventh => 9,
            Interval::DiminishedOctave => 11,
            Interval::AugmentedUnison => 1,
            Interval::AugmentedSecond => 3,
            Interval::AugmentedThird => 5,
            Interval::AugmentedFourth => 6,
            Interval::AugmentedFifth => 8,
            Interval::AugmentedSixth => 10,
            Interval::AugmentedSeventh => 12,
        }
    }

    pub fn apply_to_note(&self, note: Note) -> Note {
        note.semitones_up(self.semitones())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::note::Note::*;

    use super::*;

    #[rstest]
    #[case(Interval::PerfectUnison,   C)]
    #[case(Interval::PerfectOctave,   C)]
    #[case(Interval::DiminishedSecond, C)]
    #[case(Interval::AugmentedSeventh, C)]
    #[case(Interval::MinorSecond,     Db)]
    #[case(Interval::AugmentedUnison, Db)]
    #[case(Interval::MajorSecond,     D)]
    #[case(Interval::DiminishedThird, D)]
    #[case(Interval::MinorThird,      Eb)]
    #[case(Interval::AugmentedSecond, Eb)]
    #[case(Interval::MajorThird,      E)]
    #[case(Interval::DiminishedFourth, E)]
    #[case(Interval::PerfectFourth,   F)]
    #[case(Interval::Tritone,         Gb)]
    #[case(Interval::DiminishedFifth, Gb)]
    #[case(Interval::AugmentedFourth, Gb)]
    #[case(Interval::PerfectFifth,    G)]
    #[case(Interval::DiminishedSixth, G)]
    #[case(Interval::MinorSixth,      Ab)]
    #[case(Interval::AugmentedFifth,  Ab)]
    #[case(Interval::MajorSixth,      A)]
    #[case(Interval::DiminishedSeventh, A)]
    #[case(Interval::MinorSeventh,    Bb)]
    #[case(Interval::AugmentedSixth,  Bb)]
    #[case(Interval::MajorSeventh,    B)]
    #[case(Interval::DiminishedOctave, B)]
    fn interval_from_c(#[case] interv: Interval, #[case] expected: Note) {
        assert_eq!(C.semitones_up(interv.semitones()), expected);
    }
}
