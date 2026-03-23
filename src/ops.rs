use std::ops::Add;

use crate::chord::Chord;
use crate::interval::Interval;

impl Add for Interval {
    type Output = Chord;
    fn add(self, rhs: Interval) -> Chord {
        Chord {
            intervals: vec![Interval::PerfectUnison, self, rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::note::Note::*;

    #[test]
    fn major_triad_from_ops() {
        let chord = Interval::MajorThird + Interval::PerfectFifth;
        assert_eq!(
            chord.intervals,
            vec![Interval::PerfectUnison, Interval::MajorThird, Interval::PerfectFifth]
        );
        assert_eq!(chord.apply_to(C), vec![C, E, G]);
    }

    #[test]
    fn minor_triad_from_ops() {
        assert_eq!(
            (Interval::MinorThird + Interval::PerfectFifth).apply_to(C),
            vec![C, Eb, G]
        );
    }

    #[test]
    fn major7_from_ops() {
        let chord = Interval::MajorThird + Interval::PerfectFifth + Interval::MajorSeventh;
        assert_eq!(chord.apply_to(C), vec![C, E, G, B]);
    }
}
