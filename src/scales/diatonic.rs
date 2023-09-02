use crate::interval::Interval;
use crate::scales::Degree;

const SEMITONES: &'static [usize; 7] = &[2, 2, 1, 2, 2, 2, 1];

#[derive(Debug)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
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
            Mode::Ionian => Degree::First,
            Mode::Dorian => Degree::Second,
            Mode::Phrygian => Degree::Third,
            Mode::Lydian => Degree::Fourth,
            Mode::Mixolydian => Degree::Fifth,
            Mode::Aeolian => Degree::Sixth,
            Mode::Locrian => Degree::Seventh,
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
        Mode::Lydian,
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
        Mode::Ionian,
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
        Mode::Mixolydian,
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
        Mode::Dorian,
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
        Mode::Aeolian,
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
        Mode::Phrygian,
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
        Mode::Locrian,
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
