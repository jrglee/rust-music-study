use crate::interval::Interval;
use crate::scales::Degree;
use super::degree::interval_for;

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
        Degree::array().map(|d| self.interval_for(d))
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
        match interval_for(SEMITONES, self.starting_degree().as_number() - 1, degree) {
            Some(interval) => interval,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::interval::Interval::*;

    use super::*;

    #[rstest]
    #[case(Mode::Lydian,      [PerfectUnison, MajorSecond, MajorThird, AugmentedFourth, PerfectFifth, MajorSixth,  MajorSeventh])]
    #[case(Mode::Ionian,      [PerfectUnison, MajorSecond, MajorThird, PerfectFourth,   PerfectFifth, MajorSixth,  MajorSeventh])]
    #[case(Mode::Mixolydian,  [PerfectUnison, MajorSecond, MajorThird, PerfectFourth,   PerfectFifth, MajorSixth,  MinorSeventh])]
    #[case(Mode::Dorian,      [PerfectUnison, MajorSecond, MinorThird, PerfectFourth,   PerfectFifth, MajorSixth,  MinorSeventh])]
    #[case(Mode::Aeolian,     [PerfectUnison, MajorSecond, MinorThird, PerfectFourth,   PerfectFifth, MinorSixth,  MinorSeventh])]
    #[case(Mode::Phrygian,    [PerfectUnison, MinorSecond, MinorThird, PerfectFourth,   PerfectFifth, MinorSixth,  MinorSeventh])]
    #[case(Mode::Locrian,     [PerfectUnison, MinorSecond, MinorThird, PerfectFourth,   DiminishedFifth, MinorSixth, MinorSeventh])]
    fn mode_intervals(#[case] mode: Mode, #[case] expected: [Interval; 7]) {
        assert_eq!(mode.intervals(), expected);
    }
}
