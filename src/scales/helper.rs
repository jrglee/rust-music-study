use crate::interval::Interval;
use crate::scales::Degree;

pub fn interval_for(intervals: &[usize], offset: usize, degree: Degree) -> Option<Interval> {
    let mut shifted = vec![];
    shifted.extend_from_slice(intervals);
    shifted.rotate_left(offset);

    let semitones = shifted.iter().take(degree.as_number() - 1).sum();

    degree.interval(semitones)
}
