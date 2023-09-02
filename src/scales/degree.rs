use crate::interval::Interval;

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

    pub fn interval(&self, semitones: usize) -> Option<Interval> {
        Interval::for_degree(self)
            .iter()
            .find(|c| c.semitones() == semitones)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;

    macro_rules! degree_semitones_test {
        ($name:ident, $degree:expr, $semitones:expr, $expected:expr) => {
            paste! {
                #[test]
                fn [<degree_ $name>]() {
                    assert_eq!($degree.interval($semitones), Some($expected));
                    assert_eq!($expected.semitones(), $semitones);
                }
            }
        };
    }

    degree_semitones_test!(first_0, Degree::First, 0, Interval::PerfectUnison);
    degree_semitones_test!(second_0, Degree::Second, 0, Interval::DiminishedSecond);
    degree_semitones_test!(second_1, Degree::Second, 1, Interval::MinorSecond);
    degree_semitones_test!(second_2, Degree::Second, 2, Interval::MajorSecond);
    degree_semitones_test!(second_3, Degree::Second, 3, Interval::AugmentedSecond);
    degree_semitones_test!(third_2, Degree::Third, 2, Interval::DiminishedThird);
    degree_semitones_test!(third_3, Degree::Third, 3, Interval::MinorThird);
    degree_semitones_test!(third_4, Degree::Third, 4, Interval::MajorThird);
    degree_semitones_test!(third_5, Degree::Third, 5, Interval::AugmentedThird);
    degree_semitones_test!(fourth_4, Degree::Fourth, 4, Interval::DiminishedFourth);
    degree_semitones_test!(fourth_5, Degree::Fourth, 5, Interval::PerfectFourth);
    degree_semitones_test!(fourth_6, Degree::Fourth, 6, Interval::AugmentedFourth);
    degree_semitones_test!(fifth_6, Degree::Fifth, 6, Interval::DiminishedFifth);
    degree_semitones_test!(fifth_7, Degree::Fifth, 7, Interval::PerfectFifth);
    degree_semitones_test!(fifth_8, Degree::Fifth, 8, Interval::AugmentedFifth);
    degree_semitones_test!(sixth_7, Degree::Sixth, 7, Interval::DiminishedSixth);
    degree_semitones_test!(sixth_8, Degree::Sixth, 8, Interval::MinorSixth);
    degree_semitones_test!(sixth_9, Degree::Sixth, 9, Interval::MajorSixth);
    degree_semitones_test!(sixth_10, Degree::Sixth, 10, Interval::AugmentedSixth);
    degree_semitones_test!(seventh_9, Degree::Seventh, 9, Interval::DiminishedSeventh);
    degree_semitones_test!(seventh_10, Degree::Seventh, 10, Interval::MinorSeventh);
    degree_semitones_test!(seventh_11, Degree::Seventh, 11, Interval::MajorSeventh);
    degree_semitones_test!(seventh_12, Degree::Seventh, 12, Interval::AugmentedSeventh);
}
