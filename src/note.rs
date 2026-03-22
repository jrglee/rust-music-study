use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::interval::{canonical_interval, Interval};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
}

impl Note {
    pub(crate) fn semitones_from_c(&self) -> usize {
        match self {
            Note::C => 0,
            Note::Db => 1,
            Note::D => 2,
            Note::Eb => 3,
            Note::E => 4,
            Note::F => 5,
            Note::Gb => 6,
            Note::G => 7,
            Note::Ab => 8,
            Note::A => 9,
            Note::Bb => 10,
            Note::B => 11,
        }
    }

    pub fn semitones_up(&self, semitones: usize) -> Note {
        semitones_to_c(self.semitones_from_c() + semitones)
    }

    pub fn semitones(&self, semitones: isize) -> Note {
        let corrected: usize = (12 + (semitones % 12)) as usize;
        self.semitones_up(corrected)
    }
}

impl Add<Interval> for Note {
    type Output = Note;
    fn add(self, rhs: Interval) -> Note {
        rhs.apply_to_note(self)
    }
}

impl Sub for Note {
    type Output = Interval;
    fn sub(self, rhs: Note) -> Interval {
        let distance = (self.semitones_from_c() + 12 - rhs.semitones_from_c()) % 12;
        canonical_interval(distance)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid note {0}")]
pub struct NoteParseError(String);

impl FromStr for Note {
    type Err = NoteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "C" => Ok(Note::C),
            "C#" | "DB" => Ok(Note::Db),
            "D" => Ok(Note::D),
            "D#" | "EB" => Ok(Note::Eb),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "F#" | "GB" => Ok(Note::Gb),
            "G" => Ok(Note::G),
            "G#" | "AB" => Ok(Note::Ab),
            "A" => Ok(Note::A),
            "A#" | "BB" => Ok(Note::Bb),
            "B" => Ok(Note::B),
            v => Err(NoteParseError(v.to_string())),
        }
    }
}

fn semitones_to_c(semitones: usize) -> Note {
    match semitones % 12 {
        0 => Note::C,
        1 => Note::Db,
        2 => Note::D,
        3 => Note::Eb,
        4 => Note::E,
        5 => Note::F,
        6 => Note::Gb,
        7 => Note::G,
        8 => Note::Ab,
        9 => Note::A,
        10 => Note::Bb,
        11 => Note::B,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;
    use crate::note::{Note, NoteParseError};
    use crate::note::Note::*;
    use rstest::rstest;

    #[test]
    fn semitones_up() {
        assert_eq!(C.semitones_up(1), Db);
        assert_eq!(C.semitones_up(2), D);
        assert_eq!(Db.semitones_up(1), D);
        assert_eq!(F.semitones_up(3), Ab);
        assert_eq!(C.semitones_up(0), C);
        assert_eq!(C.semitones_up(12), C);
        assert_eq!(C.semitones_up(24), C);
    }

    #[test]
    fn semitones() {
        assert_eq!(C.semitones(-1), B);
        assert_eq!(C.semitones(1), Db);
        assert_eq!(C.semitones(0), C);
        assert_eq!(C.semitones(12), C);
    }

    #[rstest]
    #[case("C",  C)]
    #[case("C#", Db)]
    #[case("Db", Db)]
    #[case("D",  D)]
    #[case("D#", Eb)]
    #[case("E",  E)]
    #[case("F",  F)]
    #[case("F#", Gb)]
    #[case("Gb", Gb)]
    #[case("G",  G)]
    #[case("G#", Ab)]
    #[case("Ab", Ab)]
    #[case("A",  A)]
    #[case("A#", Bb)]
    #[case("Bb", Bb)]
    #[case("B",  B)]
    fn parse_note(#[case] input: &str, #[case] expected: Note) -> Result<(), NoteParseError> {
        assert_eq!(String::from(input).to_uppercase().parse::<Note>()?, expected);
        assert_eq!(String::from(input).to_lowercase().parse::<Note>()?, expected);

        Ok(())
    }

    #[test]
    fn parse_error() {
        assert_eq!(
            "h".parse::<Note>().unwrap_err().to_string(),
            "Invalid note H"
        )
    }

    #[rstest]
    #[case(C, Interval::MajorThird,   E)]
    #[case(C, Interval::PerfectFifth, G)]
    #[case(G, Interval::MajorThird,   B)]
    #[case(B, Interval::MinorSecond,  C)]
    #[case(C, Interval::PerfectUnison, C)]
    fn note_add_interval(#[case] note: Note, #[case] interval: Interval, #[case] expected: Note) {
        assert_eq!(note + interval, expected);
    }

    #[rstest]
    #[case(E, C, Interval::MajorThird)]
    #[case(G, C, Interval::PerfectFifth)]
    #[case(C, B, Interval::MinorSecond)]
    #[case(C, C, Interval::PerfectUnison)]
    #[case(Bb, C, Interval::MinorSeventh)]
    fn note_sub_note(#[case] high: Note, #[case] low: Note, #[case] expected: Interval) {
        assert_eq!(high - low, expected);
    }
}
