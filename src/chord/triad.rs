use crate::interval::Interval;

#[derive(Debug)]
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
            Triad::Major => [Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth],
            Triad::Minor => [Interval::PerfectUnison, Interval::MinorThird, Interval::PerfectFifth],
            Triad::Diminished => [Interval::PerfectUnison, Interval::MinorThird, Interval::DiminishedFifth],
            Triad::Augmented => [Interval::PerfectUnison, Interval::MajorThird, Interval::AugmentedFifth],
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
    use rstest::rstest;

    use crate::note::Note;
    use crate::note::Note::*;

    use super::*;

    #[rstest]
    #[case(Triad::Major,     C, E,  G)]
    #[case(Triad::Minor,     C, Eb, G)]
    #[case(Triad::Augmented, C, E,  Ab)]
    #[case(Triad::Diminished, C, Eb, Gb)]
    #[case(Triad::Sus2,      C, D,  G)]
    #[case(Triad::Sus4,      C, F,  G)]
    fn triad_intervals(#[case] chord: Triad, #[case] root: Note, #[case] third: Note, #[case] fifth: Note) {
        let ints = chord.intervals();
        assert_eq!(ints[1].apply_to_note(root), third);
        assert_eq!(ints[2].apply_to_note(root), fifth);
    }
}
