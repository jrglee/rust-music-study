use clap::{arg, Command};

use music_study::interval::Interval;
use music_study::note::Note;
use music_study::scales;

fn cli() -> Command {
    Command::new("musicionist")
        .about("A command line music theory tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("scale")
                .about("Generate a scale")
                .arg(arg!(<KEY> "the first note of the scale").value_parser(parse_note))
                .arg(arg!(<NAME> "the name of the scale, like major or minor").value_parser(parse_scale))
                .arg_required_else_help(true),
        )
}

fn parse_note(s: &str) -> Result<Note, String> {
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
        v => Err(format!("Invalid note {}", v)),
    }
}

fn parse_scale(s: &str) -> Result<Vec<Interval>, String> {
    match s.to_lowercase().as_str() {
        "major" | "ionian" => Ok(scales::diatonic::Mode::Ionian.intervals().to_vec()),
        "dorian" => Ok(scales::diatonic::Mode::Dorian.intervals().to_vec()),
        "phrygian" => Ok(scales::diatonic::Mode::Phrygian.intervals().to_vec()),
        "lydian" => Ok(scales::diatonic::Mode::Lydian.intervals().to_vec()),
        "mixolydian" => Ok(scales::diatonic::Mode::Mixolydian.intervals().to_vec()),
        "minor" | "aeolian" => Ok(scales::diatonic::Mode::Aeolian.intervals().to_vec()),
        "locrian" => Ok(scales::diatonic::Mode::Locrian.intervals().to_vec()),

        "harmonic minor" | "harmonic-minor" => Ok(scales::harmonic_minor::Mode::HarmonicMinor.intervals().to_vec()),
        "locrian maj6" | "locrian-maj6" | "locrian major6" | "locrian-major6" => {
            Ok(scales::harmonic_minor::Mode::LocrianMaj6.intervals().to_vec())
        }
        "ionian #5" | "ionian-#5" | "ionian aug5" | "ionian-aug5" => {
            Ok(scales::harmonic_minor::Mode::IonianAug5.intervals().to_vec())
        }
        "dorian lydian" | "dorian-lydian" | "dorian #4" | "dorian-#4" => {
            Ok(scales::harmonic_minor::Mode::DorianLydian.intervals().to_vec())
        }
        "phrygian dominant" | "phrygian-dominant" | "phrygian maj3" | "phrygian-maj3" => {
            Ok(scales::harmonic_minor::Mode::PhrygianDominant.intervals().to_vec())
        }
        "lydian #2" | "lydian-#2" | "lydian aug2" | "lydian-aug2" => {
            Ok(scales::harmonic_minor::Mode::LydianAug2.intervals().to_vec())
        }
        "superlocrian" | "super locrian" | "super-locrian" => {
            Ok(scales::harmonic_minor::Mode::SuperLocrian.intervals().to_vec())
        }
        other => Err(format!("Unrecognizable scale {}", other)),
    }
}

fn main() {
    match cli().get_matches().subcommand() {
        Some(("scale", sub_matches)) => {
            let key = sub_matches.get_one::<Note>("KEY").expect("required");
            let intervals = sub_matches.get_one::<Vec<Interval>>("NAME").expect("required");

            println!("{:?}", scales::generate_scale(*key, intervals));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use music_study::note::Note;

    use super::parse_note;

    macro_rules! parse_note_test {
        ($name:ident, $call:expr, $expected:expr) => {
            paste! {
                #[test]
                fn [<parse_note_ $name>]() {
                    assert_eq!(parse_note(&String::from($call).to_uppercase()).unwrap(), $expected);
                    assert_eq!(parse_note(&String::from($call).to_lowercase()).unwrap(), $expected);
                }
            }
        };
    }

    parse_note_test!(c, "C", Note::C);
    parse_note_test!(c_sharp, "C#", Note::Db);
    parse_note_test!(d_flat, "Db", Note::Db);
    parse_note_test!(d, "D", Note::D);
    parse_note_test!(d_sharp, "D#", Note::Eb);
    parse_note_test!(e, "E", Note::E);
    parse_note_test!(f, "F", Note::F);
    parse_note_test!(f_sharp, "F#", Note::Gb);
    parse_note_test!(g_flat, "Gb", Note::Gb);
    parse_note_test!(g, "G", Note::G);
    parse_note_test!(g_sharp, "G#", Note::Ab);
    parse_note_test!(a_flat, "Ab", Note::Ab);
    parse_note_test!(a, "A", Note::A);
    parse_note_test!(a_sharp, "A#", Note::Bb);
    parse_note_test!(b_flat, "Bb", Note::Bb);
    parse_note_test!(b, "B", Note::B);

    #[test]
    fn parse_error() {
        assert_eq!(
            parse_note(&String::from("h").to_uppercase()).err(),
            Some(String::from("Invalid note H"))
        )
    }
}
