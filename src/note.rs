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
    pub fn semitones_from_c(&self) -> usize {
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
        let semitones = (self.semitones_from_c() as isize + semitones).rem_euclid(12);
        match semitones {
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

#[cfg(test)]
mod tests {
    use crate::note::{Note, NoteParseError};
    use rstest::rstest;

    #[rstest]
    #[case("C", Note::C)]
    #[case("c", Note::C)]
    #[case("C#", Note::Db)]
    #[case("c#", Note::Db)]
    #[case("Db", Note::Db)]
    #[case("db", Note::Db)]
    #[case("D", Note::D)]
    #[case("d", Note::D)]
    #[case("D#", Note::Eb)]
    #[case("d#", Note::Eb)]
    #[case("EB", Note::Eb)]
    #[case("eb", Note::Eb)]
    #[case("E", Note::E)]
    #[case("e", Note::E)]
    #[case("F", Note::F)]
    #[case("f", Note::F)]
    #[case("F#", Note::Gb)]
    #[case("f#", Note::Gb)]
    #[case("GB", Note::Gb)]
    #[case("gb", Note::Gb)]
    #[case("G", Note::G)]
    #[case("g", Note::G)]
    #[case("G#", Note::Ab)]
    #[case("g#", Note::Ab)]
    #[case("AB", Note::Ab)]
    #[case("ab", Note::Ab)]
    #[case("A", Note::A)]
    #[case("a", Note::A)]
    #[case("A#", Note::Bb)]
    #[case("a#", Note::Bb)]
    #[case("BB", Note::Bb)]
    #[case("bb", Note::Bb)]
    #[case("B", Note::B)]
    #[case("b", Note::B)]
    fn parse_note(#[case] input: &str, #[case] expected: Note) -> Result<(), NoteParseError> {
        assert_eq!(input.parse::<Note>()?, expected);
        Ok(())
    }

    #[rstest]
    #[case("h")]
    #[case("X")]
    #[case("Cx")]
    #[case("D##")]
    #[case("")]
    fn parse_error(#[case] input: &str) {
        assert!(input.parse::<Note>().is_err());
    }

    #[rstest]
    #[case(Note::C, 7, Note::G)]
    #[case(Note::Db, 1, Note::D)]
    #[case(Note::F, 5, Note::Bb)]
    fn shl_semitones(#[case] note: Note, #[case] semitones: isize, #[case] expected: Note) {
        assert_eq!(note << semitones, expected);
    }

    #[rstest]
    #[case(Note::E, 4, Note::C)]
    #[case(Note::G, 7, Note::C)]
    #[case(Note::C, 1, Note::B)]
    #[case(Note::C, 0, Note::C)]
    #[case(Note::C, 12, Note::C)]
    #[case(Note::Bb, 10, Note::C)]
    fn shr_semitones(#[case] note: Note, #[case] semitones: isize, #[case] expected: Note) {
        assert_eq!(note >> semitones, expected);
    }
}
