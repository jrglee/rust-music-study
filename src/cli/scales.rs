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

#[derive(Debug, thiserror::Error)]
#[error("Invalid scale name: {0}")]
pub struct ScaleNameParseError(String);

impl std::str::FromStr for ScaleName {
    type Err = ScaleNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <ScaleName as clap::ValueEnum>::from_str(s, true)
            .map_err(ScaleNameParseError)
    }
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

pub fn handle_interactive() -> anyhow::Result<()> {
    let notes = vec!["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];
    let key: Note = Select::new("Key:", notes).prompt()?.parse()?;

    let options: Vec<String> = ScaleName::value_variants()
        .iter()
        .filter_map(|v| v.to_possible_value().map(|pv| pv.get_name().to_string()))
        .collect();
    let scale: ScaleName = Select::new("Scale:", options).prompt()?.parse()?;

    println!("{:?}", scales::generate_scale(key, &scale.to_intervals()));
    Ok(())
}

pub fn handle(m: &ArgMatches) -> anyhow::Result<()> {
    let key = match m.get_one::<Note>("KEY") {
        Some(k) => *k,
        None => {
            let notes = vec!["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];
            Select::new("Key:", notes).prompt()?.parse()?
        }
    };
    let scale = match m.get_one::<ScaleName>("NAME") {
        Some(s) => s.clone(),
        None => {
            let options: Vec<String> = ScaleName::value_variants()
                .iter()
                .filter_map(|v| v.to_possible_value().map(|pv| pv.get_name().to_string()))
                .collect();
            Select::new("Select a scale:", options).prompt()?.parse()?
        }
    };
    println!("{:?}", scales::generate_scale(key, &scale.to_intervals()));
    Ok(())
}

pub fn scale_subcommand() -> Command {
    Command::new("scale")
        .about("Generate a scale")
        .arg(arg!([KEY] "the first note of the scale").value_parser(clap::value_parser!(Note)))
        .arg(
            arg!([NAME] "the name of the scale, like major or minor")
                .value_parser(clap::value_parser!(ScaleName)),
        )
}
