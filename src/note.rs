use std::ops::{Shl, Shr};
use std::str::FromStr;

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

    pub fn transpose(&self, semitones: isize) -> Note {
        semitones_to_c_isize(self.semitones_from_c() as isize + semitones.rem_euclid(12))
    }
}

impl Shl<isize> for Note {
    type Output = Note;
    fn shl(self, rhs: isize) -> Note {
        self.transpose(rhs)
    }
}

impl Shr<isize> for Note {
    type Output = Note;
    fn shr(self, rhs: isize) -> Note {
        self.transpose(-rhs)
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

fn semitones_to_c_isize(semitones: isize) -> Note {
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
    use crate::note::Note::*;
    use crate::note::{Note, NoteParseError};
    use rstest::rstest;

    #[test]
    fn transpose_positive() {
        assert_eq!(C.transpose(1), Db);
        assert_eq!(C.transpose(2), D);
        assert_eq!(Db.transpose(1), D);
        assert_eq!(F.transpose(3), Ab);
        assert_eq!(C.transpose(0), C);
        assert_eq!(C.transpose(12), C);
        assert_eq!(C.transpose(24), C);
    }

    #[test]
    fn transpose_negative() {
        assert_eq!(C.transpose(-1), B);
        assert_eq!(C.transpose(1), Db);
        assert_eq!(C.transpose(0), C);
        assert_eq!(C.transpose(12), C);
        assert_eq!(E.transpose(-4), C);
    }

    #[rstest]
    #[case("C", C)]
    #[case("C#", Db)]
    #[case("Db", Db)]
    #[case("D", D)]
    #[case("D#", Eb)]
    #[case("E", E)]
    #[case("F", F)]
    #[case("F#", Gb)]
    #[case("Gb", Gb)]
    #[case("G", G)]
    #[case("G#", Ab)]
    #[case("Ab", Ab)]
    #[case("A", A)]
    #[case("A#", Bb)]
    #[case("Bb", Bb)]
    #[case("B", B)]
    fn parse_note(#[case] input: &str, #[case] expected: Note) -> Result<(), NoteParseError> {
        assert_eq!(String::from(input).to_uppercase().parse::<Note>()?, expected);
        assert_eq!(String::from(input).to_lowercase().parse::<Note>()?, expected);

        Ok(())
    }

    #[test]
    fn parse_error() {
        assert_eq!("h".parse::<Note>().unwrap_err().to_string(), "Invalid note H")
    }

    #[test]
    fn shl_semitones() {
        assert_eq!(C << 4, E);
        assert_eq!(C << 7, G);
        assert_eq!(C << 0, C);
        assert_eq!(C << 12, C);
    }

    #[test]
    fn shr_semitones() {
        assert_eq!(E >> 4, C);
        assert_eq!(C >> 1, B);
        assert_eq!(C >> 0, C);
        assert_eq!(G >> 7, C);
    }
}
