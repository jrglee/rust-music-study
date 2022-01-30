use crate::intervals::Interval;

const IONIAN_INTERVALS: &'static [Interval; 7] = &[
    Interval::PerfectUnison,
    Interval::MajorSecond,
    Interval::MajorThird,
    Interval::PerfectFourth,
    Interval::PerfectFifth,
    Interval::MajorSixth,
    Interval::MajorSeventh,
];

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
    pub fn intervals(&self) -> [Interval; 7] {
        let shift = self.shift_from_ionian();
        let mut ints: [Interval; 7] = IONIAN_INTERVALS.clone();
        ints.rotate_left(shift);

        let offset = ints[0].semitones();
        ints.iter()
            .map(|int| diatonic_interval(int.semitones() + 12 - offset, Some(self)))
            .collect::<Vec<Interval>>()
            .try_into()
            .unwrap()
    }

    fn shift_from_ionian(&self) -> usize {
        match self {
            DiatonicMode::Ionian => 0,
            DiatonicMode::Dorian => 1,
            DiatonicMode::Phrygian => 2,
            DiatonicMode::Lydian => 3,
            DiatonicMode::Mixolydian => 4,
            DiatonicMode::Aeolian => 5,
            DiatonicMode::Locrian => 6,
        }
    }
}

fn diatonic_interval(semitone: usize, mode_context: Option<&DiatonicMode>) -> Interval {
    match (mode_context, semitone % 12) {
        (Some(DiatonicMode::Locrian), 6) => Interval::DiminishedFifth,
        _ => {
            match semitone % 12 {
                0 => Interval::PerfectUnison,
                1 => Interval::MinorSecond,
                2 => Interval::MajorSecond,
                3 => Interval::MinorThird,
                4 => Interval::MajorThird,
                5 => Interval::PerfectFourth,
                6 => Interval::AugmentedFourth,
                7 => Interval::PerfectFifth,
                8 => Interval::MinorSixth,
                9 => Interval::MajorSixth,
                10 => Interval::MinorSeventh,
                11 => Interval::MajorSeventh,
                _ => panic!("Unreachable case"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::intervals::Interval::*;

    macro_rules! mode_interval_test {
        ($name:ident, $mode:expr, $intervals:expr) => {
            #[test]
            fn $name() {
                assert_eq!($mode.intervals(), $intervals);
            }
        };
    }

    mode_interval_test!(lydian_intervals, DiatonicMode::Lydian, [
        PerfectUnison,
        MajorSecond,
        MajorThird,
        AugmentedFourth,
        PerfectFifth,
        MajorSixth,
        MajorSeventh,
    ]);

    mode_interval_test!(ionian_intervals, DiatonicMode::Ionian, [
        PerfectUnison,
        MajorSecond,
        MajorThird,
        PerfectFourth,
        PerfectFifth,
        MajorSixth,
        MajorSeventh,
    ]);

    mode_interval_test!(mixolydian_intervals, DiatonicMode::Mixolydian, [
        PerfectUnison,
        MajorSecond,
        MajorThird,
        PerfectFourth,
        PerfectFifth,
        MajorSixth,
        MinorSeventh,
    ]);

    mode_interval_test!(dorian_intervals, DiatonicMode::Dorian, [
        PerfectUnison,
        MajorSecond,
        MinorThird,
        PerfectFourth,
        PerfectFifth,
        MajorSixth,
        MinorSeventh,
    ]);

    mode_interval_test!(aeolian_intervals, DiatonicMode::Aeolian, [
        PerfectUnison,
        MajorSecond,
        MinorThird,
        PerfectFourth,
        PerfectFifth,
        MinorSixth,
        MinorSeventh,
    ]);

    mode_interval_test!(phrygian_phrygian, DiatonicMode::Phrygian, [
        PerfectUnison,
        MinorSecond,
        MinorThird,
        PerfectFourth,
        PerfectFifth,
        MinorSixth,
        MinorSeventh,
    ]);

    mode_interval_test!(locrian_phrygian, DiatonicMode::Locrian, [
        PerfectUnison,
        MinorSecond,
        MinorThird,
        PerfectFourth,
        DiminishedFifth,
        MinorSixth,
        MinorSeventh,
    ]);
}