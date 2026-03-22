use crate::interval::Interval;

#[derive(Debug)]
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
    use rstest::rstest;

    use crate::note::Note;
    use crate::note::Note::*;

    use super::*;

    #[rstest]
    #[case(Tetrad::Major7, C, E, G, B)]
    #[case(Tetrad::Dominant, C, E, G, Bb)]
    #[case(Tetrad::Minor7, C, Eb, G, Bb)]
    #[case(Tetrad::MinorMajor7, C, Eb, G, B)]
    #[case(Tetrad::Minor7Flat5, C, Eb, Gb, Bb)]
    #[case(Tetrad::Diminished7, C, Eb, Gb, A)]
    fn tetrad(#[case] t: Tetrad, #[case] root: Note, #[case] third: Note, #[case] fifth: Note, #[case] seventh: Note) {
        let ints = t.intervals();
        assert_eq!(ints[1].apply_to_note(root), third);
        assert_eq!(ints[2].apply_to_note(root), fifth);
        assert_eq!(ints[3].apply_to_note(root), seventh);
    }
}
