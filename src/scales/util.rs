use crate::interval::Interval;
use crate::note::Note;
use crate::scales::diatonic::Mode;

pub fn generate_scale(key: Note, intervals: &[Interval]) -> Vec<Note> {
    let mut scale: Vec<Note> = intervals.iter().map(|int| int.apply_to_note(key)).collect();
    scale.push(key);
    scale
}

pub fn major(key: Note) -> Vec<Note> {
    diatonic_mode(key, Mode::Ionian)
}

pub fn minor(key: Note) -> Vec<Note> {
    diatonic_mode(key, Mode::Aeolian)
}

pub fn diatonic_mode(key: Note, mode: Mode) -> Vec<Note> {
    generate_scale(key, &mode.intervals())
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use Mode::*;
    use Note::*;

    use super::*;

    macro_rules! scale_test {
        ($name:ident, $call:expr, $expected:expr) => {
            paste! {
                #[test]
                fn [<$name _scale>]() {
                    assert_eq!($call, $expected);
                }
            }
        };
    }

    scale_test!(c_major, major(C), [C, D, E, F, G, A, B, C]);
    scale_test!(g_major, major(G), [G, A, B, C, D, E, Gb, G]);
    scale_test!(a_minor, minor(A), [A, B, C, D, E, F, G, A]);
    scale_test!(e_minor, minor(E), [E, Gb, G, A, B, C, D, E]);

    scale_test!(c_ionian, diatonic_mode(C, Ionian), [C, D, E, F, G, A, B, C]);
    scale_test!(d_dorian, diatonic_mode(D, Dorian), [D, E, F, G, A, B, C, D]);
    scale_test!(e_phrygian, diatonic_mode(E, Phrygian), [E, F, G, A, B, C, D, E]);
    scale_test!(f_lydian, diatonic_mode(F, Lydian), [F, G, A, B, C, D, E, F]);
    scale_test!(g_mixolydian, diatonic_mode(G, Mixolydian), [G, A, B, C, D, E, F, G]);
    scale_test!(a_aeolian, diatonic_mode(A, Aeolian), [A, B, C, D, E, F, G, A]);
    scale_test!(b_locrian, diatonic_mode(B, Locrian), [B, C, D, E, F, G, A, B]);
}
