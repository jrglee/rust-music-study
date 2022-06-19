use crate::intervals::Interval;

const HARMONIC_MINOR_INTERVALS: &'static [Interval; 7] = &[
    Interval::PerfectUnison,
    Interval::MajorSecond,
    Interval::MinorThird,
    Interval::PerfectFourth,
    Interval::PerfectFifth,
    Interval::MinorSixth,
    Interval::MajorSeventh,
];

pub enum HarmonicMinorMode {
    HarmonicMinor,
    LocrianMaj6,
    IonianAug5,
    DorianLydian,
    PhrygianDominant,
    LydianAug2,
    SuperLocrian,
}

impl HarmonicMinorMode {
    pub fn intervals(&self) -> [Interval; 7] {
        let shift = self.shift_from_harmonic_minor();
        let mut ints: [Interval; 7] = HARMONIC_MINOR_INTERVALS.clone();
        ints.rotate_left(shift);

        let offset = ints[0].semitones();
        ints.iter()
            .map(|int| harmonic_minor_interval(int.semitones() + 12 - offset, Some(self)))
            .collect::<Vec<Interval>>()
            .try_into()
            .unwrap()
    }

    fn shift_from_harmonic_minor(&self) -> usize {
        match self {
            HarmonicMinorMode::HarmonicMinor => 0,
            HarmonicMinorMode::LocrianMaj6 => 1,
            HarmonicMinorMode::IonianAug5 => 2,
            HarmonicMinorMode::DorianLydian => 3,
            HarmonicMinorMode::PhrygianDominant => 4,
            HarmonicMinorMode::LydianAug2 => 5,
            HarmonicMinorMode::SuperLocrian => 6,
        }
    }
}


fn harmonic_minor_interval(semitone: usize, mode_context: Option<&HarmonicMinorMode>) -> Interval {
    match (mode_context, semitone % 12) {
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
                assert_eq!($mode.intervals(), $intervals)
            }
        };
    }

    mode_interval_test!(harmonic_minor_intervals, HarmonicMinorMode::HarmonicMinor, [
        PerfectUnison,
        MajorSecond,
        MinorThird,
        PerfectFourth,
        PerfectFifth,
        MinorSixth,
        MajorSeventh,
    ]);

    mode_interval_test!(dorian_lydian_intervals, HarmonicMinorMode::DorianLydian, [
        PerfectUnison,
        MajorSecond,
        MinorThird,
        AugmentedFourth,
        PerfectFifth,
        MajorSixth,
        MinorSeventh,
    ]);

    mode_interval_test!(phrygian_dominant_intervals, HarmonicMinorMode::PhrygianDominant, [
        PerfectUnison,
        MinorSecond,
        MajorThird,
        PerfectFourth,
        PerfectFifth,
        MinorSixth,
        MinorSeventh,
    ]);
}