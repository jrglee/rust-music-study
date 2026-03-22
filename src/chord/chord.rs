use std::ops::Add;

use crate::interval::Interval;
use crate::note::Note;

/// A chord expressed as a set of intervals from an implied root.
/// The first element is always `PerfectUnison` (the root itself).
#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    pub intervals: Vec<Interval>,
}

impl Chord {
    /// Apply this chord to a root note, returning the notes of the chord.
    pub fn apply_to(&self, root: Note) -> Vec<Note> {
        self.intervals.iter().map(|i| root + *i).collect()
    }
}

impl Add<Interval> for Chord {
    type Output = Chord;
    fn add(self, rhs: Interval) -> Chord {
        let mut intervals = self.intervals;
        intervals.push(rhs);
        Chord { intervals }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::chord::chord::Chord;
    use crate::interval::Interval;
    use crate::note::Note::*;

    #[rstest]
    #[case(Interval::MajorThird, Interval::PerfectFifth, vec![C, E, G])]
    #[case(Interval::MinorThird, Interval::PerfectFifth, vec![C, Eb, G])]
    #[case(Interval::MajorThird, Interval::AugmentedFifth, vec![C, E, Ab])]
    #[case(Interval::MinorThird, Interval::DiminishedFifth, vec![C, Eb, Gb])]
    fn chord_apply_to_c(#[case] third: Interval, #[case] fifth: Interval, #[case] expected: Vec<crate::note::Note>) {
        use crate::interval::Interval as I;
        let chord = Chord { intervals: vec![I::PerfectUnison, third, fifth] };
        assert_eq!(chord.apply_to(C), expected);
    }

    #[test]
    fn chord_add_interval_extends() {
        let triad = Chord { intervals: vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth] };
        let maj7 = triad + Interval::MajorSeventh;
        assert_eq!(maj7.apply_to(C), vec![C, E, G, B]);
    }
}
