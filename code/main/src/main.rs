use std::fs::File;

use gif::Frame;
use sorting::{
    bingo_sort, bogosort, insertion_sort,
    util::{generate_mixed_vector, generate_random_vector},
};

fn main() {
    let mut vec = generate_mixed_vector(250);
    let mut frames: Vec<Frame> = Vec::new();
    bingo_sort::bingo_sort(&mut vec, &mut frames);

    let mut gif = File::create("sample.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut gif, 250, 250, &[]).unwrap();

    let mut counter = 0;
    for frame in frames {
        encoder.write_frame(&frame).unwrap();
        println!("Frame {} was written.", counter);
        counter += 1;
    }
}
