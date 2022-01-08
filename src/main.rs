use music_study::note::Note;
use music_study::scales;

fn main() {
    println!("{:?}", scales::major(&Note::D));
    println!("{:?}", scales::minor(&Note::D));
}
