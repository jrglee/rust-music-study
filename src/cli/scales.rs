use clap::arg;
use clap::{ArgMatches, Command, ValueEnum};
use inquire::Select;
use crate::interval::Interval;
use crate::note::Note;
use crate::scales;

#[derive(Clone, clap::ValueEnum)]
pub enum ScaleName {
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
    pub fn to_intervals(&self) -> Vec<Interval> {
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

pub fn handle(m: &ArgMatches) {
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

pub fn scale_subcommand() -> Command {
    Command::new("scale")
        .about("Generate a scale")
        .arg(arg!(<KEY> "the first note of the scale").value_parser(clap::value_parser!(Note)))
        .arg(
            arg!([NAME] "the name of the scale, like major or minor")
                .value_parser(clap::value_parser!(ScaleName)),
        )
}
