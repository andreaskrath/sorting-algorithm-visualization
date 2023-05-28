use std::fs::File;

use gif::Frame;
use sorting::{bingo_sort, bogosort, insertion_sort, util::generate_mixed_vector};

fn main() {
    let mut vec = generate_mixed_vector(100);
    let mut frames: Vec<Frame> = Vec::new();
    insertion_sort::insertion_sort(&mut vec, &mut frames);

    let mut gif = File::create("sample.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut gif, 100, 100, &[]).unwrap();

    for (counter, frame) in frames.iter().enumerate() {
        encoder.write_frame(frame).unwrap();
        println!("Frame {} was written.", counter + 1);
    }
}
