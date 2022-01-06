#[derive(Clone, Debug, PartialEq)]
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

pub trait Interval {
    fn half_step_up(&self) -> &Note;
}

impl Interval for Note {
    fn half_step_up(&self) -> &Note {
        match self {
            Note::C => &Note::Db,
            Note::Db => &Note::D,
            Note::D => &Note::Eb,
            Note::Eb => &Note::E,
            Note::E => &Note::F,
            Note::F => &Note::Gb,
            Note::Gb => &Note::G,
            Note::G => &Note::Ab,
            Note::Ab => &Note::A,
            Note::A => &Note::Bb,
            Note::Bb => &Note::B,
            Note::B => &Note::C,
        }
    }
}