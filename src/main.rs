mod note;
mod scales;

use note::Note;

fn main() {
    println!("{:?}", scales::major(&Note::D));
    println!("{:?}", scales::minor(&Note::D));
}
