use crate::interval::Interval;

pub enum Triad {
    Major,
    Minor,
    Diminished,
    Augmented,
    Sus4,
    Sus2,
}

impl Triad {
    pub fn intervals(&self) -> [Interval; 3] {
        match self {
            Triad::Major => [
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::PerfectFifth,
            ],
            Triad::Minor => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::PerfectFifth,
            ],
            Triad::Diminished => [
                Interval::PerfectUnison,
                Interval::MinorThird,
                Interval::DiminishedFifth,
            ],
            Triad::Augmented => [
                Interval::PerfectUnison,
                Interval::MajorThird,
                Interval::AugmentedFifth,
            ],
            Triad::Sus4 => [
                Interval::PerfectUnison,
                Interval::AugmentedThird,
                Interval::PerfectFifth,
            ],
            Triad::Sus2 => [
                Interval::PerfectUnison,
                Interval::DiminishedThird,
                Interval::PerfectFifth,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note::*;

    use super::*;

    macro_rules! triad_test {
        ($name:ident, $triad:expr, $n1: expr, $n2:expr, $n3:expr) => {
            #[test]
            fn $name() {
                let ints = $triad.intervals();
                assert_eq!(ints[1].apply_to_note($n1), $n2);
                assert_eq!(ints[2].apply_to_note($n1), $n3);
            }
        };
    }

    triad_test!(major, Triad::Major, C, E, G);
    triad_test!(minor, Triad::Minor, C, Eb, G);
    triad_test!(augmented, Triad::Augmented, C, E, Ab);
    triad_test!(diminished, Triad::Diminished, C, Eb, Gb);
    triad_test!(sus2, Triad::Sus2, C, D, G);
    triad_test!(sus4, Triad::Sus4, C, F, G);
}
