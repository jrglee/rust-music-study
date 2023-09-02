use crate::interval::Interval;
use crate::scales::Degree;

const SEMITONES: &'static [usize; 7] = &[2, 2, 1, 2, 2, 2, 1];

#[derive(Debug)]
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
        [
            self.interval_for(Degree::First),
            self.interval_for(Degree::Second),
            self.interval_for(Degree::Third),
            self.interval_for(Degree::Fourth),
            self.interval_for(Degree::Fifth),
            self.interval_for(Degree::Sixth),
            self.interval_for(Degree::Seventh),
        ]
    }

    fn starting_degree(&self) -> Degree {
        match self {
            DiatonicMode::Ionian => Degree::First,
            DiatonicMode::Dorian => Degree::Second,
            DiatonicMode::Phrygian => Degree::Third,
            DiatonicMode::Lydian => Degree::Fourth,
            DiatonicMode::Mixolydian => Degree::Fifth,
            DiatonicMode::Aeolian => Degree::Sixth,
            DiatonicMode::Locrian => Degree::Seventh,
        }
    }

    fn interval_for(&self, degree: Degree) -> Interval {
        let mut shifted = vec![];
        shifted.extend_from_slice(SEMITONES);
        shifted.rotate_left(self.starting_degree().as_number() - 1);

        let semitones = shifted.iter().take(degree.as_number() - 1).sum();

        match degree.interval(semitones) {
            Some(interval) => interval,
            None => panic!("Unreachable case"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval::*;

    use super::*;

    macro_rules! mode_interval_test {
        ($name:ident, $mode:expr, $intervals:expr) => {
            #[test]
            fn $name() {
                assert_eq!($mode.intervals(), $intervals);
            }
        };
    }

    mode_interval_test!(
        lydian_intervals,
        DiatonicMode::Lydian,
        [
            PerfectUnison,
            MajorSecond,
            MajorThird,
            AugmentedFourth,
            PerfectFifth,
            MajorSixth,
            MajorSeventh,
        ]
    );

    mode_interval_test!(
        ionian_intervals,
        DiatonicMode::Ionian,
        [
            PerfectUnison,
            MajorSecond,
            MajorThird,
            PerfectFourth,
            PerfectFifth,
            MajorSixth,
            MajorSeventh,
        ]
    );

    mode_interval_test!(
        mixolydian_intervals,
        DiatonicMode::Mixolydian,
        [
            PerfectUnison,
            MajorSecond,
            MajorThird,
            PerfectFourth,
            PerfectFifth,
            MajorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        dorian_intervals,
        DiatonicMode::Dorian,
        [
            PerfectUnison,
            MajorSecond,
            MinorThird,
            PerfectFourth,
            PerfectFifth,
            MajorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        aeolian_intervals,
        DiatonicMode::Aeolian,
        [
            PerfectUnison,
            MajorSecond,
            MinorThird,
            PerfectFourth,
            PerfectFifth,
            MinorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        phrygian_phrygian,
        DiatonicMode::Phrygian,
        [
            PerfectUnison,
            MinorSecond,
            MinorThird,
            PerfectFourth,
            PerfectFifth,
            MinorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        locrian_phrygian,
        DiatonicMode::Locrian,
        [
            PerfectUnison,
            MinorSecond,
            MinorThird,
            PerfectFourth,
            DiminishedFifth,
            MinorSixth,
            MinorSeventh,
        ]
    );
}
