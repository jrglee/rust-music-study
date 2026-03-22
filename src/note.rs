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
    fn semitones_from_c(&self) -> usize {
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
    use crate::note::Note;
    use crate::note::Note::*;
    use paste::paste;

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

    macro_rules! parse_note_test {
        ($name:ident, $call:expr, $expected:expr) => {
            paste! {
                #[test]
                fn [<parse_note_ $name>]() {
                    assert_eq!(&String::from($call).to_uppercase().parse::<Note>().unwrap(), &$expected);
                    assert_eq!(&String::from($call).to_lowercase().parse::<Note>().unwrap(), &$expected);
                }
            }
        };
    }

    parse_note_test!(c, "C", C);
    parse_note_test!(c_sharp, "C#", Db);
    parse_note_test!(d_flat, "Db", Db);
    parse_note_test!(d, "D", D);
    parse_note_test!(d_sharp, "D#", Eb);
    parse_note_test!(e, "E", E);
    parse_note_test!(f, "F", F);
    parse_note_test!(f_sharp, "F#", Gb);
    parse_note_test!(g_flat, "Gb", Gb);
    parse_note_test!(g, "G", G);
    parse_note_test!(g_sharp, "G#", Ab);
    parse_note_test!(a_flat, "Ab", Ab);
    parse_note_test!(a, "A", A);
    parse_note_test!(a_sharp, "A#", Bb);
    parse_note_test!(b_flat, "Bb", Bb);
    parse_note_test!(b, "B", B);

    #[test]
    fn parse_error() {
        assert_eq!(
            "h".parse::<Note>().unwrap_err().to_string(),
            "Invalid note H"
        )
    }
}
