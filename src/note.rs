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
    fn half_step_up(&self) -> Note {
        match self {
            Note::C => Note::Db,
            Note::Db => Note::D,
            Note::D => Note::Eb,
            Note::Eb => Note::E,
            Note::E => Note::F,
            Note::F => Note::Gb,
            Note::Gb => Note::G,
            Note::G => Note::Ab,
            Note::Ab => Note::A,
            Note::A => Note::Bb,
            Note::Bb => Note::B,
            Note::B => Note::C,
        }
    }

    pub fn semitone_up(&self, semitones: usize) -> Note {
        let mut note = self.to_owned();

        for _ in 0..semitones {
            note = note.half_step_up()
        }

        note
    }

    pub fn minor_second(&self) -> Note {
        self.semitone_up(1)
    }

    pub fn major_second(&self) -> Note {
        self.semitone_up(2)
    }
}

impl Iterator for Note {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        *self = self.half_step_up();
        Some(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::note::Note::{C, D, Db};

    #[test]
    fn get_half_step_up() {
        assert_eq!(C.half_step_up(), Db);
        assert_eq!(C.half_step_up().half_step_up(), D);
    }

    #[test]
    fn iterate_octave() {
        assert_eq!(C.skip(11).next(), Some(C));
    }

    #[test]
    fn get_minor_second() {
        assert_eq!(C.minor_second(), Db);
    }

    #[test]
    fn get_major_second() {
        assert_eq!(C.major_second(), D);
    }
}
