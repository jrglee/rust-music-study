use super::degree::interval_for;
use crate::interval::Interval;
use crate::scales::Degree;

const SEMITONES: &'static [usize; 7] = &[2, 1, 2, 2, 1, 3, 1];

#[derive(Debug)]
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
        match interval_for(SEMITONES, self.starting_degree().as_number() - 1, degree) {
            Some(interval) => interval,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    use crate::interval::Interval::*;

    #[rstest]
    #[case(Mode::HarmonicMinor,    [PerfectUnison, MajorSecond, MinorThird,  PerfectFourth,   PerfectFifth,    MinorSixth,  MajorSeventh])]
    #[case(Mode::IonianAug5,       [PerfectUnison, MajorSecond, MajorThird,  PerfectFourth,   AugmentedFifth,  MajorSixth,  MajorSeventh])]
    #[case(Mode::DorianLydian,     [PerfectUnison, MajorSecond, MinorThird,  AugmentedFourth, PerfectFifth,    MajorSixth,  MinorSeventh])]
    #[case(Mode::PhrygianDominant, [PerfectUnison, MinorSecond, MajorThird,  PerfectFourth,   PerfectFifth,    MinorSixth,  MinorSeventh])]
    #[case(Mode::SuperLocrian,     [PerfectUnison, MinorSecond, MinorThird,  DiminishedFourth, DiminishedFifth, MinorSixth,  DiminishedSeventh])]
    fn mode_intervals(#[case] mode: Mode, #[case] expected: [Interval; 7]) {
        assert_eq!(mode.intervals(), expected);
    }
}
