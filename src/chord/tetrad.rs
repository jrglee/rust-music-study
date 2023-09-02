use crate::interval::Interval;

pub enum Tetrad {
    Major7,
    Dominant,
    Minor7,
    MinorMajor7,
    Minor7Flat5,
    Diminished7,
}

impl Tetrad {
    pub fn intervals(&self) -> [Interval; 4] {
        match self {
            Tetrad::Major7 => [
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MajorSeventh,
            ],
            Tetrad::Dominant => [
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ],
            Tetrad::Minor7 => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MinorSeventh,
            ],
            Tetrad::MinorMajor7 => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFifth,
                Interval::MajorSeventh,
            ],
            Tetrad::Minor7Flat5 => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::DiminishedFifth,
                Interval::MinorSeventh,
            ],
            Tetrad::Diminished7 => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::DiminishedFifth,
                Interval::DiminishedSeventh,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note::*;

    use super::*;

    macro_rules! tetrad_test {
        ($name:ident, $tetrad:expr, $n1:expr, $n2:expr, $n3:expr, $n4:expr) => {
            #[test]
            fn $name() {
                let ints = $tetrad.intervals();
                assert_eq!(ints[1].apply_to_note(&$n1), $n2);
                assert_eq!(ints[2].apply_to_note(&$n1), $n3);
                assert_eq!(ints[3].apply_to_note(&$n1), $n4);
            }
        };
    }

    tetrad_test!(major7, Tetrad::Major7, C, E, G, B);
    tetrad_test!(dominant, Tetrad::Dominant, C, E, G, Bb);
    tetrad_test!(minor7, Tetrad::Minor7, C, Eb, G, Bb);
    tetrad_test!(minor_major7, Tetrad::MinorMajor7, C, Eb, G, B);
    tetrad_test!(minor7_flat5, Tetrad::Minor7Flat5, C, Eb, Gb, Bb);
    tetrad_test!(diminished7, Tetrad::Diminished7, C, Eb, Gb, A);
}
