use clap::{arg, Command, ValueEnum};
use inquire::Select;

use musicionist::interval::Interval;
use musicionist::note::Note;
use musicionist::scales;

#[derive(Clone, clap::ValueEnum)]
enum ScaleName {
    // Diatonic modes
    #[value(alias = "ionian")]
    Major,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    #[value(alias = "aeolian")]
    Minor,
    Locrian,

    // Harmonic minor modes
    HarmonicMinor,
    LocrianMaj6,
    IonianAug5,
    DorianLydian,
    PhrygianDominant,
    LydianAug2,
    SuperLocrian,
}

impl ScaleName {
    fn to_intervals(&self) -> Vec<Interval> {
        match self {
            ScaleName::Major => scales::diatonic::Mode::Ionian.intervals().to_vec(),
            ScaleName::Dorian => scales::diatonic::Mode::Dorian.intervals().to_vec(),
            ScaleName::Phrygian => scales::diatonic::Mode::Phrygian.intervals().to_vec(),
            ScaleName::Lydian => scales::diatonic::Mode::Lydian.intervals().to_vec(),
            ScaleName::Mixolydian => scales::diatonic::Mode::Mixolydian.intervals().to_vec(),
            ScaleName::Minor => scales::diatonic::Mode::Aeolian.intervals().to_vec(),
            ScaleName::Locrian => scales::diatonic::Mode::Locrian.intervals().to_vec(),

            ScaleName::HarmonicMinor => scales::harmonic_minor::Mode::HarmonicMinor.intervals().to_vec(),
            ScaleName::LocrianMaj6 => scales::harmonic_minor::Mode::LocrianMaj6.intervals().to_vec(),
            ScaleName::IonianAug5 => scales::harmonic_minor::Mode::IonianAug5.intervals().to_vec(),
            ScaleName::DorianLydian => scales::harmonic_minor::Mode::DorianLydian.intervals().to_vec(),
            ScaleName::PhrygianDominant => scales::harmonic_minor::Mode::PhrygianDominant.intervals().to_vec(),
            ScaleName::LydianAug2 => scales::harmonic_minor::Mode::LydianAug2.intervals().to_vec(),
            ScaleName::SuperLocrian => scales::harmonic_minor::Mode::SuperLocrian.intervals().to_vec(),
        }
    }
}

fn cli() -> Command {
    Command::new("musicionist")
        .about("A command line music theory tool")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("scale")
                .about("Generate a scale")
                .arg(arg!(<KEY> "the first note of the scale").value_parser(parse_note))
                .arg(
                    arg!([NAME] "the name of the scale, like major or minor")
                        .value_parser(clap::value_parser!(ScaleName)),
                ),
        )
        .subcommand(Command::new("list").about("List all available scale names"))
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

fn main() {
    match cli().get_matches().subcommand() {
        Some(("scale", m)) => {
            let key = m.get_one::<Note>("KEY").expect("required");
            let scale = match m.get_one::<ScaleName>("NAME") {
                Some(s) => s.clone(),
                None => {
                    let options: Vec<String> = ScaleName::value_variants()
                        .iter()
                        .filter_map(|v| v.to_possible_value().map(|pv| pv.get_name().to_string()))
                        .collect();
                    let selected = Select::new("Select a scale:", options).prompt().unwrap();
                    ScaleName::from_str(&selected, true).unwrap()
                }
            };
            println!("{:?}", scales::generate_scale(*key, &scale.to_intervals()));
        }
        Some(("list", _)) => {
            for v in ScaleName::value_variants() {
                if let Some(pv) = v.to_possible_value() {
                    println!("{}", pv.get_name());
                }
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use musicionist::note::Note;

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
