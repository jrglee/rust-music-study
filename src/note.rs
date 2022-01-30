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
        let actual_semitones = semitones % 12;

        for _ in 0..actual_semitones {
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

    pub fn minor_third(&self) -> Note {
        self.semitone_up(3)
    }

    pub fn major_third(&self) -> Note {
        self.semitone_up(4)
    }

    pub fn perfect_fourth(&self) -> Note {
        self.semitone_up(5)
    }

    pub fn diminished_fifth(&self) -> Note {
        self.semitone_up(6)
    }

    pub fn perfect_fifth(&self) -> Note {
        self.semitone_up(7)
    }

    pub fn minor_sixth(&self) -> Note {
        self.semitone_up(8)
    }

    pub fn major_sixth(&self) -> Note {
        self.semitone_up(9)
    }

    pub fn minor_seventh(&self) -> Note {
        self.semitone_up(10)
    }

    pub fn major_seventh(&self) -> Note {
        self.semitone_up(11)
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
    use crate::note::Note::{Ab, Bb, Db, Eb, Gb, A, B, C, D, E, F, G};

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

    #[test]
    fn get_minor_third() {
        assert_eq!(C.minor_third(), Eb);
    }

    #[test]
    fn get_major_third() {
        assert_eq!(C.major_third(), E);
    }

    #[test]
    fn get_perfect_fourth() {
        assert_eq!(C.perfect_fourth(), F);
    }

    #[test]
    fn get_diminished_fifth() {
        assert_eq!(C.diminished_fifth(), Gb);
    }

    #[test]
    fn get_perfect_fifth() {
        assert_eq!(C.perfect_fifth(), G);
    }

    #[test]
    fn get_minor_sixth() {
        assert_eq!(C.minor_sixth(), Ab);
    }

    #[test]
    fn get_major_sixth() {
        assert_eq!(C.major_sixth(), A);
    }

    #[test]
    fn get_minor_seventh() {
        assert_eq!(C.minor_seventh(), Bb);
    }

    #[test]
    fn get_major_seventh() {
        assert_eq!(C.major_seventh(), B);
    }
}
