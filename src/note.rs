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
        _ => panic!("Unreachable case"),
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note::*;

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
}
