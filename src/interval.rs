use crate::note::Note;

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

pub(crate) fn canonical_interval(semitones: usize) -> Interval {
    match semitones % 12 {
        0  => Interval::PerfectUnison,
        1  => Interval::MinorSecond,
        2  => Interval::MajorSecond,
        3  => Interval::MinorThird,
        4  => Interval::MajorThird,
        5  => Interval::PerfectFourth,
        6  => Interval::Tritone,
        7  => Interval::PerfectFifth,
        8  => Interval::MinorSixth,
        9  => Interval::MajorSixth,
        10 => Interval::MinorSeventh,
        11 => Interval::MajorSeventh,
        _  => unreachable!(),
    }
}

impl Interval {
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
