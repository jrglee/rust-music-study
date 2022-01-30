use crate::note::Note;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn apply_to_note(&self, note: &Note) -> Note {
        note.semitone_up(self.semitones())
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note::*;

    use super::*;

    macro_rules! interval_test {
        ($name:ident, $start:expr, $interv:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($start.semitone_up($interv.semitones()), $expected);
            }
        };
    }

    interval_test!(check_perfect_unison, C, Interval::PerfectOctave, C);
    interval_test!(check_perfect_octave, C, Interval::PerfectOctave, C);
    interval_test!(check_diminished_second, C, Interval::DiminishedSecond, C);
    interval_test!(check_augmented_seventh, C, Interval::AugmentedSeventh, C);
    interval_test!(check_minor_second, C, Interval::MinorSecond, Db);
    interval_test!(check_augmented_unison, C, Interval::AugmentedUnison, Db);
    interval_test!(check_major_second, C, Interval::MajorSecond, D);
    interval_test!(check_diminished_third, C, Interval::DiminishedThird, D);
    interval_test!(check_minor_third, C, Interval::MinorThird, Eb);
    interval_test!(check_augmented_second, C, Interval::AugmentedSecond, Eb);
    interval_test!(check_major_third, C, Interval::MajorThird, E);
    interval_test!(check_diminished_fourth, C, Interval::DiminishedFourth, E);
    interval_test!(check_perfect_fourth, C, Interval::PerfectFourth, F);
    interval_test!(check_tritone, C, Interval::Tritone, Gb);
    interval_test!(check_diminished_fifth, C, Interval::DiminishedFifth, Gb);
    interval_test!(check_augmented_fourth, C, Interval::AugmentedFourth, Gb);
    interval_test!(check_perfect_fifth, C, Interval::PerfectFifth, G);
    interval_test!(check_diminished_sixth, C, Interval::DiminishedSixth, G);
    interval_test!(check_minor_sixth, C, Interval::MinorSixth, Ab);
    interval_test!(check_augmented_fifth, C, Interval::AugmentedFifth, Ab);
    interval_test!(check_major_sixth, C, Interval::MajorSixth, A);
    interval_test!(check_diminished_seventh, C, Interval::DiminishedSeventh, A);
    interval_test!(check_minor_seventh, C, Interval::MinorSeventh, Bb);
    interval_test!(check_augmented_sixth, C, Interval::AugmentedSixth, Bb);
    interval_test!(check_major_seventh, C, Interval::MajorSeventh, B);
    interval_test!(check_diminished_octave, C, Interval::DiminishedOctave, B);
}
