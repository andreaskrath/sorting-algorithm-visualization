use std::fs::File;

use gif::Frame;
use sorting::{bingo_sort, bogosort, util::generate_mixed_vector};

fn main() {
    let mut vec = generate_mixed_vector(5);
    let mut frames: Vec<Frame> = Vec::new();
    bogosort::bogosort(&mut vec, &mut frames);

    let mut gif = File::create("sample.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut gif, 5, 5, &[]).unwrap();

    for (counter, frame) in frames.iter().enumerate() {
        encoder.write_frame(frame).unwrap();
        println!("Frame {} was written.", counter + 1);
    }
}
