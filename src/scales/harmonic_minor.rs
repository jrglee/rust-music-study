use crate::interval::Interval;
use crate::scales::{helper, Degree};

const SEMITONES: &'static [usize; 7] = &[2, 1, 2, 2, 1, 3, 1];

pub enum Mode {
    HarmonicMinor,
    LocrianMaj6,
    IonianAug5,
    DorianLydian,
    PhrygianDominant,
    LydianAug2,
    SuperLocrian,
}

impl Mode {
    pub fn intervals(&self) -> [Interval; 7] {
        Degree::array().map(|d| self.interval_for(d))
    }

    fn starting_degree(&self) -> Degree {
        match self {
            Mode::HarmonicMinor => Degree::First,
            Mode::LocrianMaj6 => Degree::Second,
            Mode::IonianAug5 => Degree::Third,
            Mode::DorianLydian => Degree::Fourth,
            Mode::PhrygianDominant => Degree::Fifth,
            Mode::LydianAug2 => Degree::Sixth,
            Mode::SuperLocrian => Degree::Seventh,
        }
    }

    fn interval_for(&self, degree: Degree) -> Interval {
        match helper::interval_for(SEMITONES, self.starting_degree().as_number() - 1, degree) {
            Some(interval) => interval,
            None => panic!("Unreachable case"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::interval::Interval::*;

    macro_rules! mode_interval_test {
        ($name:ident, $mode:expr, $intervals:expr) => {
            #[test]
            fn $name() {
                assert_eq!($mode.intervals(), $intervals)
            }
        };
    }

    mode_interval_test!(
        harmonic_minor_intervals,
        Mode::HarmonicMinor,
        [
            PerfectUnison,
            MajorSecond,
            MinorThird,
            PerfectFourth,
            PerfectFifth,
            MinorSixth,
            MajorSeventh,
        ]
    );

    mode_interval_test!(
        ionian_aug5_intervals,
        Mode::IonianAug5,
        [
            PerfectUnison,
            MajorSecond,
            MajorThird,
            PerfectFourth,
            AugmentedFifth,
            MajorSixth,
            MajorSeventh
        ]
    );

    mode_interval_test!(
        dorian_lydian_intervals,
        Mode::DorianLydian,
        [
            PerfectUnison,
            MajorSecond,
            MinorThird,
            AugmentedFourth,
            PerfectFifth,
            MajorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        phrygian_dominant_intervals,
        Mode::PhrygianDominant,
        [
            PerfectUnison,
            MinorSecond,
            MajorThird,
            PerfectFourth,
            PerfectFifth,
            MinorSixth,
            MinorSeventh,
        ]
    );

    mode_interval_test!(
        super_locrian_intervals,
        Mode::SuperLocrian,
        [
            PerfectUnison,
            MinorSecond,
            MinorThird,
            DiminishedFourth,
            DiminishedFifth,
            MinorSixth,
            DiminishedSeventh,
        ]
    );
}
