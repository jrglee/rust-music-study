use std::collections::HashSet;

use crate::interval::Interval;

pub fn interval_for(intervals: &[usize], offset: usize, degree: Degree) -> Option<Interval> {
    let mut shifted = vec![];
    shifted.extend_from_slice(intervals);
    shifted.rotate_left(offset);

    let semitones = shifted.iter().take(degree.as_number() - 1).sum();

    degree.interval(semitones)
}

#[derive(Debug)]
pub enum Degree {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

impl Degree {
    pub fn array() -> [Degree; 7] {
        [
            Degree::First,
            Degree::Second,
            Degree::Third,
            Degree::Fourth,
            Degree::Fifth,
            Degree::Sixth,
            Degree::Seventh,
        ]
    }

    pub fn as_number(&self) -> usize {
        match self {
            Degree::First => 1,
            Degree::Second => 2,
            Degree::Third => 3,
            Degree::Fourth => 4,
            Degree::Fifth => 5,
            Degree::Sixth => 6,
            Degree::Seventh => 7,
        }
    }

    fn valid_intervals(&self) -> HashSet<Interval> {
        match self {
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

    pub fn interval(&self, semitones: usize) -> Option<Interval> {
        self.valid_intervals()
            .iter()
            .find(|c| c.semitones() == semitones)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Degree::First, 0, Interval::PerfectUnison)]
    #[case(Degree::Second, 0, Interval::DiminishedSecond)]
    #[case(Degree::Second, 1, Interval::MinorSecond)]
    #[case(Degree::Second, 2, Interval::MajorSecond)]
    #[case(Degree::Second, 3, Interval::AugmentedSecond)]
    #[case(Degree::Third, 2, Interval::DiminishedThird)]
    #[case(Degree::Third, 3, Interval::MinorThird)]
    #[case(Degree::Third, 4, Interval::MajorThird)]
    #[case(Degree::Third, 5, Interval::AugmentedThird)]
    #[case(Degree::Fourth, 4, Interval::DiminishedFourth)]
    #[case(Degree::Fourth, 5, Interval::PerfectFourth)]
    #[case(Degree::Fourth, 6, Interval::AugmentedFourth)]
    #[case(Degree::Fifth, 6, Interval::DiminishedFifth)]
    #[case(Degree::Fifth, 7, Interval::PerfectFifth)]
    #[case(Degree::Fifth, 8, Interval::AugmentedFifth)]
    #[case(Degree::Sixth, 7, Interval::DiminishedSixth)]
    #[case(Degree::Sixth, 8, Interval::MinorSixth)]
    #[case(Degree::Sixth, 9, Interval::MajorSixth)]
    #[case(Degree::Sixth, 10, Interval::AugmentedSixth)]
    #[case(Degree::Seventh, 9, Interval::DiminishedSeventh)]
    #[case(Degree::Seventh, 10, Interval::MinorSeventh)]
    #[case(Degree::Seventh, 11, Interval::MajorSeventh)]
    #[case(Degree::Seventh, 12, Interval::AugmentedSeventh)]
    fn degree_interval(#[case] degree: Degree, #[case] semitones: usize, #[case] expected: Interval) {
        assert_eq!(degree.interval(semitones), Some(expected));
        assert_eq!(expected.semitones(), semitones);
    }
}
